import os

ROOT_PATH = ""

i = 0
for max_parallel in [10, 20, 30, 60]:
    for ratio in [10000, 1000, 100, 10]:
            with open(f'./{i}.toml', 'w') as f:
                f.write(f"""
    thread = 4
    max_parallel = {max_parallel}
    root_path = "{ROOT_PATH}"
    file_per_spawn = 2000
    op_per_spawn = 20000
    enable_mix = true
    set_permission_ratio = {ratio}
                    """)
            i += 1