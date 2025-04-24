import os

ROOT_PATH = "/mnt/beegfs/foo"

i = 0
for dir_size in [500, 1000, 2000]:
    for max_parallel in [1, 3, 5, 10]:
        with open(f'./{i}.toml', 'w') as f:
            f.write(f"""
    thread = 8
    max_parallel = {max_parallel}
    root_path = "{ROOT_PATH}"
    dir_cnt = 200
    dir_size = {dir_size}
    zipf_s = 0
    op_per_spawn = 6000
    distribution_type = "even"
                """)
        i += 1
# <class '_io.TextIOWrapper'>