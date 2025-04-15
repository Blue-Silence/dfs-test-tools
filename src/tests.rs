mod dir_contention_test;

use crate::Test;

pub fn get_tests(name: &str) -> Option<Box<dyn Test>> {//HashMap<String, Box<dyn Test>> {
    match name {
        "DirContentionTest" => Some(Box::new(dir_contention_test::DirContentionTest::new())),
        _ => None,
    }
} 