use std::process;

use clap::Parser;
use hashtag_macros::hashtag;

#[derive(Debug, Parser)]
#[command(author, version, name = "hashtag", about = "Run tests with tags")]
struct Hashtag {
    #[clap(short, long = "tag", help = "Specify a tag to run the test")]
    #[arg(num_args(0..))]
    tags: Option<Vec<String>>,
}

fn main() {
    let prog = Hashtag::parse();

    match prog.tags {
        Some(tags) => run_tagged_test(tags),
        None => run_cargo_test(),
    }
}

fn run_cargo_test() {
    process::Command::new("cargo")
        .arg("test")
        .output()
        .expect("failed to execute process");
}

fn run_tagged_test(tags: Vec<String>) {
    tags.iter().for_each(|tag| {
        process::Command::new("bash")
            .arg("-c")
            .arg(format!("./tag_parser.sh {:?}", tag))
            .output()
            .expect("failed to execute process");
    });
}

// #[hashtag("foo")]
// fn foo() {
//     println!("foo");
// }

// #[hashtag("some long tag")]
// fn another_foo() {
//     println!("another foo");
// }

#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn test_check_input() {
        let prog = Hashtag::parse_from(&["hashtag", "-t"]);
        assert_eq!(
            prog.tags,
            Some(vec![])
        );

        let prog = Hashtag::parse_from(&["hashtag", "-t", "test"]);
        assert_eq!(
            prog.tags,
            Some(vec!["test".to_string()])
        );
    }

    #[test]
    fn test_check_multiple_tags() {
        let prog = Hashtag::parse_from(&["hashtag", "-t", "test", "test2"]);
        assert_eq!(
            prog.tags,
            Some(vec!["test".to_string(), "test2".to_string()])
        );
    }
}
