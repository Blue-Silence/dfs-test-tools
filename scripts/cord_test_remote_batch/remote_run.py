import threading
import paramiko
import time

blocking_output_1 = 'Ready'
blocking_output_2 = 'Done'
input_s = 'c'

class RemoteProgramThread(threading.Thread):
    def __init__(self, command, input_s, blocking_output_1, barrier_1, blocking_output_2, barrier_2, host, username):
        super().__init__()
        self.command = command
        self.input_s = input_s
        self.blocking_output_1 = blocking_output_1
        self.barrier_1 = barrier_1
        self.blocking_output_2 = blocking_output_2
        self.barrier_2 = barrier_2
        self.host = host
        self.username = username

    def run(self):
        ssh_client = paramiko.SSHClient()
        ssh_client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
        ssh_client.connect(self.host, username=self.username)


        ssh_stdin, ssh_stdout, ssh_stderr = ssh_client.exec_command(self.command)
        # print(f"Exec: {self.command}")

        # 检查输出，直到输出包含指定的阻塞点标志1
        output = ''
        while True:
            # 读取输出流
            output_line = ssh_stdout.readline()
            if output_line:
                output += output_line
                #print(f"Program output: {output_line.strip()}")  # 打印实时输出（可选）
            
            # 检查是否已到达阻塞点（例如包含指定的字符串）
            if self.blocking_output_1 in output:
                break
            else:
                print(f"{self.blocking_output_1} not in {output}")
                print(ssh_stdout.read().decode())
                print(ssh_stderr.read().decode())
                exit(1)

        # 到达阻塞点后，等待其他程序一起同步
        #print('Barrier 1 reached')
        self.barrier_1.wait()

        # 所有程序同步后，继续执行（可以输入继续执行的指令）
        ssh_stdin.write(self.input_s + '\n')
        ssh_stdin.flush()

        start_time = time.perf_counter()  # 获取开始时间

        # 检查输出，直到输出包含指定的阻塞点标志2
        output = ''
        while True:
            # 读取输出流
            # print("Get:", output_line)
            output_line = ssh_stdout.readline()
            if output_line:
                output += output_line
                # print(f"Program output: {output_line.strip()}")  # 打印实时输出（可选）
            
            # 检查是否已到达阻塞点（例如包含指定的字符串）
            if self.blocking_output_2 in output:
                break

        end_time = time.perf_counter()  # 获取结束时间
        # print("Hit!")
        self.barrier_2.wait()

        stdout_o = ssh_stdout.read().decode()
        stderr_o = ssh_stderr.read().decode()

        if stdout_o:
            print(f"Output of {self.command}: {stdout_o}")
        if stderr_o:
            print(f"Error in {self.command}: {stderr_o}")
        self.elapsed_time_us = (end_time - start_time) * 1_000_000  # 转换为微秒


def remote_run(conf):
    print(f"Test start")

    print(f"\tAll init start")

    total_parallel = len(conf.REMOTE_NODES) * conf.PARALLELISM
    # 创建Barrier对象，等待n+1个线程同步
    barrier_1 = threading.Barrier(total_parallel+1)
    barrier_2 = threading.Barrier(total_parallel+1)
    # 启动所有程序线程
    threads = []
    tag = 0
    for idx, host in enumerate(conf.REMOTE_NODES):
        remote_dir = f"{conf.REMOTE_ROOT_DIR}/TEST_NODE_{idx}"
        for _ in range(0, conf.PARALLELISM):
            thread = RemoteProgramThread(f"cd {remote_dir} && {remote_dir}/{conf.TEST_PROGRAM} {conf.TEST_NAME} {remote_dir}/{conf.TEST_CONF} {tag} {total_parallel}", input_s, blocking_output_1, barrier_1, blocking_output_2, barrier_2, host, conf.USERNAME)
            threads.append(thread)
            thread.start()
            print(f"\t\tInit {tag} on node {host} start")
            tag = tag + 1

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

    with open(f"{conf.OUT_PREFIX}/{conf.TEST_NAME}_{conf.TEST_ID}.txt", "a") as file:
        elapsed_time_us = (end_time - start_time) * 1_000_000  # 转换为微秒
        for t in threads:
            file.write(f"{t.elapsed_time_us:.2f} ")
        file.write(f"{elapsed_time_us:.2f} \n")

