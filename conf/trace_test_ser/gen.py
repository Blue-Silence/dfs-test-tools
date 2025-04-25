import os

ROOT_PATH = "/mnt/beegfs/foo"

i = 0
for trace in [0, 1, 2, 3, 4, 5, 6]:
    for max_parallel in [10, 20, 30, 40, 60]:
            with open(f'./{i}.toml', 'w') as f:
                f.write(f"""
    trace_path = "./{trace}.log"
    thread = 16
    max_parallel = {max_parallel}
    root_path = "{ROOT_PATH}"
    iter_per_spawn = 2
                    """)
            i += 1
# <class '_io.TextIOWrapper'>