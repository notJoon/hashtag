# hashtag

Tag tests to run only the tests you want

## How to use

You can use the `hashtag` macro to tag your tests. The macro takes as values the tags to filter on when testing. Each tag is a string, and you can specify multiple.

For example, if we want to tag some tests with `foo` and `bar`, we can do it like this:

```rust
use hashtag::hashtag;

#[test]
#[hashtag("foo")]
fn test_1() {
    assert!(true);
}

#[test]
#[hashtag("bar", "foo")]
fn test_2() {
    assert!(true);
}

#[test]
#[hashtag("baz")]
fn this_test_will_filtered() {
    assert!(true);
}
```

Then, you can run your tests with the hashtag CLI with -t flag. The flag takes as values the tags to filter on. Each tag is a string, and you can specify multiple.

```bash
hashtag -t foo
```

This will run only the tests tagged with `foo`, which in this case is `test_1` and `test_2`.

### Structure

> See [Macro document](hashtag_macros/README.md) for more information on how this macro works.

It would be nice to be able to use the parsed values from the macro, but I don't know how to send them to another file, so for now I have an external script (shell script) to parse the tags and test function names. This seems potentially inefficient, so my goal is to find a better way to do it as soon as possible, and I'll work on it over time.

Anyway, after mapping the tags and test function names I find, I run the tests, again using a shell script.

Rust has a feature where you can put a string as a trailing argument when running a `cargo test`, and it will [filter out tests](https://doc.rust-lang.org/cargo/commands/cargo-test.html#examples) that contain that string. To take advantage of this filtering feature, I made it recognize function names.

In other words, it takes all the test names with the tag I'm looking for and runs them one by one in the cargo test.

When searching for a tag, if there is an overlap, even if it is a different tag due to the grep attribute, it will be searched as well. We could have excluded this from the search, but we didn't because we think it could be extended to fuzzy search with some refinement.

Finally, since I implemented a simple string pattern search to get the test case name, it only searches for code that is purely within a certain range from the macro, so it may not work well if there are comments or other macros in between. For now, the default is if it's within 15 lines. I believe there's no outliers beyond that.
