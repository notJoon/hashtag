use std::sync::{Arc, Mutex};

type GlobalBuilder = Arc<Mutex<Vec<u8>>>;

lazy_static::lazy_static! {
    pub static ref GLOBAL_BUILDER: GlobalBuilder = Arc::new(Mutex::new(Vec::new()));
}

pub fn test_println(msg: &str) {
    let mut buffer = GLOBAL_BUILDER.lock().unwrap();
    buffer.extend_from_slice(msg.as_bytes());
    buffer.push(b'\n');
}

// clear buffer and return the captured output
pub fn capture_output() -> String {
    let mut buffer = GLOBAL_BUILDER.lock().unwrap();
    let output = String::from_utf8(buffer.clone()).unwrap();
    buffer.clear();
    output
}

#[cfg(test)]
mod cli_tests {
    use hashtag_core::TaggedTest;
    use hashtag_macros::hashtag;

    use crate::{capture_output, test_println};

    #[hashtag("tag1")]
    fn test1() {
        println!("Running test: test1");
    }

    #[hashtag("tag2")]
    fn test2() {
        println!("Running test: test2");
    }

    #[hashtag("tag1", "tag3")]
    fn test3() {
        println!("Running test: test3");
    }

    fn run_all_tests() {
        for test in inventory::iter::<TaggedTest> {
            test_println(&format!("Running test: {}", test.name));
            (test.test_fn)();
        }
    }

    fn run_tagged_tests(tags: &[String]) {
        for test in inventory::iter::<TaggedTest> {
            if tags.iter().any(|tag| test.tags.contains(&tag.as_str())) {
                test_println(&format!("Running test: {}", test.name));
                (test.test_fn)();
            }
        }
    }

    #[test]
    fn test_run_all_tests() {
        run_all_tests();
        let output = capture_output();
        assert!(output.contains("Running test: test1"));
        assert!(output.contains("Running test: test2"));
        assert!(output.contains("Running test: test3"));
    }

    #[test]
    fn test_run_tagged_tests() {
        run_tagged_tests(&["tag1".to_string()]);
        let output = capture_output();
        assert!(output.contains("Running test: test1"));
        assert!(!output.contains("Running test: test2"));
        assert!(output.contains("Running test: test3"));
    }
}
