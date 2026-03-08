#![allow(dead_code, reason = "still in early experimentation")]

use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, TokenStream, TokenTree};

// pub(crate) so $crate:: paths in macros can reach them.

pub(crate) fn tt_punct_alone(out: &mut Vec<TokenTree>, chr: char) {
    out.push(TokenTree::Punct(Punct::new(chr, Spacing::Alone)));
}
pub(crate) fn tt_punct_joint(out: &mut Vec<TokenTree>, chr: char) {
    out.push(TokenTree::Punct(Punct::new(chr, Spacing::Joint)));
}
pub(crate) fn tt_ident(out: &mut Vec<TokenTree>, ident: &'static str) {
    out.push(TokenTree::Ident(Ident::new_static(ident)));
}
pub(crate) fn tt_group_empty(out: &mut Vec<TokenTree>, delimiter: Delimiter) {
    out.push(TokenTree::Group(Group::new(delimiter, TokenStream::new())));
}
pub(crate) fn tt_group(out: &mut Vec<TokenTree>, delimiter: Delimiter, from: usize) {
    let group = TokenTree::Group(Group::new(
        delimiter,
        TokenStream::from_iter(out.drain(from..)),
    ));
    out.push(group);
}

#[rustfmt::skip]
macro_rules! append_tok {
    ($ident:ident $d:tt) => {
       $crate::tt_ident($d, stringify!($ident))
    };
    ({} $d: tt) => {
        $crate::tt_group_empty($d, proc_macro2::Delimiter::Brace);
    };
    (() $d: tt) => {
        $crate::tt_group_empty($d, proc_macro2::Delimiter::Parenthesis);
    };
    ([] $d:tt) => {
        $crate::tt_group_empty($d, proc_macro2::Delimiter::Bracket);
    };
    ({$($tt:tt)*} $d: tt) => {{
        let at = $d.len(); $(append_tok!($tt $d);)* $crate::tt_group($d, proc_macro2::Delimiter::Brace, at);
    }};
    (($($tt:tt)*) $d: tt) => {{
        let at = $d.len(); $(append_tok!($tt $d);)* $crate::tt_group($d, proc_macro2::Delimiter::Parenthesis, at);
    }};
    ([[$($tt:tt)*]] $d:tt) => {{
        let at = $d.len(); $(append_tok!($tt $d);)* $crate::tt_group($d, proc_macro2::Delimiter::Bracket, at);
    }};
    (_ $d:tt) => { $crate::tt_ident($d, "_") };
    ([$ident:ident] $d:tt) => {
        $d.push($($tt)*)
    };
    ([?($($cond:tt)*) $($body:tt)*] $d:tt) => {
        if $($cond)* { $(append_tok!($body $d);)* }
    };
    ([@$($tt:tt)*] $d:tt) => {
        $d.push(($($tt)*).into())
    };
    ([try $($tt:tt)*] $d:tt) => {
        if let Err(err) = $($tt)* { return Err(err); }
    };
    ([for ($($iter:tt)*) {$($body:tt)*}] $d:tt) => {
        for $($iter)* { $(append_tok!($body $d);)* }
    };
    ([#$($tt:tt)*] $d:tt) => {
        $d.push(proc_macro2::TokenTree::from($($tt)*.clone()))
    };
    ([~$($tt:tt)*] $d:tt) => {
        $d.extend_from_slice($($tt)*)
    };
    ([$($rust:tt)*] $d:tt) => {{
         $($rust)*
    }};
    (~ $d:tt) => { $crate::tt_punct_joint($d, '\'') };
    (: $d:tt) => { $crate::tt_punct_alone($d, ':') };
    (# $d:tt) => { $crate::tt_punct_joint($d, '#') };
    (< $d:tt) => { $crate::tt_punct_alone($d, '<') };
    (% $d:tt) => { $crate::tt_punct_joint($d, ':') };
    (:: $d:tt) => { $crate::tt_punct_joint($d, ':'); $crate::tt_punct_alone($d, ':'); };
    (.. $d:tt) => { $crate::tt_punct_joint($d, '.'); $crate::tt_punct_alone($d, '.'); };
    (-> $d:tt) => { $crate::tt_punct_joint($d, '-'); $crate::tt_punct_alone($d, '>'); };
    (=> $d:tt) => { $crate::tt_punct_joint($d, '='); $crate::tt_punct_alone($d, '>'); };
    (== $d:tt) => { $crate::tt_punct_joint($d, '='); $crate::tt_punct_alone($d, '='); };
    (> $d:tt) => { $crate::tt_punct_alone($d, '>') };
    (! $d:tt) => { $crate::tt_punct_alone($d, '!') };
    (| $d:tt) => { $crate::tt_punct_alone($d, '|') };
    (+ $d:tt) => { $crate::tt_punct_alone($d, '+') };
    (. $d:tt) => { $crate::tt_punct_alone($d, '.') };
    (; $d:tt) => { $crate::tt_punct_alone($d, ';') };
    (& $d:tt) => { $crate::tt_punct_alone($d, '&') };
    (= $d:tt) => { $crate::tt_punct_alone($d, '=') };
    (, $d:tt) => { $crate::tt_punct_alone($d, ',') };
    (* $d:tt) => { $crate::tt_punct_alone($d, '*') };
    ($literal:literal $d:tt) => {
        $d.push(proc_macro2::TokenTree::Literal(proc_macro2::Literal::string($literal)))
    };
}

macro_rules! splat { ($d:tt; $($tt:tt)*) => { { $(append_tok!($tt $d);)* } } }
macro_rules! sfn { ($d:tt; $($tt:tt)*) => { & (|$d: &mut Vec<proc_macro2::TokenTree>| { $(append_tok!($tt $d);)* }) } }

mod bench;
mod cli;
mod datagen;
mod features;
mod library;
mod report;
mod schema;
mod social;
mod task;
mod token;

use anyhow::Context;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::args().map_err(|e| anyhow::anyhow!("{}", e))?;

    if args.help {
        println!("{}", cli::Args::help());
        return Ok(());
    }

    match args.command {
        cli::Command::Run(run) => cmd_run(run),
        cli::Command::Report(report) => cmd_report(report),
        cli::Command::Generate(gen) => cmd_generate(gen),
        cli::Command::Verify(verify) => cmd_verify(verify),
    }
}

fn cmd_run(args: cli::RunArgs) -> anyhow::Result<()> {
    let libraries = cli::resolve_libraries(&args.libs, &args.jsony_path);
    let scenarios = cli::resolve_scenarios(&args.scenarios, &args.profiles, &args.incrementals);
    let _layout = cli::resolve_layout(&args.layout);

    if libraries.is_empty() {
        anyhow::bail!("No libraries selected");
    }
    if scenarios.is_empty() {
        anyhow::bail!("No scenarios selected");
    }

    let run_synthetic = matches!(args.task.as_str(), "synthetic" | "all");
    let run_social = matches!(args.task.as_str(), "social-only-json" | "all");
    let run_multi_format = matches!(args.task.as_str(), "social-with-binary" | "all");

    if !run_synthetic && !run_social && !run_multi_format {
        anyhow::bail!(
            "Unknown task: {}. Use: synthetic, social-only-json, social-with-binary, all",
            args.task
        );
    }

    let results_dir = &args.results_dir;

    if run_synthetic {
        let mut alloc = bumpalo::Bump::new();
        let mut types = schema::Types {
            bump: &mut alloc,
            db: Vec::new(),
        };

        types.insert_synthetic_seeds();

        let bench_task = task::LotsOfStructs::new(75, &mut types);

        let baseline_scenarios: Vec<_> = scenarios
            .iter()
            .filter(|s| !matches!(s, bench::Scenario::RuntimeBenchmark { profile: bench::BuildProfile::Debug }))
            .cloned()
            .collect();
        if !baseline_scenarios.is_empty() {
            let (versions, result) = bench_task
                .bench(
                    "baseline_sm",
                    library::Libary::Baseline,
                    &baseline_scenarios,
                    args.samples,
                )
                .context("baseline_sm")?;
            save_lib_result(results_dir, "synthetic", "baseline", &versions, &result)?;
        }

        let task_suffix = "sm";
        for lib in &libraries {
            let crate_name = format!("{}_{}", lib.crate_prefix(), task_suffix);
            let (versions, result) = bench_task
                .bench(&crate_name, lib.clone(), &scenarios, args.samples)
                .context(crate_name)?;
            save_lib_result(results_dir, "synthetic", lib.name(), &versions, &result)?;
        }
    }

    if run_social {
        let alloc = bumpalo::Bump::new();
        let mut types = schema::Types {
            bump: &alloc,
            db: Vec::new(),
        };

        let social_task = social::SocialMediaTask::new(&mut types);

        let baseline_scenarios: Vec<_> = scenarios
            .iter()
            .filter(|s| !matches!(s, bench::Scenario::RuntimeBenchmark { profile: bench::BuildProfile::Debug }))
            .cloned()
            .collect();
        if !baseline_scenarios.is_empty() {
            let (versions, result) = social_task
                .bench(
                    "baseline_social",
                    library::Libary::Baseline,
                    &baseline_scenarios,
                    args.samples,
                )
                .context("baseline_social")?;
            save_lib_result(
                results_dir,
                "social-only-json",
                "baseline",
                &versions,
                &result,
            )?;
        }

        for lib in &libraries {
            if !lib.supports_social() {
                continue;
            }
            let crate_name = format!("{}_social", lib.crate_prefix());
            let (versions, result) = social_task
                .bench(&crate_name, lib.clone(), &scenarios, args.samples)
                .context(crate_name)?;
            save_lib_result(
                results_dir,
                "social-only-json",
                lib.name(),
                &versions,
                &result,
            )?;
        }
    }

    if run_multi_format {
        let alloc = bumpalo::Bump::new();
        let mut types = schema::Types {
            bump: &alloc,
            db: Vec::new(),
        };

        let social_task = social::SocialMediaTask::new(&mut types);

        let baseline_scenarios: Vec<_> = scenarios
            .iter()
            .filter(|s| !matches!(s, bench::Scenario::RuntimeBenchmark { profile: bench::BuildProfile::Debug }))
            .cloned()
            .collect();
        if !baseline_scenarios.is_empty() {
            let (versions, result) = social_task
                .bench(
                    "baseline_multi_social",
                    library::Libary::Baseline,
                    &baseline_scenarios,
                    args.samples,
                )
                .context("baseline_multi_social")?;
            save_lib_result(
                results_dir,
                "social-with-binary",
                "baseline",
                &versions,
                &result,
            )?;
        }

        for lib in &libraries {
            if !lib.supports_multi_format() {
                continue;
            }
            let crate_name = format!("{}_multi_social", lib.crate_prefix());
            let (versions, result) = social_task
                .bench_multi(&crate_name, lib.clone(), &scenarios, args.samples)
                .context(crate_name)?;
            save_lib_result(
                results_dir,
                "social-with-binary",
                lib.name(),
                &versions,
                &result,
            )?;
        }
    }

    Ok(())
}

fn cmd_generate(args: cli::GenerateArgs) -> anyhow::Result<()> {
    let libraries = cli::resolve_libraries(&args.libs, &args.jsony_path);

    if libraries.is_empty() {
        anyhow::bail!("No libraries selected");
    }

    let gen_synthetic = matches!(args.task.as_str(), "synthetic" | "all");
    let gen_social = matches!(args.task.as_str(), "social-only-json" | "all");
    let gen_multi_format = matches!(args.task.as_str(), "social-with-binary" | "all");

    if !gen_synthetic && !gen_social && !gen_multi_format {
        anyhow::bail!(
            "Unknown task: {}. Use: synthetic, social-only-json, social-with-binary, all",
            args.task
        );
    }

    let base: &Path = match &args.output_dir {
        Some(dir) => {
            std::fs::create_dir_all(dir)?;
            Path::new(dir.as_str())
        }
        None => bench::base_dir(),
    };

    if gen_synthetic {
        let mut alloc = bumpalo::Bump::new();
        let mut types = schema::Types {
            bump: &mut alloc,
            db: Vec::new(),
        };
        types.insert_synthetic_seeds();
        let bench_task = task::LotsOfStructs::new(75, &mut types);

        for lib in &libraries {
            let crate_name = format!("{}_sm", lib.crate_prefix());
            let b = bench::Benchy::open_in(base, &crate_name, &lib.dependencies())?;
            let source = bench_task.codegen_models(lib);
            let formatted = report::format_model_source(&source);
            b.write_bytes("main.rs", &formatted)?;
            let crate_dir = base.join(&crate_name);
            run_cargo_fmt(&crate_dir)?;
            run_cargo_check(&crate_dir)?;
            println!("Generated {}", crate_dir.display());
        }
    }

    if gen_social {
        let alloc = bumpalo::Bump::new();
        let mut types = schema::Types {
            bump: &alloc,
            db: Vec::new(),
        };
        let social_task = social::SocialMediaTask::new(&mut types);

        for lib in &libraries {
            if !lib.supports_social() {
                continue;
            }
            let crate_name = format!("{}_social", lib.crate_prefix());
            let b = bench::Benchy::open_in(base, &crate_name, &lib.dependencies())?;
            let models = social_task.codegen_models(lib);
            let formatted = report::format_model_source(&models);
            b.write_bytes("models.rs", &formatted)?;
            b.write_bytes("compat.rs", &lib.compat_module_bytes())?;
            b.write_bytes("db.rs", include_bytes!("social/db_template.rs"))?;
            b.write_bytes("routes.rs", include_bytes!("social/routes_template.rs"))?;
            b.write_bytes("main.rs", include_bytes!("social/main_template.rs"))?;
            let crate_dir = base.join(&crate_name);
            run_cargo_fmt(&crate_dir)?;
            run_cargo_check(&crate_dir)?;
            println!("Generated {}", crate_dir.display());
        }
    }

    if gen_multi_format {
        let alloc = bumpalo::Bump::new();
        let mut types = schema::Types {
            bump: &alloc,
            db: Vec::new(),
        };
        let social_task = social::SocialMediaTask::new(&mut types);

        for lib in &libraries {
            if !lib.supports_multi_format() {
                continue;
            }
            let crate_name = format!("{}_multi_social", lib.crate_prefix());
            let b = bench::Benchy::open_in(base, &crate_name, &lib.dependencies_multi())?;
            let models = social_task.codegen_models_multi(lib);
            let formatted = report::format_model_source(&models);
            b.write_bytes("models.rs", &formatted)?;
            b.write_bytes("compat.rs", &lib.compat_module_multi_bytes())?;
            b.write_bytes("db.rs", include_bytes!("social/db_binary_template.rs"))?;
            b.write_bytes("routes.rs", include_bytes!("social/routes_template.rs"))?;
            b.write_bytes("main.rs", include_bytes!("social/main_template.rs"))?;
            let crate_dir = base.join(&crate_name);
            run_cargo_fmt(&crate_dir)?;
            run_cargo_check(&crate_dir)?;
            println!("Generated {}", crate_dir.display());
        }
    }

    Ok(())
}

fn run_cargo_fmt(crate_dir: &Path) -> anyhow::Result<()> {
    let status = std::process::Command::new("cargo")
        .arg("fmt")
        .current_dir(crate_dir)
        .status()
        .context("failed to run cargo fmt")?;
    if !status.success() {
        anyhow::bail!("cargo fmt failed in {}", crate_dir.display());
    }
    Ok(())
}

fn run_cargo_check(crate_dir: &Path) -> anyhow::Result<()> {
    let status = std::process::Command::new("cargo")
        .arg("check")
        .current_dir(crate_dir)
        .status()
        .context("failed to run cargo check")?;
    if !status.success() {
        anyhow::bail!("cargo check failed in {}", crate_dir.display());
    }
    Ok(())
}

fn cmd_report(args: cli::ReportArgs) -> anyhow::Result<()> {
    generate_full_report(
        &args.results_dir,
        &args.report_dir,
        &args.task,
        &args.ignore_libs,
    )
}

fn cmd_verify(args: cli::VerifyArgs) -> anyhow::Result<()> {
    let libraries = cli::resolve_libraries(&args.libs, &args.jsony_path);
    let alloc = bumpalo::Bump::new();
    let mut types = schema::Types {
        bump: &alloc,
        db: Vec::new(),
    };
    let social_task = social::SocialMediaTask::new(&mut types);
    social_task.verify(&libraries)
}

fn save_lib_result(
    results_dir: &str,
    task: &str,
    lib_name: &str,
    versions: &[(String, String)],
    benchmarks: &[(bench::Scenario, bench::Perf)],
) -> anyhow::Result<()> {
    let dir = format!("{results_dir}/{task}");
    std::fs::create_dir_all(&dir)?;
    let entry: (&str, &[(String, String)], &[(bench::Scenario, bench::Perf)]) =
        (lib_name, versions, benchmarks);
    let json = jsony::to_json(&entry);
    std::fs::write(format!("{dir}/{lib_name}.json"), &json)?;
    Ok(())
}

fn load_task_results(
    task_dir: &Path,
) -> anyhow::Result<
    Vec<(
        String,
        Vec<(String, String)>,
        Vec<(bench::Scenario, bench::Perf)>,
    )>,
> {
    let mut combined = Vec::new();
    let mut entries: Vec<_> = std::fs::read_dir(task_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
        .collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let content = std::fs::read_to_string(entry.path())?;
        let parsed: (
            String,
            Vec<(String, String)>,
            Vec<(bench::Scenario, bench::Perf)>,
        ) = jsony::from_json(&content)
            .with_context(|| format!("parsing {}", entry.path().display()))?;
        combined.push(parsed);
    }
    Ok(combined)
}

fn generate_full_report(
    results_dir: &str,
    report_dir: &str,
    task_filter: &str,
    ignore_libs: &[String],
) -> anyhow::Result<()> {
    std::fs::create_dir_all(report_dir)?;

    let run_all = task_filter == "all";
    let results_path = Path::new(results_dir);

    let mut task_dirs: Vec<_> = std::fs::read_dir(results_path)
        .with_context(|| format!("reading results directory: {results_dir}"))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    task_dirs.sort_by_key(|e| e.file_name());

    let mut processed_tasks: Vec<String> = Vec::new();

    for entry in &task_dirs {
        let task_name = entry.file_name().to_string_lossy().to_string();

        if !run_all && task_filter != task_name {
            continue;
        }

        let combined = load_task_results(&entry.path())?;
        if combined.is_empty() {
            eprintln!("No results found in {}", entry.path().display());
            continue;
        }

        let combined: Vec<_> = if ignore_libs.is_empty() {
            combined
        } else {
            combined
                .into_iter()
                .filter(|(lib, _, _)| !ignore_libs.iter().any(|ig| ig == lib))
                .collect()
        };

        let combined_json = jsony::to_json(&combined);
        std::fs::write(format!("{results_dir}/{task_name}.json"), &combined_json)?;

        // Re-parse to get borrowed &str types that format_markdown_results expects
        let data: Vec<(
            &str,
            Vec<(String, String)>,
            Vec<(bench::Scenario, bench::Perf)>,
        )> = jsony::from_json(&combined_json)?;
        let data: Vec<_> = if ignore_libs.is_empty() {
            data
        } else {
            data.into_iter()
                .filter(|(lib, _, _)| !ignore_libs.iter().any(|ig| ig == lib))
                .collect()
        };
        report::format_markdown_results(data, report_dir, &task_name)?;
        println!("Generated report/BENCH-{task_name}.md");

        processed_tasks.push(task_name);
    }

    let aggregate_graph_dir = Path::new("aggregate_graph");
    if aggregate_graph_dir.exists() {
        let abs_results = std::fs::canonicalize(results_dir)?;
        let abs_report = Path::new(report_dir).join("report");
        std::fs::create_dir_all(&abs_report)?;
        let abs_report = std::fs::canonicalize(&abs_report)?;

        for task_name in &processed_tasks {
            let input = abs_results.join(format!("{task_name}.json"));
            let output = abs_report.join(format!("{task_name}_aggregate.png"));
            let title = match task_name.as_str() {
                "synthetic" => "Synthetic Benchmark",
                "social-only-json" => "Social Model with JSON Only",
                "social-with-binary" => "Social Model with JSON + Binary",
                other => other,
            };
            let status = std::process::Command::new("uv")
                .args(["run", "main.py", "--input"])
                .arg(&input)
                .arg("--output")
                .arg(&output)
                .args(["--title", title])
                .current_dir(aggregate_graph_dir)
                .status();
            match status {
                Ok(s) if s.success() => println!("Generated {task_name}_aggregate.png"),
                Ok(s) => eprintln!("aggregate_graph failed for {task_name} (exit: {s})"),
                Err(e) => eprintln!("Failed to run aggregate_graph for {task_name}: {e}"),
            }
        }
    }

    let report_subdir = format!("{report_dir}/report");
    std::fs::create_dir_all(&report_subdir)?;
    let jsony_lib = library::Libary::Jsony { path: None };

    for task_name in &processed_tasks {
        let model_source = match task_name.as_str() {
            "synthetic" => {
                let mut alloc = bumpalo::Bump::new();
                let mut types = schema::Types {
                    bump: &mut alloc,
                    db: Vec::new(),
                };
                types.insert_synthetic_seeds();
                let bench_task = task::LotsOfStructs::new(75, &mut types);
                bench_task.codegen_models(&jsony_lib)
            }
            "social-only-json" => {
                let alloc = bumpalo::Bump::new();
                let mut types = schema::Types {
                    bump: &alloc,
                    db: Vec::new(),
                };
                let social_task = social::SocialMediaTask::new(&mut types);
                social_task.codegen_models(&jsony_lib)
            }
            "social-with-binary" => {
                let alloc = bumpalo::Bump::new();
                let mut types = schema::Types {
                    bump: &alloc,
                    db: Vec::new(),
                };
                let social_task = social::SocialMediaTask::new(&mut types);
                social_task.codegen_models_multi(&jsony_lib)
            }
            _ => continue,
        };
        let model_source = report::format_model_source(&model_source);
        std::fs::write(format!("{report_subdir}/{task_name}_model.rs"), &model_source)?;
        println!("Generated {task_name}_model.rs");
    }

    let task_refs: Vec<&str> = processed_tasks.iter().map(|s| s.as_str()).collect();
    report::generate_readme(report_dir, &task_refs)?;
    println!("Generated README.md");

    Ok(())
}
