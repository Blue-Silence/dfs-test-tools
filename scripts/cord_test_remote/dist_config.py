import os

################## LOCAL PATHS #####################
HOME = os.path.expanduser("~")

LOCAL_PROJ_PATH = "/local/dfs-test-tools"
LOCAL_CONFIG_PATH = f'{LOCAL_PROJ_PATH}/conf/dir_contention_test.toml'
LOCAL_EXE_PATH = f'{LOCAL_PROJ_PATH}/target/release/main'

################## REMOTE PATHS #####################
REMOTE_ROOT_DIR = f'/dev/shm'

################## CLUSTER CONFIG #####################

REMOTE_NODES = ['amd197.utah.cloudlab.us', 'amd198.utah.cloudlab.us']
USERNAME = "Finch"


################## BUILD CONFIG #####################

FEATURES = [

]