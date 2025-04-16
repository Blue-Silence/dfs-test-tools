mod dir_contention_test;
mod dir_sync_test;

use crate::Test;

pub fn get_tests(name: &str) -> Option<Box<dyn Test>> {//HashMap<String, Box<dyn Test>> {
    match name {
        "DirContentionTest" => Some(Box::new(dir_contention_test::DirContentionTest::new())),
        //"DirContentionMultiCliTest" => Some(Box::new(dir_contention_multi_cli_test::DirContentionMultiCliTest::new())),
        //"DirSyncTest" => Some(Box::new(dir_sync_test::DirSyncTest::new())),
        "DirSyncTest" => Some(Box::new(dir_sync_test::DirSyncTest::new())),
        _ => None,
    }
} 