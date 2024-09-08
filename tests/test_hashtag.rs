#[cfg(test)]
mod parsing {
    use hashtag_macros::hashtag;

    #[hashtag("foo", "another_name")]
    #[test]
    #[should_panic]
    // some comment
    // another comment
    // 1
    // 2

    // 3
    fn foo() {
        assert!(false)
    }

    #[test]
    #[hashtag("bar")]
    fn bar1() {
        assert!(true)
    }

    #[test]
    #[hashtag("tag_tag_bar")]
    fn baz() {
        assert!(true)
    }

    #[test]
    #[hashtag("some_long_tag_name", "another_long_tag_name")]
    fn test_long_tag_name() {
        assert!(true)
    }
}

use hashtag_core::TaggedTest;
use hashtag_macros::hashtag;
use std::panic;

// Helper function to run a test and check if it panics
fn test_panics(f: impl FnOnce() -> () + panic::UnwindSafe) -> bool {
    panic::catch_unwind(f).is_err()
}

#[test]
fn test_hashtag_macro_basic() {
    #[hashtag("basic")]
    fn test_function() {
        assert!(true);
    }

    // Check if the function is still a test
    assert!(test_panics(|| {
        panic!("This should be caught");
    }));

    test_function();
}

#[test]
fn test_hashtag_macro_multiple_tags() {
    #[hashtag("tag1", "tag2")]
    fn test_function() {
        assert!(true);
    }

    assert!(test_panics(|| {
        panic!("This should be caught");
    }));

    test_function();
}

#[test]
fn test_hashtag_macro_empty_tags() {
    #[hashtag()]
    fn test_function() {
        assert!(true);
    }

    assert!(test_panics(|| {
        panic!("This should be caught");
    }));

    test_function();
}

#[test]
fn test_hashtag_macro_doc_comment() {
    #[hashtag("doc_test")]
    fn test_function() {
        assert!(true);
    }

    let _: () = {
        const _HASHTAG_TAGS: &str = "doc_test";
    };
}

// [WARN] This test requires the `inventory` crate to be set up
#[test]
fn test_hashtag_macro_inventory() {
    #[hashtag("inventory_test")]
    fn test_function() {
        assert!(true);
    }

    // Check if the test is registered in the inventory
    let found = inventory::iter::<TaggedTest>
        .into_iter()
        .any(|test| test.name == "test_function" && test.tags.contains(&"inventory_test"));
    assert!(found, "Test function not found in inventory");
}

#[test]
fn test_hashtag_macro_multiple_attributes() {
    #[hashtag("multi_attr1", "multi_attr2")]
    #[test]
    fn test_function() {
        assert!(true);
    }

    // This should not panic
    test_function();

    // Use the compiler to check for the presence of multiple tags
    let _: () = {
        const _HASHTAG_TAGS: &str = "multi_attr1, multi_attr2";
    };
}
