use std::{fs::Permissions, os::unix::fs::PermissionsExt};

use tokio::fs;

use crate::{FSClient, FD};

pub struct NativeClient {}

pub struct NativeClientFactory {}

impl NativeClientFactory {
    pub fn new() -> Self {
        NativeClientFactory {  }
    }

    pub fn new_client(&mut self) -> NativeClient {
        NativeClient {}
    }
}

impl FSClient for NativeClient {
    async fn create_dir(&mut self, path: &String) -> Result<(), String> {
        let re = tokio::fs::create_dir(path).await;
        to_re(re)
    }

    async fn dir_change_permission(&mut self, path: &String, mode: u32) -> Result<(), String> {
        let perm = Permissions::from_mode(mode); // 所有用户只能读，不能写
        let re = tokio::fs::set_permissions(path, perm).await;
        to_re(re)
    }

    async fn file_change_permission(&mut self, path: &String, mode: u32) -> Result<(), String> {
        let perm = Permissions::from_mode(mode); // 所有用户只能读，不能写
        let re = tokio::fs::set_permissions(path, perm).await;
        to_re(re)
    }

    async fn file_stat(&mut self, path: &String) -> Result<(), String> {
        let re = fs::metadata(path).await;
        to_re(re)
    }

    async fn dir_stat(&mut self, path: &String) -> Result<(), String> {
        let re = fs::metadata(path).await;
        to_re(re)
    }

    async fn file_create(&mut self, path: &String) -> Result<FD, String> {
        let re = fs::File::create(path).await;
        match re {
            Ok(f) => Ok(f),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    async fn dir_try_exist(&mut self, path: &String) -> Result<bool, String> {
        match fs::try_exists(path).await {
            Ok(true) => Ok(true),
            Ok(false) => Ok(false),
            Err(e) => return Err(format!("{:?}", e)),
        }
    }

    async fn close(&mut self, _: FD) -> Result<(), String> {
        Ok(())
    }

    async fn open(&mut self, path: &String) -> Result<FD, String> {
        let re = fs::File::open(path).await;
        match re {
            Ok(f) => Ok(f),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    async fn delete(&mut self, path: &String) -> Result<(), String> {
        let re = fs::remove_file(path).await;
        to_re(re)
    }
}

fn to_re<A>(r: Result<A, impl std::fmt::Debug>) -> Result<(), String> {
    match r {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}
