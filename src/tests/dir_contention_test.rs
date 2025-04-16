use serde::Deserialize;
use std::io::{self, Write};
use tokio::fs::{self, create_dir, set_permissions};

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use crate::Test;

#[derive(Deserialize, Debug, Clone)]
struct TestConfig {
    pub thread: usize,
    pub max_parallel: usize,

    pub root_path: String,
    pub dir_cnt: usize,
    pub file_per_dir_per_spwan: usize,
    pub op_per_spawn: usize,
    pub skew_dir_cnt: usize,
}

pub struct DirContentionTest {
    conf: Option<TestConfig>,
    unique_id: usize,
    all_task_cnt: usize,

    //Then starts the unique part for the test.
    file_ps: Vec<Vec<String>>,
}

impl DirContentionTest {
    pub fn new() -> Self {
        DirContentionTest {
            conf: None,
            unique_id: 0,
            all_task_cnt: 0,
            file_ps: vec![],
        }
    }
}

impl Test for DirContentionTest {
    fn name(&self) -> &'static str {
        "Dir Contention Test"
    }

    fn set_config(&mut self, config: String, unique_id: usize, all_task_cnt: usize) {
        self.conf = Some(toml::from_str(&config).unwrap());
        self.unique_id = unique_id;
        self.all_task_cnt = all_task_cnt;
    }

    //#[tokio::main]
    fn init(&mut self) -> bool {
        if let None = self.conf {
            return false;
        }
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(self.conf.clone().unwrap().thread)
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { self.init_help().await })
    }

    fn run(&self) -> bool {
        if let None = self.conf {
            return false;
        }
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(self.conf.clone().unwrap().thread)
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { self.run_help().await })
    }
}

async fn wait_until_exist(p: &str) -> Result<(), std::io::Error> {
    loop {
        match fs::try_exists(p).await {
            Ok(true) => return Ok(()),
            Ok(false) => tokio::time::sleep(std::time::Duration::from_secs(1)).await,
            Err(e) => return Err(e),
        }
    }
}

fn all_file_ps_gen(all_task_cnt: usize, conf: &TestConfig) -> Vec<Vec<Vec<String>>> {
    (0..all_task_cnt)
        .map(|i| {
            (0..conf.dir_cnt)
                .map(|j| {
                    (0..(conf.max_parallel * conf.file_per_dir_per_spwan))
                        .map(|k| format!("{}/dir_{}/file_{}_{}", conf.root_path, j, i, k))
                        .collect()
                })
                .collect()
        })
        .collect()
}

impl DirContentionTest {
    async fn init_help(&mut self) -> bool {
        let conf = self.conf.clone().unwrap();
        let dir_ps: Vec<_> = (0..conf.dir_cnt)
            .map(|i| format!("{}/dir_{}", conf.root_path, i))
            .collect();

        for dir_path in dir_ps {
            if self.unique_id == 0 {
                let re = create_dir(dir_path.as_str());
                if let Err(e) = re.await {
                    panic!("Error! mkdir {}, err:{:?}", dir_path, e);
                }
            } else {
                if let Err(e) = wait_until_exist(&dir_path).await {
                    panic!("Error! Try to find dir {}, err:{:?}", dir_path, e);
                }
            }
        }

        let all_file_ps = all_file_ps_gen(self.all_task_cnt, &conf);

        for p1 in all_file_ps[self.unique_id].clone() {
            for file_path in p1 {
                let re = fs::File::create(file_path.as_str());
                if let Err(e) = re.await {
                    panic!("Error! file create {}, err:{:?}", file_path, e);
                }
            }
        }

        let skew_dir_cnt = if conf.skew_dir_cnt == 0 {
            conf.dir_cnt
        } else {
            conf.skew_dir_cnt
        };

        let dir_ids: Vec<_> = (0..skew_dir_cnt).collect();
        let mut rng = StdRng::seed_from_u64(self.unique_id as u64);
        for i in 0..conf.max_parallel {
            self.file_ps.push(vec![]);
            for _ in 0..conf.op_per_spawn {
                self.file_ps[i].push(
                    all_file_ps
                        .choose(&mut rng)
                        .unwrap()
                        [*dir_ids.choose(&mut rng).unwrap()]
                        .choose(&mut rng)
                        .unwrap()
                        .clone(),
                );
            }
        }
        return true;
    }

    async fn run_help(&self) -> bool {
        let conf = self.conf.clone().unwrap();

        async_scoped::TokioScope::scope_and_block(|s| {
            for i in 0..conf.max_parallel {
                let file_ps = &self.file_ps[i];
                //let mut topo = topo.clone();
                s.spawn(async move {
                    for j in 0..conf.op_per_spawn {
                        //let re = fs::metadata(&file_ps[j]);
                        let re = modify_permissions(j, &file_ps[j]);
                        if let Err(e) = re.await {
                            println!("Error!:{:?}", e);
                            io::stdout().flush().unwrap();
                            panic!("Error! id:{}, err:{:?}", i, e);
                        }
                    }
                });
            }
        });

        return true;
    }
}


use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

async fn modify_permissions(i: usize, path: &str) -> std::io::Result<()> {
    if i % 2 == 0 {
        let read_only_permissions = Permissions::from_mode(0o555);  // 所有用户只能读，不能写
        set_permissions(path, read_only_permissions).await?;
    } else {
        let read_write_permissions = Permissions::from_mode(0o755);  // 所有者可读写，其他用户可读
        set_permissions(path, read_write_permissions).await?;
    }
    return Ok(());
}
