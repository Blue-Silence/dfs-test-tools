import os

class CONF:
    def __init__(self):
        None

conf = CONF()
################## LOCAL PATHS #####################

conf.LOCAL_PROJ_PATH = "/local/dfs-test-tools"
conf.LOCAL_FILES = [
    (f'{conf.LOCAL_PROJ_PATH}/target/release/main', 'main'),
    (f'{conf.LOCAL_PROJ_PATH}/conf/trace_test.toml', 'conf.toml'),
    (f'{conf.LOCAL_PROJ_PATH}/scripts/cord_test_remote_batch/config/client.toml', 'client_config.toml'),
    (f'{conf.LOCAL_PROJ_PATH}/scripts/cord_test_remote_batch/config/global.toml', 'global_config.toml'),
    # (f'{conf.LOCAL_PROJ_PATH}/traces/1.log', 'trace.log'),
]

################## REMOTE PATHS #####################
conf.REMOTE_ROOT_DIR = f'/dev/shm'

################## CLUSTER CONFIG #####################

conf.REMOTE_NODES = ['amd117.utah.cloudlab.us', 'amd109.utah.cloudlab.us']
conf.USERNAME = "Finch"


################## BUILD CONFIG #####################

conf.FEATURES = [
    'infinifs_client'
]

################## TEST CONFIG #####################

conf.TEST_PROGRAM = "main"

conf.TEST_NAME = 'TraceTest'

conf.TEST_CONF = 'conf.toml'

conf.OUT_PREFIX = '/local/perf'

conf.REMOTE_TEST_ROOT = ''

conf.PARALLELISM = 4