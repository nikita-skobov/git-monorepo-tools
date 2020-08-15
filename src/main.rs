use git2::Repository;
use clap::{App, ArgMatches};

mod commands;

fn get_cli_input<'a>() -> ArgMatches<'a> {
    let mut base_app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"));

    base_app = base_app.subcommands(vec![
        commands::split_in(),
        commands::split_out()
    ]);

    return base_app.get_matches();
}

fn main() {
    let matches = get_cli_input();

    if let Some(submatches) = matches.subcommand_matches("split") {
        let iterator = submatches.value_of("repo_file").unwrap_or("");
        println!("{:?}", iterator);
    }

    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
}