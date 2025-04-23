import os

class CONF:
    def __init__(self):
        None

conf = CONF()
################## LOCAL PATHS #####################

conf.LOCAL_PROJ_PATH = "/local/dfs-test-tools"
conf.LOCAL_FILES = [
    (f'{conf.LOCAL_PROJ_PATH}/target/release/main', 'main'),
    # (f'{conf.LOCAL_PROJ_PATH}/conf/dir_contention_test.toml', 'conf.toml'),
    # (f'{conf.LOCAL_PROJ_PATH}/scripts/cord_test_remote_batch/config/client.toml', 'client_config.toml'),
    # (f'{conf.LOCAL_PROJ_PATH}/scripts/cord_test_remote_batch/config/global.toml', 'global_config.toml'),
    # (f'{conf.LOCAL_PROJ_PATH}/traces/1.log', 'trace.log'),
]

################## REMOTE PATHS #####################
conf.REMOTE_ROOT_DIR = f'/dev/shm'

################## CLUSTER CONFIG #####################

conf.REMOTE_NODES = ['10.10.1.13', '10.10.1.14', '10.10.1.15']
conf.USERNAME = "Finch"


################## BUILD CONFIG #####################

conf.FEATURES = [
    'native_client'
]

################## TEST CONFIG #####################

conf.TEST_PROGRAM = "main"

conf.TEST_NAME = 'DirContentionTestDistribution'

conf.TEST_CONF = 'conf.toml'

conf.OUT_PREFIX = '/local/perf'

conf.REMOTE_TEST_ROOT = '/mnt/beegfs/foo'

conf.PARALLELISM = 1