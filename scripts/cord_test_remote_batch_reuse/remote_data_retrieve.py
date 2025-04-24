import threading
import paramiko
import time
import os


def remote_data_retrieve(conf, test_id):
    print(f"Start retrieving remote data.")
    tag = 0
    out_dir = f'{conf.OUT_PREFIX}/OUT_{test_id}'
    os.system(f'mkdir {out_dir}')
    for idx, host in enumerate(conf.REMOTE_NODES):
        remote_dir = f"TEST_NODE_{idx}/OUT"
        os.system(f'ssh {conf.USERNAME}@{host} "cd {conf.REMOTE_ROOT_DIR} && tar -czvf TEST_NODE_OUT_{idx}.tar.gz {remote_dir}"')
        os.system(f'scp {conf.USERNAME}@{host}:{conf.REMOTE_ROOT_DIR}/TEST_NODE_OUT_{idx}.tar.gz {out_dir}')
