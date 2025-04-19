import os
from dist_config import *

def run_compile():
    features = ''
    if len(FEATURES) > 0:
        features = '--features "'
        for feat in FEATURES:
            features += feat + ' '
        features += '"'

    cmd = f'cd {LOCAL_PROJ_PATH} && cargo build --release {features}'   
    print(cmd)
    return os.system(cmd)
 
def dist_remote():
    for idx, dst in enumerate(REMOTE_NODES):
        target_dir = f'{REMOTE_ROOT_DIR}/TEST_NODE_{idx}'
        cmd = f'ssh {dst} "rm -rf {target_dir}; mkdir {target_dir}"'
        for file_p in LOCAL_FILES:
            cmd += f' && scp {file_p} {dst}:{target_dir}/'
        os.system(cmd)

if __name__ == '__main__':  
    run_compile()    
    dist_remote()
