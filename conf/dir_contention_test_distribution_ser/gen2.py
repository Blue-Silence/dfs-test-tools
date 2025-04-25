import os

ROOT_PATH = ""

i = 0
for dir_size in [500, 1000]:
    for max_parallel in [10, 20, 30, 40, 50]:
        with open(f'./{i}.toml', 'w') as f:
            f.write(f"""
    thread = 4
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