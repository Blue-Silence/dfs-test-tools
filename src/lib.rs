
use std::fmt::Debug;

use client::native_client;
use tokio::fs::File;
pub trait Test {
    fn name(&self) -> &'static str;
    fn set_config(&mut self, config: String, unique_id: usize, all_task_cnt: usize);
    fn init(&mut self, c_gen: ClientGen) -> bool;
    fn run(&mut self) -> bool;
}

pub trait FSClient {
    async fn create_dir(&mut self, path: &str) -> Result<(), String>; //io::Result<()>;
    async fn change_permission(&mut self, path: &str, mode: u32) -> Result<(), String>;
    async fn file_stat(&mut self, path: &str) -> Result<(), String>;
    async fn dir_stat(&mut self, path: &str) -> Result<(), String>;
    async fn file_create(&mut self, path: &str) -> Result<FD, String>; //io::Result<()>;
    async fn try_exist(&mut self, path: &str) -> Result<bool, String>; //io::Result<()>;
    async fn close(&mut self, fd: FD) -> Result<(), String>; //io::Result<()>;
    async fn open(&mut self, path: &str) -> Result<FD, String>; //io::Result<()>;
    async fn delete(&mut self, path: &str) -> Result<(), String>; //io::Result<()>;
}


pub mod tests;
pub mod client;
pub mod trace;


pub type ClientGen = native_client::NativeClientFactory;
pub type Client = native_client::NativeClient;
pub type FD = File;