//! mdbook-gitinfo: inject Git metadata into mdBook chapters.

mod processor;
mod timefmt;
mod renderer;
mod layout;
mod repo;
mod chapters;

pub use mdbook_gitinfo::{config, git};

use clap::{ArgMatches, Command, arg, command};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use std::{io, process};
use processor::GitInfo;

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The '{}' plugin was built against version {} of mdbook, but we're being called from version {}",
            pre.name(), mdbook::MDBOOK_VERSION, ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.get_one::<String>("renderer").expect("Renderer required");
    process::exit(if pre.supports_renderer(renderer) { 0 } else { 1 });
}

fn main() {
    let matches = command!("mdbook-gitinfo")
        .about("An mdBook preprocessor that injects Git metadata into the book")
        .subcommand(
            Command::new("supports")
                .arg(arg!(<renderer> "Renderer to check"))
                .about("Check renderer support"),
        )
        .get_matches();

    let pre = GitInfo::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&pre, sub_args);
    }

    if let Err(e) = handle_preprocessing(&pre) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
