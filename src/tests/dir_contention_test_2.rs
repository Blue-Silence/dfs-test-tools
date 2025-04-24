use serde::Deserialize;
use std::io::{self, Write};

use rand::{rngs::StdRng, seq::IndexedRandom};
use rand::SeedableRng;

use crate::{Client, ClientGen, FSClient, Test};

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
    dir_out: String,
    reuse_init: bool,

    //Then starts the unique part for the test.
    clients: Vec<Client>,
    file_ps: Vec<Vec<String>>,
}

impl DirContentionTest {
    pub fn new() -> Self {
        DirContentionTest {
            conf: None,
            unique_id: 0,
            all_task_cnt: 0,
            clients: vec![],
            file_ps: vec![],
            dir_out: "".to_string(),
            reuse_init: false,
        }
    }
}

impl Test for DirContentionTest {
    fn name(&self) -> &'static str {
        "Dir Contention Test"
    }

    fn set_config(&mut self, config: String, unique_id: usize, all_task_cnt: usize, dir_out: String, reuse_init: bool) {
        self.conf = Some(toml::from_str(&config).unwrap());
        self.unique_id = unique_id;
        self.all_task_cnt = all_task_cnt;
        self.dir_out = dir_out;
        self.reuse_init = reuse_init;
    }

    //#[tokio::main]
    fn init(&mut self, c_gen: ClientGen) -> bool {
        if let None = self.conf {
            return false;
        }
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(self.conf.clone().unwrap().thread)
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { self.init_help(c_gen).await })
    }

    fn run(&mut self) -> bool {
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

async fn wait_until_exist(client: &mut Client, p: &String) -> Result<(), String> {
    loop {
        match client.dir_try_exist(p).await {
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

fn my_dir_init_duty(all_task_cnt: usize, unique_id: usize, conf: &TestConfig) -> Vec<usize> {
    (0..conf.dir_cnt).filter(|i| i % all_task_cnt == unique_id).collect()
}

impl DirContentionTest {
    async fn init_help(&mut self, mut c_gen: ClientGen) -> bool {
        let conf = self.conf.clone().unwrap();
        let dir_ps: Vec<_> = (0..conf.dir_cnt)
            .map(|i| format!("{}/dir_{}", conf.root_path, i))
            .collect();

        for _ in 0..conf.max_parallel {
            self.clients.push(c_gen.new_client());
        }

        let duty_dir = my_dir_init_duty(self.all_task_cnt, self.unique_id, &conf);
        for i in duty_dir.iter() {
            if self.reuse_init {
                break;
            }
            let re = self.clients[0].create_dir(&dir_ps[*i]);
                if let Err(e) = re.await {
                    panic!("Error! mkdir {}, err:{:?}", dir_ps[*i], e);
                }
        }

        let all_file_ps = all_file_ps_gen(self.all_task_cnt, &conf);

        for dir_id in duty_dir.iter() {
            if self.reuse_init {
                break;
            }
            for p1 in all_file_ps.iter() {
                for file_path in p1[*dir_id].iter() {
                    let re = self.clients[0].file_create(file_path).await;
                    if let Err(e) = re {
                        panic!("Error! file create {}, err:{:?} uid:{} ids:{:?}", file_path, e, self.unique_id, duty_dir);
                    }
                    if let Err(e) = self.clients[0].close(re.unwrap()).await {
                        panic!("Error! file close after creation {}, err:{:?} uid:{} ids:{:?}", file_path, e, self.unique_id, duty_dir);
                    }
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

    async fn run_help(&mut self) -> bool {
        let conf = self.conf.clone().unwrap();

        async_scoped::TokioScope::scope_and_block(|s| {
            for (i, client) in self.clients.iter_mut().enumerate() {
                let file_ps = &self.file_ps[i];
                //let mut topo = topo.clone();
                s.spawn(async move {
                    for j in 0..conf.op_per_spawn {
                        //let re = fs::metadata(&file_ps[j]);
                        let re = file_modify_permissions(client, j, &file_ps[j]);
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

/* 
async fn dir_modify_permissions(client: &mut Client, i: usize, path: &String) -> Result<(), String> {
    if i % 2 == 0 {
        return client.dir_change_permission(path, 0o555).await;
    } else {
        return client.dir_change_permission(path, 0o755).await;
    }
}
*/

async fn file_modify_permissions(client: &mut Client, i: usize, path: &String) -> Result<(), String> {
    if i % 2 == 0 {
        return client.file_change_permission(path, 0o555).await;
    } else {
        return client.file_change_permission(path, 0o755).await;
    }
}