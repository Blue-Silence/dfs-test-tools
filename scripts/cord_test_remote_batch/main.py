import os
import shutil
import tempfile
import copy
import time
from dist import run_compile, dist_remote, clean_up
from remote_run import remote_run
from remote_data_retrieve import remote_data_retrieve

# For infinifs server start up only.
# from server_setup import start_all, kill_all

from config import conf 



if __name__ == '__main__': 
    for i in range(0,10): 
        for j in range(0,3):
            test_id = f'{i}-{j}'
            print(f'Test {test_id} start')
            conf_t = copy.deepcopy(conf)
            conf_t.TEST_ID = i
            conf_t.LOCAL_FILES.append(
                # (f'{conf.LOCAL_PROJ_PATH}/conf/dir_contention_test_ser/dir_contention_test_{i}.toml', 'conf.toml')
                (f'{conf.LOCAL_PROJ_PATH}/conf/dir_contention_test_distribution_ser/{i}.toml', 'conf.toml')
            )
            clean_up(conf_t)
            run_compile(conf_t)  
            dist_remote(conf_t)
            # start_all()
            remote_run(conf_t)
            remote_data_retrieve(conf_t, test_id)
            # kill_all()
            clean_up(conf_t)
            time.sleep(3)

