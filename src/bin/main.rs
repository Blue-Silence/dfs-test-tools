use std::io::{self, Write};

use dfs_test_tools::{tests::get_tests, ClientGen};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let test_name = args[1].as_str();

    let conf =
        &std::fs::read_to_string(args[2].as_str()).expect("Should have been able to read the file");
    let unique_id = args[3].parse().unwrap();
    let all_task_cnt = args[4].parse().unwrap();

    let test = get_tests(test_name);
    if let None = test {
        panic!("Unknown test: {}", test_name);
    }
    let mut test = test.unwrap();

    test.set_config(conf.clone(), unique_id, all_task_cnt);

    test.init(ClientGen{});
    flush_out("Ready");

    wait_input();

    test.run();
    flush_out("Done");
}

fn wait_input() {
    let mut continue_singal = String::new();
    io::stdin().read_line(&mut continue_singal).expect("Failed to read line");
}

fn flush_out(s: &str) {
    println!("{}", s);
    io::stdout().flush().unwrap();
}