use hashtag_macros::hashtag;

#[hashtag("foo")]
fn foo() {
    println!("foo");
}

#[hashtag("foo", "bar")]
fn bar() {
    println!("bar");
}

#[hashtag("foo", "bar", "baz")]
fn baz() {
    println!("baz");
}