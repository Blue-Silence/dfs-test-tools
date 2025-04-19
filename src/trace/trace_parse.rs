//use std::fs::File;
//use std::io::{self, BufRead};
use std::io;

// 定义 TraceEvent 的枚举类型
#[derive(Debug, Clone)]
pub enum TraceEvent {
    Create { path: String, address: String },
    Mkdir { path: String},
    Close { address: String },
    Open { path: String, address: String },
    Delete { path: String },
    FileStat { path: String },
    DirStat { path: String },
}

// 实现从字符串解析 TraceEvent
impl TraceEvent {

    fn from_str(s: &str) -> Option<Result<Self, ()>> {
        if s.starts_with("DUMMY create") {
            let parts: Vec<&str> = s.split("=").collect();
            let path_and_filename = parts[0].split("create: ").nth(1).unwrap().trim();
            let address = parts[1].trim();
            Some(Ok(TraceEvent::Create {
                path: path_and_filename.to_string(),
                address: address.to_string(),
            }))
        } else if s.starts_with("DUMMY close") {
            let parts: Vec<&str> = s.split_whitespace().collect();
            let address = parts[2];
            Some(Ok(TraceEvent::Close {
                address: address.to_string(),
            }))
        } else if s.starts_with("DUMMY open") {
            let parts: Vec<&str> = s.split("=").collect();
            let path_and_filename = parts[0].split("open: ").nth(1).unwrap().trim();
            let address = parts[1].trim();
            Some(Ok(TraceEvent::Open {
                path: path_and_filename.to_string(),
                address: address.to_string(),
            }))
        } else if s.starts_with("DUMMY delete") {
            let path_and_filename = s.split("delete: ").nth(1).unwrap().trim();
            Some(Ok(TraceEvent::Delete {
                path: path_and_filename.to_string(),
            }))
        } else if s.starts_with("DUMMY mkdir") {
            let path_and_filename = s.split("mkdir: ").nth(1).unwrap().trim();
            Some(Ok(TraceEvent::Mkdir {
                path: path_and_filename.to_string(),
            }))
        } else if s.starts_with("DUMMY file_stat") {
            let path_and_filename = s.split("file_stat: ").nth(1).unwrap().trim();
            Some(Ok(TraceEvent::FileStat {
                path: path_and_filename.to_string(),
            }))
        } else if s.starts_with("DUMMY dir_stat") {
            let path_and_filename = s.split("dir_stat: ").nth(1).unwrap().trim();
            Some(Ok(TraceEvent::DirStat {
                path: path_and_filename.to_string(),
            }))
        } else if s.starts_with("WARNING") {
            None
        } else if s.starts_with("////") {
            None
        }
        else {
            Some(Err(()))
        }
    }
}

// 读取文件并解析 trace
pub fn parse_trace(inp: &str) -> io::Result<Vec<TraceEvent>> {
    let mut events = Vec::new();
    for line in inp.lines() {
        match TraceEvent::from_str(&line) {
            Some(Ok(event)) => events.push(event),
            None => (),
            _ => panic!("Error when parsing:{}", line)
        }
    }
    Ok(events)
}