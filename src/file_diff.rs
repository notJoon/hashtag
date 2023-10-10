// After run tests, hashtag has stored the test caches (test name and tags) in the cache file.
// We update the cache file by timestamp, but also by the check is file changed.
// That is, if the file changes, we update the cache file.

use std::collections::HashMap;

type FileMap = HashMap<String, Vec<String>>;

#[derive(Debug, Default, PartialEq)]
pub struct Caches {
    file_map: FileMap,
    has_changed: bool,
}

impl Caches {
    fn new() -> Self {
        Default::default()
    }

    fn has_file_changed(&self) -> bool {
        // get src and test files

        // get each file's check sums
        // use md5 to get the check sum

        // compare the check sums

        // if changed, update the cache file
        todo!()
    }

    fn timestamp(&self) -> u64 {
        // get the timestamp of the cache file
        todo!()
    }

    fn update_cache_file() {
        // update the cache file
    }
}