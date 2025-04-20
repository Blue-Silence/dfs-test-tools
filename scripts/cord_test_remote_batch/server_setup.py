import os
import time
import pyrem.host

from server_setup_config import *


def kill_procs():
    cmd = [f'sudo pkill -INT -f mds_renamer ; \
            sudo pkill -INT -f mds_server']

    for node in ALL_NODES:
        host = pyrem.host.RemoteHost(node)
        task = host.run(cmd, quiet=False)
        pyrem.task.Parallel([task], aggregate=True).start(wait=True)
    
    # print(kill_tasks)
    print('KILLED PROCESSES')


def run_compile():
    features = ''
    if len(FEATURES) > 0:
        features = '--features "'
        for feat in FEATURES:
            features += feat + ' '
        features += '"'

    cmd = f'cd {MDS_PATH} && cargo build --release {features}'   
    print(cmd)
    return os.system(cmd)
 
def dist_server():
    for idx, dst in enumerate(SERVER_NODES):
        target_dir = f'{ROOT_DIR}/server_{idx}'
        cmd = f'ssh {dst} "rm -rf {target_dir}; mkdir {target_dir}" && \
            scp -r {CONFIG_DIR_PATH} {dst}:{target_dir}/ && \
            scp {MDS_PATH}/target/release/server {dst}:{target_dir}/mds_server'
        os.system(cmd)

def dist_renamer():
    for idx, dst in enumerate(RENAMER_NODES):
        target_dir = f'{ROOT_DIR}/renamer_{idx}'
        cmd = f'ssh {dst} "rm -rf {target_dir}; mkdir {target_dir}" && \
            scp -r {CONFIG_DIR_PATH} {dst}:{target_dir}/ && \
            scp {MDS_PATH}/target/release/renamer {dst}:{target_dir}/mds_renamer'
        os.system(cmd)

def run_server():
    server_tasks = []
    for idx, dst in enumerate(SERVER_NODES):
        print(f'RUNNING SERVER{idx}')
        target_dir = f'{ROOT_DIR}/server_{idx}'
        host = pyrem.host.RemoteHost(dst)
        cmd = [f'cd {target_dir} && ./mds_server config/global.toml config/s{idx}.toml']
        task = host.run(cmd, quiet=False)
        server_tasks.append(task)
    pyrem.task.Parallel(server_tasks, aggregate=True).start(wait=False)    
    time.sleep(3)
    print(f'Server is running')


def run_renamer():
    server_tasks = []
    for idx, dst in enumerate(RENAMER_NODES):
        print(f'RUNNING RENAMER{idx}')
        target_dir = f'{ROOT_DIR}/renamer_{idx}'
        host = pyrem.host.RemoteHost(dst)
        cmd = [f'cd {target_dir} && ./mds_renamer config/global.toml']
        task = host.run(cmd, quiet=False)
        server_tasks.append(task)
    pyrem.task.Parallel(server_tasks, aggregate=True).start(wait=False)    
    time.sleep(3)
    print(f'Renamer is running')


def start_all():
    run_compile()

    kill_procs()
    
    dist_server()
    dist_renamer()

    run_server()
    run_renamer()

def kill_all():
    kill_procs()
