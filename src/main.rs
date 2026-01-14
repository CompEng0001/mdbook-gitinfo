//! mdbook-gitinfo: inject Git metadata into mdBook chapters.

mod chapters;
mod layout;
mod processor;
mod renderer;
mod repo;
mod theme;
mod timefmt;

pub use mdbook_gitinfo::{config, git};

use clap::{arg, command, ArgMatches, Command};
use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::{parse_input, Preprocessor, MDBOOK_VERSION};
use processor::GitInfo;
use std::{io, process};

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = parse_input(io::stdin())?;

    if ctx.mdbook_version != MDBOOK_VERSION {   
        eprintln!(
            "Warning: The '{}' plugin was built against version {} of mdbook, but we're being called from version {}",
            pre.name(), MDBOOK_VERSION, ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Renderer required");
    let ok = match pre.supports_renderer(renderer) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e}");
            false
        }
    };
    process::exit(if ok { 0 } else { 1 });
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
