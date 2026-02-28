mod clap_app;
mod commands;
mod completions;
mod config;
mod diagnostic;
mod error;
mod log_macros;
mod ops;
mod prompts;
mod tui;
mod utils;
#[cfg(not(debug_assertions))]
mod version;

use clap::Parser;

use clap_app::ClapApp;

use crate::error::AppResult;

#[cfg(not(debug_assertions))]
fn check_for_updates() -> AppResult<()> {
    use crate::version::get_latest_version;
    use semver::Version;

    let latest_version = get_latest_version()?;
    let current_version = Version::parse(env!("CARGO_PKG_VERSION"))?;

    if latest_version > current_version {
        warning_msg!("{} -> {}", current_version, latest_version);
    }

    Ok(())
}

fn run() -> AppResult<()> {
    #[cfg(not(debug_assertions))]
    check_for_updates()?;

    ClapApp::parse().run()
}

fn main() {
    match run() {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            error_msg!(e);
            std::process::exit(1);
        }
    }
}
