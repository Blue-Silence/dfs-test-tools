import threading
import subprocess
import time

from test_config import * 

blocking_output_1 = 'Ready'
blocking_output_2 = 'Done'
input_s = 'c'

class ProgramThread(threading.Thread):
    def __init__(self, command, input_s, blocking_output_1, barrier_1, blocking_output_2, barrier_2):
        super().__init__()
        self.command = command
        self.input_s = input_s
        self.blocking_output_1 = blocking_output_1
        self.barrier_1 = barrier_1
        self.blocking_output_2 = blocking_output_2
        self.barrier_2 = barrier_2

    def run(self):
        process = subprocess.Popen(self.command, shell=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

        # 检查输出，直到输出包含指定的阻塞点标志1
        output = ''
        while True:
            # 读取输出流
            output_line = process.stdout.readline()
            if output_line:
                output += output_line
                #print(f"Program output: {output_line.strip()}")  # 打印实时输出（可选）
            
            # 检查是否已到达阻塞点（例如包含指定的字符串）
            if self.blocking_output_1 in output:
                break
            else:
                print(f"{self.blocking_output_1} not in {output}")

        # 到达阻塞点后，等待其他程序一起同步
        #print('Barrier 1 reached')
        self.barrier_1.wait()

        # 所有程序同步后，继续执行（可以输入继续执行的指令）
        process.stdin.write(self.input_s + '\n')
        process.stdin.flush()

        start_time = time.perf_counter()  # 获取开始时间

        # 检查输出，直到输出包含指定的阻塞点标志2
        output = ''
        while True:
            # 读取输出流
            # print("Get:", output_line)
            output_line = process.stdout.readline()
            if output_line:
                output += output_line
                # print(f"Program output: {output_line.strip()}")  # 打印实时输出（可选）
            
            # 检查是否已到达阻塞点（例如包含指定的字符串）
            if self.blocking_output_2 in output:
                break

        end_time = time.perf_counter()  # 获取结束时间
        # print("Hit!")
        self.barrier_2.wait()

        # 捕获程序的最终输出和错误
        stdout, stderr = process.communicate()

        if stdout:
            print(f"Output of {self.command}: {stdout}")
        if stderr:
            print(f"Error in {self.command}: {stderr}")
        self.elapsed_time_us = (end_time - start_time) * 1_000_000  # 转换为微秒


def main():
    print(f"Test start")

    print(f"\tAll init start")

    # 创建Barrier对象，等待n+1个线程同步
    barrier_1 = threading.Barrier(PARALLELISM+1)
    barrier_2 = threading.Barrier(PARALLELISM+1)
    # 启动所有程序线程
    threads = []
    for tag in range(0, PARALLELISM):
        thread = ProgramThread(f"{TEST_PROGRAM} {TEST_NAME} {TEST_CONF_PATH} {tag} {PARALLELISM}", input_s, blocking_output_1, barrier_1, blocking_output_2, barrier_2)
        threads.append(thread)
        thread.start()
        print(f"\t\tInit {tag} done")

    barrier_1.wait()
    print('\tAll init done')
    start_time = time.perf_counter()  # 获取开始时间
    

    barrier_2.wait()
    end_time = time.perf_counter()  # 获取结束时间

    # 等待所有线程执行完毕
    for thread in threads:
        thread.join()

    elapsed_time = (end_time - start_time)  # 转换为微秒
    print(f"Test time: {elapsed_time:.2f}s")

    with open(f"{OUT_PREFIX}/{TEST_NAME}.txt", "a") as file:
        elapsed_time_us = (end_time - start_time) * 1_000_000  # 转换为微秒
        for t in threads:
            file.write(f"{t.elapsed_time_us:.2f} ")
        file.write(f"{elapsed_time_us:.2f} \n")

if __name__ == "__main__":
    main()
