mod dir_contention_test;
mod dir_contention_test_2;
mod dir_contention_test_distribution;
mod dir_sync_test;
mod dir_sync_pair_test;
mod trace_test;

use crate::Test;

pub fn get_tests(name: &str) -> Option<Box<dyn Test>> {//HashMap<String, Box<dyn Test>> {
    match name {
        "DirContentionTest" => Some(Box::new(dir_contention_test::DirContentionTest::new())),
        "DirContentionTest2" => Some(Box::new(dir_contention_test_2::DirContentionTest::new())),
        "DirContentionTestDistribution" => Some(Box::new(dir_contention_test_distribution::DirContentionTest::new())),
        //"DirContentionMultiCliTest" => Some(Box::new(dir_contention_multi_cli_test::DirContentionMultiCliTest::new())),
        //"DirSyncTest" => Some(Box::new(dir_sync_test::DirSyncTest::new())),
        "DirSyncTest" => Some(Box::new(dir_sync_test::DirSyncTest::new())),
        "DirSyncPairTest" => Some(Box::new(dir_sync_pair_test::DirSyncPairTest::new())),
        "TraceTest" => Some(Box::new(trace_test::TraceTest::new())),
        _ => None,
    }
} 