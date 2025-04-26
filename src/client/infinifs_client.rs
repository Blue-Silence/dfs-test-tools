use mds::const_def::ROOT_PERMISSION;
use mds::server::db::type_def::{IdT, NameT};
use mds::{client::Client, config};

use crate::FSClient;

pub struct InfinifsClient {
    cli: Client,
}

pub struct InfinifsClientFactory {
    global_config: String,
    client_config: String,
}

pub type FD = (IdT, NameT);

impl InfinifsClientFactory {
    pub fn new(global_config_path: &str, client_config_path: &str) -> Self {
        let global_config = std::fs::read_to_string(global_config_path)
            .expect("Should have been able to read the file");
        let client_config = std::fs::read_to_string(client_config_path)
            .expect("Should have been able to read the file");
        InfinifsClientFactory {
            global_config,
            client_config,
        }
    }

    pub fn new_client(&mut self) -> InfinifsClient {
        let global_cfg = config::GlobalInfo::new(&self.global_config);
        let client_cfg = config::ClientConfig::new(&self.client_config);
        InfinifsClient {
            cli: Client::new(global_cfg, client_cfg).unwrap(),
        }
    }
}

impl FSClient for InfinifsClient {
    async fn create_dir(&mut self, path: &String) -> Result<(), String> {
        loop {
            let re = self.cli.mkdir(path, ROOT_PERMISSION()).await;
            if !test_lock(&re) {
                return to_re(re);
            }
            //println!("CREATE_DIR Looping for lock");
            tokio::time::sleep(tokio::time::Duration::from_micros(1000)).await;
        }
    }

    async fn dir_change_permission(&mut self, path: &String, mode: u32) -> Result<(), String> {
        //let re = self.cli.dir_set_permission(path, ROOT_PERMISSION()).await;
        //to_re(re)

        loop {
            let re = self.cli.dir_set_permission(path, ROOT_PERMISSION()).await;
            if !test_lock(&re) {
                return to_re(re);
            }
            //println!("CREATE_DIR Looping for lock");
            tokio::time::sleep(tokio::time::Duration::from_micros(1000)).await;
        }
    }

    async fn file_change_permission(&mut self, path: &String, mode: u32) -> Result<(), String> {
        let re1 = self.cli.shared_open(path).await;
        let (pid, val) = match re1 {
            Ok(v) => v,
            Err(e) => return Err(format!("{:?}", e)),
        };
        let name = get_last_name(path);
        let name = &name.as_bytes().to_vec();
        let re2 = self.cli.modify(&pid, name, val).await;
        match re2 {
            Ok(v) => v,
            Err(e) => return Err(format!("{:?}", e)),
        };
        let re3 = self.cli.close(&pid, name).await;
        to_re(re3)
    }

    async fn file_stat(&mut self, path: &String) -> Result<(), String> {
        loop {
            let re = self.cli.file_stat(path).await;
            if !test_lock(&re) {
                return to_re(re);
            }
            //println!("STAT Looping for lock");
            tokio::time::sleep(tokio::time::Duration::from_micros(1000)).await;
        }
    }

    async fn dir_stat(&mut self, path: &String) -> Result<(), String> {
        let re = self.cli.dir_stat(path).await;
        to_re(re)
    }

    async fn file_create(&mut self, path: &String) -> Result<FD, String> {
        loop {
            let (dir_path, name) = split_path(path);
            let name = name.as_bytes().to_vec();
            let re = self
                .cli
                .file_create(&dir_path, &name, 123, ROOT_PERMISSION(), true)
                .await;
            if test_lock(&re) {
                tokio::time::sleep(tokio::time::Duration::from_micros(1000)).await;
                continue;
            }
            match re {
                Ok(pid) => Ok((pid, name)),
                Err(e) => Err(format!("{:?}", e)),
            };  
        }
    }

    async fn dir_try_exist(&mut self, path: &String) -> Result<bool, String> {
        let re = self.cli.dir_stat(path).await;
        match re {
            Ok(_) => Ok(true),
            Err(e) => match e {
                mds::client::user_error::UserError::RemoteError(mds::error::Error::MDSError(mds::error::MDSError::NotExist(_))) =>Ok(false),
                _ => Err(format!("{:?}", e)),
            },
        }
    }

    async fn close(&mut self, fd: FD) -> Result<(), String> {
        let re = self.cli.close(&fd.0, &fd.1).await;
        to_re(re)
    }

    async fn open(&mut self, path: &String) -> Result<FD, String> {
        let name = get_last_name(path);
        let name = name.as_bytes().to_vec();
        let re = self.cli.unique_open(path).await;
        match re {
            Ok(f) => Ok((f.0, name)),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    async fn delete(&mut self, path: &String) -> Result<(), String> {
        let re = self.cli.file_delete(path).await;
        to_re(re)
    }
}

fn to_re<A>(r: Result<A, impl std::fmt::Debug>) -> Result<(), String> {
    match r {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

fn get_last_name(inp: &String) -> &str {
    inp.split('/').last().unwrap()
}

fn split_path(inp: &String) -> (String, &str) {
    let it: Vec<_> = inp.split('/').collect();
    let par_p = &it[0..it.len() - 1];
    (par_p.join("/"), it[it.len() - 1])
}

fn test_lock<T>(re: &Result<T, mds::client::user_error::UserError>) -> bool {
    match re {
        Err(mds::client::user_error::UserError::RemoteError(mds::error::Error::MDSError(
            mds::error::MDSError::Locked(_),
        ))) => true,
        _ => false,
    }
}
