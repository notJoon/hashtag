use clap::Parser;
use hashtag_core::TaggedTest;

pub mod file_diff;

#[derive(Debug, Parser)]
#[command(author, version, name = "hashtag", about = "Run tests with tags")]
struct Hashtag {
    #[clap(short, long = "tag", help = "Specify a tag to run the test")]
    #[arg(num_args(0..))]
    tags: Vec<String>,
    #[clap(short = 'a', long = "all", help = "Run all tests")]
    all: bool,
}

fn main() {
    let args = Hashtag::parse();

    if args.all {
        run_all_tests();
    } else if !args.tags.is_empty() {
        run_tagged_tests(&args.tags);
    } else {
        println!("Please specify tags or use --all to run all tests.");
    }
}

fn run_all_tests() {
    for test in inventory::iter::<TaggedTest> {
        println!("Running test: {}", test.name);
        (test.test_fn)();
    }
}

fn run_tagged_tests(tags: &[String]) {
    for test in inventory::iter::<TaggedTest> {
        if tags.iter().any(|tag| test.tags.contains(&tag.as_str())) {
            println!("Running test: {}", test.name);
            (test.test_fn)();
        }
    }
}
