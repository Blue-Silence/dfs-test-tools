TEST_PROGRAM = "/local/dfs-test-tools/target/release/main"
TEST_NAME = 'DirContentionTest'

TEST_CONF_PATH = '/local/dfs-test-tools/conf/dir_contention_test.toml'

OUT_PREFIX = '/local/perf'

PARALLELISM = 8

TAGS = 'abcdefghijklmnopqrstuvwxyz'[slice(0,PARALLELISM)]