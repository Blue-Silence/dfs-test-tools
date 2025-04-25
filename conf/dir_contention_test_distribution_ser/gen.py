import os

ROOT_PATH = ""

i = 0
for max_parallel in [5, 10, 30, 20]:
    for zipf_s in [0.1, 0.5, 1, 2, 4]:
        with open(f'./{i}.toml', 'w') as f:
            f.write(f"""
    thread = 4
    max_parallel = {max_parallel}
    root_path = "{ROOT_PATH}"
    dir_cnt = 200
    dir_size = 1000
    zipf_s = {zipf_s}
    op_per_spawn = 6000
    distribution_type = "zipf"
                """)
        i += 1
# <class '_io.TextIOWrapper'>