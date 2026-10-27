//! `oci-arm-catcher` — grab Oracle Cloud's free ARM capacity the moment it appears.

mod args;
mod exit;
mod run;
mod settings;
mod ui;
mod update;

use std::process::ExitCode;

use std::time::Duration;

use clap::Parser as _;

use crate::args::Args;
use crate::update::UpdateCheck;

fn main() -> ExitCode {
    // clap exits with code 2 by itself on a parse error, matching our usage code.
    let args = Args::parse();
    let mut update = UpdateCheck::start();

    let code = match run::run(&args, &mut update) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            ui::report_error(&error);
            ExitCode::from(exit::code_for(&error))
        }
    };
    // A no-op if `run` already reported it.
    update.report(Duration::from_millis(1500));
    code
}
