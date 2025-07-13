use std::process::ExitCode;

fn main() -> ExitCode {
    // Only reached if run-bin code fails, otherwise process exits early from within
    // binary::run.
    if let Err(res) = cargo_run_bin::cli::run() {
        eprintln!("\x1b[31mrun-bin failed: {res}\x1b[0m");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
