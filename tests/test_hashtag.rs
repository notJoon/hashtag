#[cfg(test)]
mod dummy_tests {
    use hashtag_macros::hashtag;

    #[test]
    #[hashtag("foo")]
    fn foo() {
        assert!(true)
    }

    #[test]
    #[hashtag("foo", "bar")]
    fn bar() {
        assert!(true)
    }

    #[test]
    #[hashtag("foo", "bar", "baz")]
    fn baz() {
        assert!(true)
    }
}
