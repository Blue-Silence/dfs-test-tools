import os

################## LOCAL PATHS #####################
HOME = os.path.expanduser("~")

LOCAL_PROJ_PATH = "/local/dfs-test-tools"
LOCAL_CONFIG_PATH = f'{LOCAL_PROJ_PATH}/conf/dir_contention_test.toml'
LOCAL_EXE_PATH = f'{LOCAL_PROJ_PATH}/conf/dir_contention_test.toml'

################## REMOTE PATHS #####################
REMOTE_ROOT_DIR = f'/dev/shm'

################## CLUSTER CONFIG #####################

REMOTE_NODES = ['Finch@amd152.utah.cloudlab.us', 'Finch@amd152.utah.cloudlab.us','Finch@amd152.utah.cloudlab.us','Finch@amd152.utah.cloudlab.us',]


################## BUILD CONFIG #####################

FEATURES = [

]