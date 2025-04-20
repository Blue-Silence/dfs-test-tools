use crate::{Client, FSClient, FD};

use super::trace_parse::TraceEvent;

pub struct TraceEngine {
    addr_info: std::collections::HashMap<String, FD>,
    client: Client,
}

impl TraceEngine {
    pub fn new(client: Client) -> Self {
        TraceEngine {
            addr_info: std::collections::HashMap::new(),
            client,
        }
    }

    pub async fn exec(&mut self, e: TraceEvent, tag: &str, root_path: &str) {
        match e {
            TraceEvent::Create { path, address } => {
                let path = modify_path_with_suffix_and_root(path.as_str(), tag, root_path);
                let r = self.client.file_create(&path);
                let fd = r.await.unwrap();
                self.addr_info.insert(address, fd);
            }
            TraceEvent::Close { address } => {
                let fd = self.addr_info.remove(&address).unwrap();
                self.client.close(fd).await.unwrap();
                self.addr_info.remove(&address);
            }
            TraceEvent::Open { path, address } => {
                let path = modify_path_with_suffix_and_root(path.as_str(), tag, root_path);
                //println!("Open: {} = {}", path, address);
                let fd = self.client.open(&path).await.unwrap();
                self.addr_info.insert(address, fd);
            }
            TraceEvent::Delete { path } => {
                let path = modify_path_with_suffix_and_root(path.as_str(), tag, root_path);
                self.client.delete(&path).await.unwrap();
                //println!("Delete: {}", path);
            }
            TraceEvent::Mkdir { path } => {
                let path = modify_path_with_suffix_and_root(path.as_str(), tag, root_path);
                //println!("Final path is:{}", path);
                self.client.create_dir(&path).await.unwrap();
            }
            TraceEvent::FileStat { path } => {
                let path = modify_path_with_suffix_and_root(path.as_str(), tag, root_path);
                self.client.file_stat(&path).await.unwrap();
            }
            TraceEvent::DirStat { path } => {
                let path = modify_path_with_suffix_and_root(path.as_str(), tag, root_path);
                self.client.dir_stat(&path).await.unwrap();
            }
        }
    }
}

/* 
static OBJECT_ID: u128 = 12345;

fn split_path(path: &str) -> (String, String) {
    let path = std::path::Path::new(path);

    // 获取母路径
    let parent = path.parent().unwrap().to_str().unwrap().to_string();
    // 获取子路径（文件名或目录名）
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

    return (parent, file_name);
}
*/

fn modify_path_with_suffix_and_root(mut path: &str, suffix: &str, root_path: &str) -> String {
    //println!("Suffix:{}", suffix);
    if path.chars().last().unwrap() == '/' {
        path = &path[0..path.len()-1]
    }
    let mut re = path
        .split('/')
        .map(|segment| format!("{}{}", segment, suffix))
        .collect::<Vec<String>>();
    re[0] = root_path.to_string();
    //println!("Str: {:?}", re);
    re.join("/")
}
