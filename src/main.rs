use std::process;

use clap::Parser;

mod file_diff;

#[derive(Debug, Parser)]
#[command(author, version, name = "hashtag", about = "Run tests with tags")]
struct Hashtag {
    #[clap(short, long = "tag", help = "Specify a tag to run the test")]
    #[arg(num_args(0..))]
    tags: Option<Vec<String>>,
    #[clap(short = 'a', long = "all", help = "Run all tests")]
    all: bool,
}

fn main() {
    let prog = Hashtag::parse();
    match prog {
        Hashtag {
            tags: None,
            all: true,
        } => run_cargo_test(),
        Hashtag {
            tags: Some(tags),
            all: false,
        } => get_tagged_test_name(tags),
        _ => println!("No tags specified"),
    }
}

fn run_cargo_test() {
    process::Command::new("cargo")
        .arg("test")
        .status()
        .expect("failed to execute process");
}

fn get_tagged_test_name(tags: Vec<String>) {
    tags.iter().for_each(|tag| {
        process::Command::new("bash")
            .arg("-C")
            .arg("script/tag.sh")
            .arg(tag)
            .status()
            .expect("failed to execute process");
    })
}

#[cfg(test)]
mod cli_tests {
    use super::*;
    use hashtag_macros::hashtag;

    #[test]
    #[hashtag("input")]
    fn test_check_input() {
        let prog = Hashtag::parse_from(&["hashtag", "-t"]);
        assert_eq!(prog.tags, Some(vec![]));

        let prog = Hashtag::parse_from(&["hashtag", "-t", "test"]);
        assert_eq!(prog.tags, Some(vec!["test".to_string()]));
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
