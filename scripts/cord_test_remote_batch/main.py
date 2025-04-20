import os
import shutil
import tempfile
import copy
from dist import run_compile, dist_remote, clean_up
from remote_run import remote_run
from server_setup import start_all, kill_all

from config import conf 



if __name__ == '__main__': 
    for i in range(4,5): 
        conf_t = copy.deepcopy(conf)
        conf_t.TEST_ID = i
        conf_t.LOCAL_FILES.append(
            (f'{conf.LOCAL_PROJ_PATH}/gen_trace_out/{i}.log', 'trace.log')
        )
        run_compile(conf_t)  
        dist_remote(conf_t)
        start_all()
        remote_run(conf_t)
        kill_all()
        exit(1)
        # clean_up(conf_t)

