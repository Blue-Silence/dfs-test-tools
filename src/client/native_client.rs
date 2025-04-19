use std::{fs::Permissions, os::unix::fs::PermissionsExt};

use tokio::fs::{self, create_dir, set_permissions};

use crate::FSClient;

pub struct NativeClient {

}

pub struct NativeClientFactory {

}

impl NativeClientFactory {
    pub fn init_param(&mut self, _: &str) {
    }

    pub fn new_client(&mut self) -> NativeClient {
        NativeClient {}
    }
}

impl FSClient for NativeClient {
    async fn create_dir(&mut self, path: &str) -> Result<(), String> {
        let re = tokio::fs::create_dir(path).await;
        to_re(re)
    }

    async fn change_permission(&mut self, path: &str, mode: u32) -> Result<(), String> {
        let perm = Permissions::from_mode(mode);  // 所有用户只能读，不能写
        let re = set_permissions(path, perm).await;
        to_re(re)
    }
    
    async fn file_stat(&mut self, path: &str) -> Result<(), String> {
        let re = fs::metadata(path).await;
        to_re(re)
    }

    async fn dir_stat(&mut self, path: &str) -> Result<(), String> {
        let re = fs::metadata(path).await;
        to_re(re)
    }

    async fn file_create(&mut self, path: &str) -> Result<(), String> {
        let re = fs::File::create(path).await;
        to_re(re)
    }
    
    async fn try_exist(&mut self, path: &str) -> Result<bool, String> {
        match fs::try_exists(path).await {
            Ok(true) => Ok(true),
            Ok(false) => Ok(false),
            Err(e) => return Err(format!("{:?}", e)),
        }
    }

}

fn to_re<A>(r: Result<A, impl std::fmt::Debug>) -> Result<(), String> {
    match r {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}