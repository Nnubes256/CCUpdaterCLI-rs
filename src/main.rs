#[macro_use]
extern crate clap;

mod cmd;
mod features;

use cmd::CLIBaseOptions;
use cmd::CLISubcommand;
use std::process;

fn main() {
    let top_args = CLIBaseOptions::parse();

    println!("Game base path has been set to: {}", top_args.game);

    let passed_args = top_args.clone();

    let result: i32 = match top_args.subcommand {
        CLISubcommand::Install(args) => features::install::run(passed_args, args),
        CLISubcommand::Uninstall(args) => features::uninstall::run(passed_args, args),
        CLISubcommand::Update(args) => features::update::run(passed_args, args),
        CLISubcommand::List(args) => features::list::run(passed_args, args),
        CLISubcommand::Outdated(args) => features::outdated::run(passed_args, args),
    };

    process::exit(result);
}
