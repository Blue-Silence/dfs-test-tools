import os

TRACE_DIR = "./gen_trace_out"

cmds = [
    # "./mdtest '-a=DUMMY' '-CTEr' '-R' '-n' '60000' '-vvvvv' -I 10000"
]

for i in [100, 500, 1000, 10000, 100000]:
    for n in [1]: #]: //For now only 1 will work. Seems like a bug in mdtest. 
        cmds.append((f"./mdtest '-a=DUMMY' '-CTEr' '-R' '-n' '{i * n}' '-vvvvv' -I {i}", n))



if __name__ == '__main__':  
    for i, (c, dir_cnt) in enumerate(cmds):
        cmd = f'echo //// {c} >> {TRACE_DIR}/{i}.log'
        cmd += f' && echo DUMMY mkdir: ./out >> {TRACE_DIR}/{i}.log'
        for j in range(0, dir_cnt):
            cmd += f' && echo DUMMY mkdir: ./out/test-dir.0-{j} >> {TRACE_DIR}/{i}.log'
        cmd += f' && {c} | grep DUMMY >> {TRACE_DIR}/{i}.log'
        print(cmd)
        os.system(cmd)