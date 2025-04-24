import os

################## PATHS #####################
MDS_PATH = "/local/new_mds"

################## CLUSTER CONFIG #####################

SERVER_NODES = ['10.10.1.1', 
                '10.10.1.2', 
                '10.10.1.3', 
                '10.10.1.4', 
                '10.10.1.5', 
                '10.10.1.6', 
                '10.10.1.7', 
                '10.10.1.8', 
                '10.10.1.9', 
                '10.10.1.10', 
                ]
RENAMER_NODES = ['10.10.1.1']
ALL_NODES = SERVER_NODES + RENAMER_NODES
CONFIG_DIR_PATH = f'/local/dfs-test-tools/scripts/cord_test_remote_batch_reuse/config'
ROOT_DIR = f'/local'

################## BUILD CONFIG #####################

FEATURES = [
    #'disable_inv_list',
    #'time_inv_list_check',
]