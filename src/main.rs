#[macro_use]
extern crate clap;

mod features;
mod options;

use options::{BaseOptions, Subcommand};
use std::process;

fn main() {
    let BaseOptions { common_options, subcommand } = BaseOptions::parse();

    println!("Game base path has been set to: {}", common_options.game);

    let result: i32 = match subcommand {
        Subcommand::Install(options) => features::install::run(common_options, options),
        Subcommand::Uninstall(options) => features::uninstall::run(common_options, options),
        Subcommand::Update(options) => features::update::run(common_options, options),
        Subcommand::List(options) => features::list::run(common_options, options),
        Subcommand::Outdated(options) => features::outdated::run(common_options, options),
    };

    process::exit(result);
}
