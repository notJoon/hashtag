#[cfg(test)]
mod dummy_tests {
    use hashtag_macros::hashtag;

    #[test]
    #[hashtag("foo")]
    fn foo() {
        assert!(true)
    }

    #[test]
    #[hashtag("foo")]
    fn bar1() {
        assert!(true)
    }

    #[test]
    #[hashtag("tag_bar")]
    fn baz() {
        assert!(true)
    }

    #[test]
    #[hashtag("some_long_tag_name")]
    fn test_long_tag_name() {
        assert!(true)
    }
}
