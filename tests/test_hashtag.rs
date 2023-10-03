#[cfg(test)]
mod dummy_tests {
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
