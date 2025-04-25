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
    # (f'{conf.LOCAL_PROJ_PATH}/scripts/cord_test_remote_batch_reuse/config/client.toml', 'client_config.toml'),
    # (f'{conf.LOCAL_PROJ_PATH}/scripts/cord_test_remote_batch_reuse/config/global.toml', 'global_config.toml'),
    # (f'{conf.LOCAL_PROJ_PATH}/traces/1.log', 'trace.log'),
    (f'{conf.LOCAL_PROJ_PATH}/gen_trace_out/0.log', '0.log'),
    (f'{conf.LOCAL_PROJ_PATH}/gen_trace_out/1.log', '1.log'),
    (f'{conf.LOCAL_PROJ_PATH}/gen_trace_out/2.log', '2.log'),
    (f'{conf.LOCAL_PROJ_PATH}/gen_trace_out/3.log', '3.log'),
    (f'{conf.LOCAL_PROJ_PATH}/gen_trace_out/4.log', '4.log'),
    (f'{conf.LOCAL_PROJ_PATH}/gen_trace_out/5.log', '5.log'),
    (f'{conf.LOCAL_PROJ_PATH}/gen_trace_out/6.log', '6.log'),
]

################## REMOTE PATHS #####################
conf.REMOTE_ROOT_DIR = f'/local'

################## CLUSTER CONFIG #####################

conf.REMOTE_NODES = ['10.10.1.4', '10.10.1.5']
conf.USERNAME = "Finch"


################## BUILD CONFIG #####################

conf.FEATURES = [
    'native_client'
]

################## TEST CONFIG #####################

conf.TEST_PROGRAM = "main"

conf.TEST_NAME = 'TraceTest'

conf.TEST_CONF = 'conf.toml'

conf.OUT_PREFIX = '/local/perf'

conf.REMOTE_TEST_ROOT = '/mnt/beegfs/foo'

conf.PARALLELISM = 1

conf.REUSE_INIT = False