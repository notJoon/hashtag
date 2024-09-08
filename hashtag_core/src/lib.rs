pub use inventory;

/// Represents a test function tagged with one or more test filters.
///
/// This struct is used to store information about a test functio that has been
/// decorated with the `#[hashtag(...)]` attribute macro. It allows for easy
/// organization and filtering the selective funning of tests based on their tags.
///
/// # Fields
///
/// * `name` - The name of the test function as a static string slice.
/// * `tags` - A static array of string slices representing the tags associated with the test function.
/// * `test_fn` - A function pointer to the actual test function.
///
/// # Usage
///
/// This struct is tipycally created and registered automatically by the `hashtag` attribute macro.
/// It's then used by the test runner to determine which tests to run based on specified tags.
///
/// # Example
///
/// ```
/// #[hashtag("integration", "database")]
/// fn test_database_connection() {
///     // Test implementation...
/// }
/// ```
///
/// This would create a `TaggedTest` instance with:
/// - name: "test_database_connection"
/// - tags: &["integration", "database"]
/// - test_fn: a pointer to the test_database_connection function
pub struct TaggedTest {
    pub name: &'static str,
    pub tags: &'static [&'static str],
    pub test_fn: fn() -> (),
}

inventory::collect!(TaggedTest);
