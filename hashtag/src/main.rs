use std::process;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Hashtag {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand, PartialEq)]
enum Command {
    #[clap(about = "Specify a tag to run the test")]
    Tag {
        #[clap(
            short = 't',
            help = "
        The tag to search. split by whitespace.
        If no tag is specified, all tests will be run, like `cargo test`."
        )]
        #[arg(num_args(0..))]
        tags: Option<Vec<String>>,
    },
}

fn main() {
    let prog = Hashtag::parse();

    match prog.cmd {
        Command::Tag { tags } => handle_tag(tags),
    }
}

fn handle_tag(tags: Option<Vec<String>>) {
    tags.map_or_else(run_cargo_test, run_tagged_test);
}

fn run_cargo_test() {
    process::Command::new("cargo")
        .arg("test")
        .output()
        .expect("failed to execute process");
}

fn run_tagged_test(tags: Vec<String>) {
    tags.iter().for_each(|tag| {
        process::Command::new("sh")
            .arg("-c")
            .arg(format!("./run.sh {:?}", tag))
            .output()
            .expect("failed to execute process");
    });
}

#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn test_check_input() {
        let prog = Hashtag::parse_from(&["hashtag", "tag", "--tag", "test"]);
        assert_eq!(
            prog.cmd,
            Command::Tag {
                tags: Some(vec!["test".to_string()])
            }
        );
    }

    #[test]
    fn test_check_input_multiple() {
        let prog = Hashtag::parse_from(&["hashtag", "tag", "--tag", "test", "test2"]);
        assert_eq!(
            prog.cmd,
            Command::Tag {
                tags: Some(vec!["test".to_string(), "test2".to_string()])
            }
        );
    }
}
