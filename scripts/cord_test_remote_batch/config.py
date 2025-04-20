import os

class CONF:
    def __init__(self):
        None

conf = CONF()
################## LOCAL PATHS #####################

conf.LOCAL_PROJ_PATH = "/local/dfs-test-tools"
conf.LOCAL_FILES = [
    (f'{conf.LOCAL_PROJ_PATH}/target/release/main', 'main'),
    (f'{conf.LOCAL_PROJ_PATH}/conf/dir_contention_test.toml', 'conf.toml'),
    # (f'{conf.LOCAL_PROJ_PATH}/traces/1.log', 'trace.log'),
]

################## REMOTE PATHS #####################
conf.REMOTE_ROOT_DIR = f'/dev/shm'

################## CLUSTER CONFIG #####################

conf.REMOTE_NODES = ['amd197.utah.cloudlab.us', 'amd198.utah.cloudlab.us']
conf.USERNAME = "Finch"


################## BUILD CONFIG #####################

conf.FEATURES = [

]

################## TEST CONFIG #####################

conf.TEST_PROGRAM = "main"

conf.TEST_NAME = 'DirContentionTest'

conf.TEST_CONF = 'dir_contention_test.toml'

conf.OUT_PREFIX = '/local/perf'

conf.REMOTE_TEST_ROOT = './dev/shm/foo'

conf.PARALLELISM = 1