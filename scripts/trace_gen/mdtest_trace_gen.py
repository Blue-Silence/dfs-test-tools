import os

TRACE_DIR = "./gen_trace_out"

cmds = [
    # "./mdtest '-a=DUMMY' '-CTEr' '-R' '-n' '60000' '-vvvvv' -I 10000"
]

for i in [100, 500, 1000, 10000, 100000]:
    for n in [1, 2, 4, 8, 16, 32, 64]: 
        cmds.append(f"./mdtest '-a=DUMMY' '-CTEr' '-R' '-n' '{i * n}' '-vvvvv' -I {i}")



if __name__ == '__main__':  
    for i, c in enumerate(cmds):
        cmd = f'echo //// {c} >> {TRACE_DIR}/{i}.log && {c} | grep DUMMY >> {TRACE_DIR}/{i}.log'
        print(cmd)
        os.system(cmd)