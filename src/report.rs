use std::collections::HashMap;
use std::io::Write;
use std::process::Stdio;

use crate::bench::{Perf, Scenario};

pub fn format_model_source(raw: &[u8]) -> Vec<u8> {
    let text = String::from_utf8_lossy(raw);
    let mut out = String::with_capacity(text.len());
    let mut pending_derives: Vec<String> = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(inner) = trimmed
            .strip_prefix("#[derive(")
            .and_then(|s| s.strip_suffix(")]"))
        {
            for item in inner.split(',') {
                let item = item.trim();
                if !item.is_empty() {
                    pending_derives.push(item.to_string());
                }
            }
            continue;
        }
        if !pending_derives.is_empty() {
            out.push_str("#[derive(");
            out.push_str(&pending_derives.join(", "));
            out.push_str(")]\n");
            pending_derives.clear();
        }
        out.push_str(line);
        out.push('\n');
    }
    // Add blank lines after closing braces to separate items
    let out = out.replace("}\n", "}\n\n");
    crate::token::pipe_rustfmt(out.as_bytes())
}

pub fn normalize_string(file: &str) -> String {
    let mut last_underscore = false;
    let mut temp: String = file
        .chars()
        .filter_map(|ch| {
            if ch.is_alphanumeric() || ch == '-' {
                last_underscore = false;
                Some(ch)
            } else {
                let v = if last_underscore { None } else { Some('_') };
                last_underscore = true;
                v
            }
        })
        .collect();
    if temp.as_bytes().last() == Some(&b'_') {
        temp.pop();
    }
    temp
}

pub struct Plotter {
    s1: &'static str,
    s2: &'static str,
    s3: &'static str,
    s4: &'static str,
    s5: &'static str,
    s6: &'static str,
}

impl Plotter {
    pub fn new() -> Plotter {
        let fs = include_str!("plot.gnuplot");
        let (s1, rest) = fs
            .split_once("__INSERT_HEIGHT_HERE__")
            .expect("Missing __INSERT_HEIGHT_HERE__");
        let (s2, rest) = rest
            .split_once("__INSERT_LABEL_HERE__")
            .expect("Missing __INSERT_LABEL_HERE__");
        let (s3, rest) = rest
            .split_once("__INSERT_DATA_HERE__")
            .expect("Missing __INSERT_DATA_HERE__");
        let (s4, rest) = rest
            .split_once("__INSERT_XMAX_HERE__")
            .expect("Missing __INSERT_XMAX_HERE__");
        let (s5, s6) = rest
            .split_once("__INSERT_VALUE_FMT_HERE__")
            .expect("Missing __INSERT_VALUE_FMT_HERE__");
        Plotter {
            s1,
            s2,
            s3,
            s4,
            s5,
            s6,
        }
    }

    pub fn plot(
        &self,
        label: &str,
        value_fmt: &str,
        data: impl IntoIterator<Item = (impl AsRef<str>, f64)>,
    ) -> anyhow::Result<String> {
        let mut render = String::new();
        let mut max_val: f64 = 0.0;
        let mut count: usize = 0;
        for (key, value) in data.into_iter() {
            use std::fmt::Write;
            writeln!(render, "{:?} {}", key.as_ref(), value)?;
            if value > max_val {
                max_val = value;
            }
            count += 1;
        }
        self.plot_inner(label, value_fmt, &render, max_val, count)
    }

    fn plot_inner(
        &self,
        label: &str,
        value_fmt: &str,
        content: &str,
        max_val: f64,
        num_bars: usize,
    ) -> anyhow::Result<String> {
        let mut render = String::new();
        render.push_str(self.s1);
        use std::fmt::Write;
        // Experimentally derived: produces ~20px tall bars in gnuplot's SVG output
        let height = num_bars * 24 + 116;
        write!(render, "{height}")?;
        render.push_str(self.s2);
        write!(render, "{label:?}")?;
        render.push_str(self.s3);
        render.push_str(content);
        render.push_str(self.s4);
        let xmax = max_val * 1.2;
        write!(render, "{xmax}")?;
        render.push_str(self.s5);
        write!(render, "{value_fmt:?}")?;
        render.push_str(self.s6);
        let mut gnuplot = std::process::Command::new("gnuplot")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        {
            gnuplot.stdin.take().unwrap().write_all(render.as_bytes())?;
        }
        let output = gnuplot.wait_with_output()?;
        Ok(String::from_utf8(output.stdout)?)
    }
}

struct LibOrder<'a> {
    rank: HashMap<&'a str, usize>,
    sorted: Vec<&'a str>,
}

impl<'a> LibOrder<'a> {
    /// Compute a global library ordering based on median "x times slower than fastest"
    /// score across all scenarios. Libraries are ranked fastest to slowest.
    fn from_scenarios(by_scenario: &HashMap<Scenario, PerfSet<'a>>) -> Self {
        let mut scores_by_lib: HashMap<&str, Vec<f64>> = HashMap::new();
        for set in by_scenario.values() {
            let min_duration = set
                .targets
                .iter()
                .map(|(_, perf)| {
                    let d = set.normalized_duration(perf);
                    if d <= 0.0 {
                        1e-6
                    } else {
                        d
                    }
                })
                .fold(f64::INFINITY, f64::min);
            if min_duration == f64::INFINITY {
                continue;
            }
            for (name, perf) in &set.targets {
                let d = set.normalized_duration(perf);
                let d = if d <= 0.0 { 1e-6 } else { d };
                scores_by_lib
                    .entry(name)
                    .or_default()
                    .push(d / min_duration);
            }
        }
        let mut lib_medians: Vec<(&str, f64)> = scores_by_lib
            .into_iter()
            .map(|(name, mut scores)| {
                scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mid = scores.len() / 2;
                let median = if scores.len() % 2 == 0 {
                    (scores[mid - 1] + scores[mid]) / 2.0
                } else {
                    scores[mid]
                };
                (name, median)
            })
            .collect();
        lib_medians.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let sorted: Vec<&str> = lib_medians.iter().map(|(name, _)| *name).collect();
        let rank: HashMap<&str, usize> = sorted.iter().enumerate().map(|(i, n)| (*n, i)).collect();
        LibOrder { rank, sorted }
    }

    fn pos(&self, name: &str) -> usize {
        self.rank.get(name).copied().unwrap_or(usize::MAX)
    }
}

#[derive(Default)]
struct PerfSet<'a> {
    baseline: Option<Perf>,
    targets: Vec<(&'a str, Perf)>,
}

impl<'a> PerfSet<'a> {
    fn normalized_duration(&self, perf: &Perf) -> f64 {
        if let Some(baseline) = &self.baseline {
            perf.normalized(baseline).duration.value
        } else {
            perf.duration.value
        }
    }
}

pub fn format_results(data: Vec<(&str, Vec<(String, String)>, Vec<(Scenario, Perf)>)>) {
    let mut by_scenario: HashMap<Scenario, PerfSet> = HashMap::with_capacity(32);
    let mut lib_versions: Vec<(&str, Vec<(String, String)>)> = Vec::new();

    for (lib, versions, scenarios) in data {
        if !versions.is_empty() {
            lib_versions.push((lib, versions));
        }
        for (scenario, perf) in scenarios {
            let set = by_scenario.entry(scenario).or_default();
            if lib == "baseline" {
                set.baseline = Some(perf);
            } else {
                set.targets.push((lib, perf));
            }
        }
    }

    let order = LibOrder::from_scenarios(&by_scenario);

    lib_versions.sort_by_key(|(lib, _)| order.pos(lib));
    for (lib, versions) in &lib_versions {
        print!("{lib} versions:");
        for (name, version) in versions {
            print!(" {name}={version}");
        }
        println!();
    }

    for set in by_scenario.values_mut() {
        set.targets.sort_by_key(|(name, _)| order.pos(name));
    }

    for scenario in Scenario::ALL {
        let Some(set) = by_scenario.get(scenario) else {
            continue;
        };
        println!(">>> {:?}", scenario);
        for (name, perf) in &set.targets {
            let normalized = if let Some(baseline) = &set.baseline {
                &perf.normalized(baseline)
            } else {
                perf
            };
            println!("{:>11}: {}", name, normalized);
        }
    }

    println!("\n\n Baselines");
    for scenario in Scenario::ALL {
        let Some(set) = by_scenario.get_mut(scenario) else {
            continue;
        };
        if let Some(baseline) = &set.baseline {
            println!(">>> {:?}", scenario);
            println!("{}", baseline);
        }
    }
}

pub fn format_markdown_results(
    data: Vec<(&str, Vec<(String, String)>, Vec<(Scenario, Perf)>)>,
    report_dir: &str,
    report_prefix: &str,
) -> anyhow::Result<()> {
    let mut out: Vec<u8> = Vec::new();
    let mut by_scenario: HashMap<Scenario, PerfSet> = HashMap::with_capacity(32);

    let plotter = Plotter::new();

    let mut lib_versions: Vec<(String, Vec<(String, String)>)> = Vec::new();
    for (lib, versions, scenarios) in data {
        if !versions.is_empty() {
            lib_versions.push((lib.to_string(), versions));
        }
        for (scenario, perf) in scenarios {
            let set = by_scenario.entry(scenario).or_default();
            if lib == "baseline" {
                set.baseline = Some(perf);
            } else {
                set.targets.push((lib, perf));
            }
        }
    }

    let order = LibOrder::from_scenarios(&by_scenario);
    for set in by_scenario.values_mut() {
        set.targets.sort_by_key(|(name, _)| order.pos(name));
    }
    lib_versions.sort_by_key(|(lib, _)| order.pos(lib));

    let title = task_title(report_prefix);
    let description = task_description(report_prefix);

    writeln!(out, "# {title}\n")?;
    if !description.is_empty() {
        writeln!(out, "{description}\n")?;
    }
    let model_path = format!("{report_dir}/report/{report_prefix}_model.rs");
    if std::path::Path::new(&model_path).exists() {
        writeln!(
            out,
            "Data model: [{report_prefix}_model.rs]({report_prefix}_model.rs)\n"
        )?;
    }

    writeln!(out, "## Benchmark Results\n")?;
    writeln!(out, "### Libraries")?;
    for name in &order.sorted {
        let primary_version =
            lib_versions
                .iter()
                .find(|(lib, _)| lib == name)
                .and_then(|(_, versions)| {
                    versions
                        .iter()
                        .filter(|(crate_name, _)| crate_name.contains(name))
                        .min_by_key(|(crate_name, _)| crate_name.len())
                        .map(|(_, ver)| ver.as_str())
                });
        if let Some(ver) = primary_version {
            writeln!(out, "- {name} v{ver}")?;
        } else {
            writeln!(out, "- {name}")?;
        }
    }
    writeln!(out)?;

    let class_descriptions: &[(&str, &str)] = &[
        ("Warm Check", "All dependencies cached and prebuilt, only the bin crate is checked. Rustc is invoked directly using the same parameters as cargo."),
        ("Warm Build", "All dependencies cached and prebuilt, only the bin crate is rebuilt. Rustc is invoked directly using the same parameters as cargo."),
        ("Clean Build", "Dependencies in global cache but target is empty after `cargo clean`. Measures full `cargo build` time."),
        ("Runtime Benchmark", "Run the built executable with JSON input and high iteration count."),
        ("Binary Size", "Stripped binary size of the built executable."),
    ];

    let has_incremental = by_scenario.keys().any(|s| match s {
        Scenario::WarmBuild { incremental, .. } | Scenario::WarmCheck { incremental } => {
            !matches!(incremental, crate::bench::Incremental::Disabled)
        }
        _ => false,
    });
    if has_incremental {
        writeln!(out, "### Incremental Modes")?;
        writeln!(
            out,
            "- `Disabled`: `-C incremental` wasn't specified in the rustc invocation"
        )?;
        writeln!(
            out,
            "- `Unchanged`: Rebuild when no file content changed (`touch src/main.rs`)"
        )?;
        writeln!(out, "- `Postfix`: New content added to end of the module")?;
        writeln!(out, "- `Prefix`: New content added to start of the module")?;
        writeln!(out)?;
    }

    writeln!(out, "### Metrics")?;
    writeln!(out, "Measured with Linux `perf stat`:")?;
    writeln!(out, "- **Duration**: Wall-clock time in milliseconds")?;
    writeln!(out, "- **Bcycles**: Billion CPU cycles")?;
    writeln!(out, "- **Binst**: Billion CPU instructions")?;
    writeln!(out, "- **task-clock**: CPU clock time across all cores")?;
    writeln!(out)?;

    // Collect all sections: (class_name, section_title, section_body)
    let mut sections: Vec<(&str, String, Vec<u8>)> = Vec::new();

    for scenario in Scenario::ALL {
        let Some(set) = by_scenario.get_mut(scenario) else {
            continue;
        };

        let Some((min_name, _)) = set
            .targets
            .iter()
            .min_by_key(|(_, perf)| perf.duration.value as u64)
        else {
            panic!();
        };
        let plot = plotter.plot(
            "Time (ms)",
            "%d ms",
            set.targets.iter().map(|(name, perf)| {
                let key = if name == min_name {
                    format!("{{/:Bold {}}}", name)
                } else {
                    name.to_string()
                };
                let normalized = if let Some(baseline) = &set.baseline {
                    &perf.normalized(baseline)
                } else {
                    perf
                };
                (key, normalized.duration.value / 1e6)
            }),
        )?;
        let scenario_str = format!("{:?}", scenario);
        let svg_name = normalize_string(&scenario_str);

        let svg_subdir = if report_prefix.is_empty() {
            format!("{report_dir}/report")
        } else {
            format!("{report_dir}/report/{report_prefix}")
        };
        std::fs::create_dir_all(&svg_subdir)?;
        std::fs::write(format!("{svg_subdir}/{svg_name}.svg"), plot.as_bytes())?;
        let relative_svg = if report_prefix.is_empty() {
            String::new()
        } else {
            format!("{report_prefix}/")
        };

        let mut body: Vec<u8> = Vec::new();
        body.extend_from_slice(b"\n\n");
        writeln!(body, "![{svg_name}]({relative_svg}{svg_name}.svg)")?;
        body.extend_from_slice(b"\n\n");
        body.extend_from_slice(b"\n```rust\n");
        for (name, perf) in &set.targets {
            let normalized = if let Some(baseline) = &set.baseline {
                &perf.normalized(baseline)
            } else {
                perf
            };
            writeln!(body, "{:>11}: {}", name, normalized)?;
        }
        body.extend_from_slice(b"```\n\n");
        if let Some(baseline) = &set.baseline {
            writeln!(body, "Baseline reference stats: `{}`", baseline)?;
        }

        sections.push((scenario.class_name(), scenario_str, body));
    }

    for (profile, profile_label) in [
        (crate::bench::BuildProfile::Release, "Release"),
        (crate::bench::BuildProfile::ReleaseLto, "Release LTO"),
    ] {
        let scenario = Scenario::RuntimeBenchmark { profile };
        let Some(set) = by_scenario.get(&scenario) else {
            continue;
        };
        let has_sizes = set
            .targets
            .iter()
            .any(|(_, perf)| perf.build_size.is_some());
        if !has_sizes {
            continue;
        }
        let baseline_size = set
            .baseline
            .as_ref()
            .and_then(|b| b.build_size)
            .unwrap_or(0);

        let Some((min_name, _)) = set
            .targets
            .iter()
            .filter_map(|(name, perf)| {
                perf.build_size
                    .map(|s| (*name, s.saturating_sub(baseline_size)))
            })
            .min_by_key(|(_, size)| *size)
        else {
            continue;
        };

        let plot = plotter.plot(
            "Size (KB)",
            "%d KB",
            set.targets.iter().filter_map(|(name, perf)| {
                let size = perf.build_size?;
                let normalized = size.saturating_sub(baseline_size);
                let key = if *name == min_name {
                    format!("{{/:Bold {}}}", name)
                } else {
                    name.to_string()
                };
                Some((key, normalized as f64 / 1024.0))
            }),
        )?;

        let svg_name = format!("BinarySize_profile_{}", normalize_string(profile_label));
        let title = format!("Binary Size ({})", profile_label);

        let svg_subdir = if report_prefix.is_empty() {
            format!("{report_dir}/report")
        } else {
            format!("{report_dir}/report/{report_prefix}")
        };
        std::fs::create_dir_all(&svg_subdir)?;
        std::fs::write(format!("{svg_subdir}/{svg_name}.svg"), plot.as_bytes())?;
        let relative_svg = if report_prefix.is_empty() {
            String::new()
        } else {
            format!("{report_prefix}/")
        };

        let mut body: Vec<u8> = Vec::new();
        body.extend_from_slice(b"\n\n");
        writeln!(body, "![{svg_name}]({relative_svg}{svg_name}.svg)")?;
        body.extend_from_slice(b"\n\n");
        body.extend_from_slice(b"\n```rust\n");
        for (name, perf) in &set.targets {
            if let Some(size) = perf.build_size {
                let normalized = size.saturating_sub(baseline_size);
                writeln!(body, "{:>11}: {} KB (stripped)", name, normalized / 1024)?;
            }
        }
        body.extend_from_slice(b"```\n\n");
        if baseline_size > 0 {
            writeln!(
                body,
                "Baseline binary size: `{} KB (stripped)`",
                baseline_size / 1024
            )?;
        }

        sections.push(("Binary Size", title, body));
    }

    // Build table of contents and grouped output
    writeln!(out, "## Table of Contents\n")?;
    for &(class, desc) in class_descriptions {
        if sections.iter().any(|(c, _, _)| *c == class) {
            let anchor = class.to_lowercase().replace(' ', "-");
            writeln!(out, "- [{}](#{}): {}", class, anchor, desc)?;
        }
    }
    writeln!(out)?;

    // Write grouped sections in defined order
    for &(class, _) in class_descriptions {
        let mut has_header = false;
        for (c, title, body) in &sections {
            if *c != class {
                continue;
            }
            if !has_header {
                writeln!(out, "## {}\n", class)?;
                has_header = true;
            }
            writeln!(out, "### {}", title)?;
            out.extend_from_slice(body);
        }
    }

    if !lib_versions.is_empty() {
        writeln!(out, "### Crate Versions")?;
        for (lib, versions) in lib_versions.iter() {
            write!(out, "- **{lib}**: ")?;
            for (i, (name, version)) in versions.iter().enumerate() {
                if i > 0 {
                    write!(out, ", ")?;
                }
                write!(out, "{name} {version}")?;
            }
            writeln!(out)?;
        }
        writeln!(out)?;
    }

    let md_name = if report_prefix.is_empty() {
        format!("{report_dir}/report/BENCH.md")
    } else {
        format!("{report_dir}/report/BENCH-{report_prefix}.md")
    };
    std::fs::create_dir_all(format!("{report_dir}/report"))?;
    std::fs::write(&md_name, &out)?;
    Ok(())
}

fn cmd_output(cmd: &str, args: &[&str]) -> String {
    std::process::Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok()
            } else {
                None
            }
        })
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn collect_system_info(out: &mut Vec<u8>) -> std::io::Result<()> {
    let rustc_v = cmd_output("rustc", &["--version"]);
    let llvm_ver = {
        let verbose = cmd_output("rustc", &["--version", "--verbose"]);
        verbose
            .lines()
            .find_map(|l| l.strip_prefix("LLVM version: "))
            .unwrap_or("")
            .to_string()
    };
    let kernel = cmd_output("uname", &["-r"]);
    let distro = std::fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|s| {
            s.lines().find_map(|l| {
                l.strip_prefix("PRETTY_NAME=")
                    .map(|v| v.trim_matches('"').to_string())
            })
        })
        .unwrap_or_default();

    let lscpu = cmd_output("lscpu", &[]);
    let mut cpu_model = String::new();
    let mut cpus = String::new();
    let mut caches = Vec::new();
    for line in lscpu.lines() {
        let Some((key, val)) = line.split_once(':') else {
            continue;
        };
        let (key, val) = (key.trim(), val.trim());
        match key {
            "Model name" => cpu_model = val.to_string(),
            "CPU(s)" => cpus = val.to_string(),
            "L1d cache" | "L1i cache" | "L2 cache" | "L3 cache" => {
                caches.push(format!("{}: {}", key.split(' ').next().unwrap_or(key), val));
            }
            _ => {}
        }
    }

    let mem_gb = std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|s| {
            s.lines().find_map(|l| {
                l.strip_prefix("MemTotal:").and_then(|v| {
                    let kb: u64 = v.trim().split_whitespace().next()?.parse().ok()?;
                    Some(format!("{:.0} GB", kb as f64 / 1_048_576.0))
                })
            })
        })
        .unwrap_or_default();

    writeln!(out, "## System Information\n")?;
    writeln!(out, "```")?;
    writeln!(out, "rustc:   {rustc_v} (LLVM {llvm_ver})")?;
    writeln!(out, "os:      {distro} (kernel {kernel})")?;
    writeln!(out, "cpu:     {cpu_model} ({cpus} threads)")?;
    writeln!(out, "memory:  {mem_gb}")?;
    if !caches.is_empty() {
        writeln!(out, "caches:  {}", caches.join(", "))?;
    }
    writeln!(out, "```\n")?;
    Ok(())
}

fn task_title(task: &str) -> &str {
    match task {
        "synthetic" => "Synthetic Benchmark",
        "social-only-json" => "Social Model with JSON Only",
        "social-with-binary" => "Social Model with JSON + Binary",
        other => other,
    }
}

fn task_description(task: &str) -> &str {
    match task {
        "synthetic" => "Generates 75 random structs with random field types to test raw derive macro throughput and compilation overhead.",
        "social-only-json" => "Uses a realistic social media data model (users, posts, comments, reactions) to test JSON-only serialization & deserialization.",
        "social-with-binary" => "Same social media data model but testing both JSON and binary serialization & deserialization.",
        _ => "",
    }
}

pub fn generate_readme(report_dir: &str, tasks: &[&str]) -> anyhow::Result<()> {
    let prelude = include_str!("readme_prelude.md");
    let (before, after) = prelude
        .split_once("<!-- INSERT_BENCHMARKS_HERE -->")
        .expect("readme_prelude.md must contain <!-- INSERT_BENCHMARKS_HERE -->");

    let mut out: Vec<u8> = Vec::new();
    out.extend_from_slice(before.as_bytes());

    for task in tasks {
        let title = task_title(task);
        let description = task_description(task);

        writeln!(out, "## {title}\n")?;
        if !description.is_empty() {
            writeln!(out, "{description}\n")?;
        }
        let model_path = format!("{report_dir}/report/{task}_model.rs");
        if std::path::Path::new(&model_path).exists() {
            writeln!(
                out,
                "Data model: [{task}_model.rs](report/{task}_model.rs)\n"
            )?;
        }
        let agg_path = format!("{report_dir}/report/{task}_aggregate.png");
        if std::path::Path::new(&agg_path).exists() {
            writeln!(out, "![{task} aggregate](report/{task}_aggregate.png)\n")?;
        }
        writeln!(
            out,
            "### See [detailed results](report/BENCH-{task}.md) for per-scenario breakdowns.\n"
        )?;
    }

    collect_system_info(&mut out)?;

    out.extend_from_slice(after.as_bytes());
    writeln!(out)?;

    std::fs::write(format!("{report_dir}/README.md"), &out)?;
    Ok(())
}
