import os

################## PATHS #####################
MDS_PATH = "/local/new_mds"

################## CLUSTER CONFIG #####################

SERVER_NODES = ['Finch@amd124.utah.cloudlab.us', 'Finch@amd113.utah.cloudlab.us','Finch@amd102.utah.cloudlab.us',]
RENAMER_NODES = ['Finch@amd124.utah.cloudlab.us',]
ALL_NODES = SERVER_NODES + RENAMER_NODES
CONFIG_DIR_PATH = f'/local/dfs-test-tools/scripts/cord_test_remote_batch/config'
ROOT_DIR = f'/dev/shm'

################## BUILD CONFIG #####################

FEATURES = [
    #'disable_inv_list',
    #'time_inv_list_check',
]