pub struct TestLog {
    //idx: usize,
    log: Vec<LogEntry>,

}

impl TestLog {
    pub fn new(len: usize) -> Self {
        TestLog { log: Vec::with_capacity(len) }
    } 

    pub fn push(&mut self, op_type: &'static str, time: usize) {
        self.log.push(LogEntry{ op_type, time });
    }

    pub fn pop(&mut self) -> Option<String> {
        match self.log.pop() {
            Some(e) => Some(format!("{} {}", e.op_type, e.time)),
            None => None,
        }
    }
}

struct LogEntry {
    op_type: &'static str,
    time: usize,
}

