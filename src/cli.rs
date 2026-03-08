use crate::bench::Scenario;
use crate::library::Libary;
#[allow(unused_imports)]
use crate::task::CodeLayout;

argwerk::define! {
    #[usage = "benchy run [options]"]
    pub struct RunArgs {
        pub libs: Vec<String>,
        pub scenarios: Vec<String>,
        pub layout: String = String::from("single-file"),

        pub jsony_path: Option<String>,
        pub profiles: Vec<String>,
        pub incrementals: Vec<String>,
        pub task: String = String::from("all"),
        pub results_dir: String = String::from("results"),
        pub samples: u32 = 5,
        help: bool,
    }
    ["-h" | "--help"] => {
        println!("{}", HELP);
        help = true;
    }
    /// Library to benchmark (comma-separated, repeatable).
    /// Available: jsony, serde, nanoserde, miniserde, midiserde, merde, musli, facet, sonic, baseline
    ["--lib", value] => {
        for lib in value.split(',') {
            libs.push(lib.trim().to_string());
        }
    }
    /// Scenario to run (comma-separated, repeatable).
    /// Available: warm-build, warm-check, clean-build, runtime
    ["--scenario", value] => {
        for s in value.split(',') {
            scenarios.push(s.trim().to_string());
        }
    }
    /// Build profile filter (comma-separated).
    /// Available: debug, release, release-lto, release-lto-native
    ["--profile", value] => {
        for p in value.split(',') {
            profiles.push(p.trim().to_string());
        }
    }
    /// Incremental mode filter (comma-separated).
    /// Available: disabled, unchanged, postfix, prefix, type-transform
    ["--incremental", value] => {
        for i in value.split(',') {
            incrementals.push(i.trim().to_string());
        }
    }
    /// Code layout: single-file, module, crate.
    ["--layout", value] => {
        layout = value;
    }

    /// Task to run: synthetic, social-only-json, social-with-binary, all (default: all).
    ["--task", value] => {
        task = value;
    }
    /// Directory for storing per-task per-library JSON result files (default: results).
    ["--results-dir", path] => {
        results_dir = path;
    }
    /// Path to local jsony checkout for testing.
    ["--jsony-path", path] => {
        jsony_path = Some(path);
    }
    /// Number of samples per benchmark (default: 5).
    ["--samples", n] => {
        samples = str::parse(&n)?;
    }
}

argwerk::define! {
    #[usage = "benchy report [options]"]
    pub struct ReportArgs {
        pub results_dir: String = String::from("results"),
        pub report_dir: String = String::from("."),
        pub task: String = String::from("all"),
        pub ignore_libs: Vec<String>,
        help: bool,
    }
    ["-h" | "--help"] => {
        println!("{}", HELP);
        help = true;
    }
    /// Directory containing per-task per-library JSON result files (default: results).
    ["--results-dir", path] => {
        results_dir = path;
    }
    /// Directory for writing generated reports (default: .).
    ["--report-dir", path] => {
        report_dir = path;
    }
    /// Filter by task: synthetic, social-only-json, social-with-binary, all (default: all).
    ["--task", value] => {
        task = value;
    }
    /// Libraries to exclude from reports (comma-separated, repeatable).
    ["--ignore-lib", value] => {
        for lib in value.split(',') {
            ignore_libs.push(lib.trim().to_string());
        }
    }
}

argwerk::define! {
    #[usage = "benchy generate [options]"]
    pub struct GenerateArgs {
        pub libs: Vec<String>,
        pub jsony_path: Option<String>,
        pub task: String = String::from("all"),
        pub output_dir: Option<String>,
        help: bool,
    }
    ["-h" | "--help"] => {
        println!("{}", HELP);
        help = true;
    }
    /// Library to generate (comma-separated, repeatable).
    /// Available: jsony, serde, nanoserde, miniserde, midiserde, merde, musli, facet, sonic, baseline
    ["--lib", value] => {
        for lib in value.split(',') {
            libs.push(lib.trim().to_string());
        }
    }
    /// Task to generate: synthetic, social-only-json, social-with-binary, all (default: all).
    ["--task", value] => {
        task = value;
    }
    /// Output directory for generated crates (default: /tmp/benchy/).
    ["--output-dir", path] => {
        output_dir = Some(path);
    }
    /// Path to local jsony checkout.
    ["--jsony-path", path] => {
        jsony_path = Some(path);
    }
}

argwerk::define! {
    #[usage = "benchy verify [options]"]
    pub struct VerifyArgs {
        pub libs: Vec<String>,
        pub jsony_path: Option<String>,
        help: bool,
    }
    ["-h" | "--help"] => {
        println!("{}", HELP);
        help = true;
    }
    /// Library to verify (comma-separated, repeatable).
    ["--lib", value] => {
        for lib in value.split(',') {
            libs.push(lib.trim().to_string());
        }
    }
    /// Path to local jsony checkout.
    ["--jsony-path", path] => {
        jsony_path = Some(path);
    }
}

#[derive(Debug)]
pub enum Command {
    Run(RunArgs),
    Report(ReportArgs),
    Generate(GenerateArgs),
    Verify(VerifyArgs),
}

argwerk::define! {
    /// Rust compile-time serialization benchmark suite.
    #[usage = "benchy <command> [options]"]
    pub struct Args {
        pub help: bool,
        #[required = "missing command: run, report, generate, or verify"]
        pub command: Command,
    }
    ["-h" | "--help"] => {
        println!("{}", HELP);
        help = true;
    }
    /// Run benchmarks.
    ["run", #[rest(os)] rest] if command.is_none() => {
        command = Some(Command::Run(RunArgs::parse(rest)?));
    }
    /// Generate reports from collected results.
    ["report", #[rest(os)] rest] if command.is_none() => {
        command = Some(Command::Report(ReportArgs::parse(rest)?));
    }
    /// Generate crates without running benchmarks.
    ["generate", #[rest(os)] rest] if command.is_none() => {
        command = Some(Command::Generate(GenerateArgs::parse(rest)?));
    }
    /// Verify social task outputs are correct across libraries.
    ["verify", #[rest(os)] rest] if command.is_none() => {
        command = Some(Command::Verify(VerifyArgs::parse(rest)?));
    }
}

pub fn resolve_libraries(libs: &[String], jsony_path: &Option<String>) -> Vec<Libary> {
    if libs.is_empty() {
        return Libary::default();
    }
    libs.iter()
        .filter_map(|name| {
            let lib = match name.as_str() {
                "jsony" => {
                    let path = jsony_path.clone();
                    Libary::Jsony { path }
                }
                "serde" => Libary::Serde,
                "nanoserde" => Libary::Nanoserde,
                "miniserde" => Libary::Miniserde,
                "midiserde" => Libary::Midiserde,
                "merde" => Libary::Merde,
                "musli" => Libary::Musli,
                "facet" => Libary::Facet,
                "sonic" | "sonic-rs" => Libary::Sonic,
                "baseline" => Libary::Baseline,
                other => {
                    eprintln!("Unknown library: {other}");
                    return None;
                }
            };
            Some(lib)
        })
        .collect()
}

pub fn resolve_scenarios(
    scenarios: &[String],
    profiles: &[String],
    incrementals: &[String],
) -> Vec<Scenario> {
    if scenarios.is_empty() {
        return Scenario::ALL.to_vec();
    }
    let all_profiles = profiles.is_empty();
    let all_incrementals = incrementals.is_empty();

    let mut result = Vec::new();
    for scenario_name in scenarios {
        match scenario_name.as_str() {
            "warm-build" => {
                for s in Scenario::ALL {
                    if let Scenario::WarmBuild {
                        incremental,
                        profile,
                    } = s
                    {
                        if (all_profiles
                            || profiles.iter().any(|p| p == profile.cli_name()))
                            && (all_incrementals
                                || incrementals.iter().any(|i| i == incremental.cli_name()))
                        {
                            result.push(s.clone());
                        }
                    }
                }
            }
            "warm-check" => {
                for s in Scenario::ALL {
                    if let Scenario::WarmCheck { incremental } = s {
                        if all_incrementals
                            || incrementals.iter().any(|i| i == incremental.cli_name())
                        {
                            result.push(s.clone());
                        }
                    }
                }
            }
            "clean-build" => {
                for s in Scenario::ALL {
                    if let Scenario::CleanBuild { profile } = s {
                        if all_profiles
                            || profiles.iter().any(|p| p == profile.cli_name())
                        {
                            result.push(s.clone());
                        }
                    }
                }
            }
            "runtime" => {
                for s in Scenario::ALL {
                    if let Scenario::RuntimeBenchmark { profile } = s {
                        if all_profiles
                            || profiles.iter().any(|p| p == profile.cli_name())
                        {
                            result.push(s.clone());
                        }
                    }
                }
            }
            other => {
                eprintln!("Unknown scenario: {other}");
            }
        }
    }
    result
}

pub fn resolve_layout(layout: &str) -> CodeLayout {
    match layout {
        "single-file" => CodeLayout::SingleFile,
        "module" => CodeLayout::SeparateModule,
        "crate" => CodeLayout::SeparateCrate,
        other => {
            eprintln!("Unknown layout: {other}, defaulting to single-file");
            CodeLayout::SingleFile
        }
    }
}
