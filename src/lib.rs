#![allow(warnings)] 

pub trait Test {
    fn name(&self) -> &'static str;
    fn set_config(&mut self, config: String, unique_id: usize, all_task_cnt: usize, out_dir: String);
    fn init(&mut self, c_gen: ClientGen) -> bool;
    fn run(&mut self) -> bool;
}

pub trait FSClient {
    async fn create_dir(&mut self, path: &String) -> Result<(), String>; //io::Result<()>;
    async fn dir_change_permission(&mut self, path: &String, mode: u32) -> Result<(), String>;
    async fn file_change_permission(&mut self, path: &String, mode: u32) -> Result<(), String>;
    async fn file_stat(&mut self, path: &String) -> Result<(), String>;
    async fn dir_stat(&mut self, path: &String) -> Result<(), String>;
    async fn file_create(&mut self, path: &String) -> Result<FD, String>; //io::Result<()>;
    async fn dir_try_exist(&mut self, path: &String) -> Result<bool, String>; //io::Result<()>;
    async fn close(&mut self, fd: FD) -> Result<(), String>; //io::Result<()>;
    async fn open(&mut self, path: &String) -> Result<FD, String>; //io::Result<()>;
    async fn delete(&mut self, path: &String) -> Result<(), String>; //io::Result<()>;
}


pub mod tests;
pub mod client;
pub mod trace;



#[cfg(feature = "native_client")]
use client::native_client;
#[cfg(feature = "native_client")]
pub type ClientGen = native_client::NativeClientFactory;
#[cfg(feature = "native_client")]
pub type Client = native_client::NativeClient;
#[cfg(feature = "native_client")]
pub type FD = tokio::fs::File;


#[cfg(feature = "infinifs_client")]
use client::infinifs_client;
#[cfg(feature = "infinifs_client")]
pub type ClientGen = infinifs_client::InfinifsClientFactory;
#[cfg(feature = "infinifs_client")]
pub type Client = infinifs_client::InfinifsClient;
#[cfg(feature = "infinifs_client")]
pub type FD = client::infinifs_client::FD;