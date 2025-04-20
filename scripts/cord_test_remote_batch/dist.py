import os

def run_compile(conf):
    features = ''
    if len(conf.FEATURES) > 0:
        features = '--features "'
        for feat in conf.FEATURES:
            features += feat + ' '
        features += '"'

    cmd = f'cd {conf.LOCAL_PROJ_PATH} && cargo build --release {features}'   
    print(cmd)
    return os.system(cmd)
 
def dist_remote(conf):
    for idx, dst in enumerate(conf.REMOTE_NODES):
        target_dir = f'{conf.REMOTE_ROOT_DIR}/TEST_NODE_{idx}'
        cmd = f'ssh {dst} "rm -rf {target_dir}; mkdir {target_dir}"'
        for (file_p, file_n) in conf.LOCAL_FILES:
            cmd += f' && scp {file_p} {dst}:{target_dir}/{file_n}'
        os.system(cmd)

def clean_up(conf):
    for idx, dst in enumerate(conf.REMOTE_NODES):
        target_dir = f'{conf.REMOTE_TEST_ROOT}'
        cmd = f'ssh {dst} "rm -rf {target_dir}/*"'
        os.system(cmd)

