TEST_PROGRAM = "/main"
TEST_NAME = 'DirContentionTest'

TEST_CONF_PATH = '/local/mds/infnifs_mds_test_client/scripts/cord_test/perf.toml'

OUT_PREFIX = '/local/perf'

PARALLELISM = 16

TAGS = 'abcdefghijklmnopqrstuvwxyz'[slice(0,PARALLELISM)]