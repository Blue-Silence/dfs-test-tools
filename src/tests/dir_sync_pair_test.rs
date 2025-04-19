use serde::Deserialize;
use std::io::{self, Write};

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use crate::{Client, ClientGen, FSClient, Test};

#[derive(Deserialize, Debug, Clone)]
struct TestConfig {
    pub thread: usize,
    pub max_parallel: usize,

    pub root_path: String,
    pub file_per_spawn: usize,
    pub op_per_spawn: usize,

    pub enable_mix: bool,
    pub set_permission_ratio: usize,
}

pub struct DirSyncPairTest {
    conf: Option<TestConfig>,
    unique_id: usize,
    all_task_cnt: usize,

    //Then starts the unique part for the test.
    clients: Vec<Client>,
    file_ps: Vec<Vec<String>>,
    dir_ps: Vec<Vec<String>>,
}

impl DirSyncPairTest {
    pub fn new() -> Self {
        DirSyncPairTest {
            conf: None,
            unique_id: 0,
            all_task_cnt: 0,
            clients: vec![],
            file_ps: vec![],
            dir_ps: vec![],
        }
    }
}

impl Test for DirSyncPairTest {
    fn name(&self) -> &'static str {
        "Dir Contention Test"
    }

    fn set_config(&mut self, config: String, unique_id: usize, all_task_cnt: usize) {
        self.conf = Some(toml::from_str(&config).unwrap());
        self.unique_id = unique_id;
        self.all_task_cnt = all_task_cnt;
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

fn all_file_ps_gen(unique_id: usize, conf: &TestConfig) -> Vec<Vec<String>> {
    (0..conf.max_parallel)
        .map(|j| {
            (0..conf.file_per_spawn)
                .map(|k| format!("{}/dir_{}_{}/file_{}", conf.root_path, unique_id, j, k))
                .collect()
        })
        .collect()
}

fn all_dir_ps_gen(all_task_cnt: usize, conf: &TestConfig) -> Vec<Vec<String>> {
    (0..all_task_cnt)
        .map(|i| {
            (0..conf.max_parallel)
                .map(|j| format!("{}/dir_{}_{}", conf.root_path, i, j))
                .collect()
        })
        .collect()
}

impl DirSyncPairTest {
    async fn init_help(&mut self, mut c_gen: ClientGen) -> bool {
        let conf = self.conf.clone().unwrap();

        let all_dir_ps = all_dir_ps_gen(self.all_task_cnt, &conf);

        for _ in 0..conf.max_parallel {
            self.clients.push(c_gen.new_client());
        }

        for dir_path in all_dir_ps[self.unique_id].as_slice() {
            let re = self.clients[0].create_dir(dir_path.as_str());
            if let Err(e) = re.await {
                panic!("Error! mkdir {}, err:{:?}", dir_path, e);
            }
        }

        let all_file_ps = all_file_ps_gen(self.unique_id, &conf);

        for p1 in all_file_ps.clone() {
            for file_path in p1 {
                let re = self.clients[0].file_create(file_path.as_str());
                if let Err(e) = re.await {
                    panic!("Error! file create {}, err:{:?}", file_path, e);
                }
            }
        }

        let mut rng = StdRng::seed_from_u64(self.unique_id as u64);

        for i in 0..conf.max_parallel {
            self.file_ps.push(vec![]);
            self.dir_ps.push(vec![]);

            for j in 0..conf.op_per_spawn {
                self.file_ps[i].push(all_file_ps[i].choose(&mut rng).unwrap().clone());
                if j % conf.set_permission_ratio == 1 {
                    let p = if conf.enable_mix {
                        all_dir_ps[(self.unique_id + 1) % all_dir_ps.len()].choose(&mut rng).unwrap()
                    } else {
                        all_dir_ps[self.unique_id].choose(&mut rng).unwrap()
                    };
                    self.dir_ps[i].push(p.clone());
                }
            }
        }

        return true;
    }

    async fn run_help(&mut self) -> bool {
        let conf = self.conf.clone().unwrap();

        async_scoped::TokioScope::scope_and_block(|s| {
            for (i, client) in self.clients.iter_mut().enumerate() {
                let file_ps = &self.file_ps[i];
                let client = client;
                let dir_ps = &self.dir_ps;
                //let mut topo = topo.clone();
                s.spawn(async move {
                    for j in 0..conf.op_per_spawn {
                        let re = client.file_stat(&file_ps[j]);
                        if let Err(e) = re.await {
                            println!("Error!:{:?}", e);
                            io::stdout().flush().unwrap();
                            panic!("Error! id:{}, err:{:?}", i, e);
                        }

                        if j % conf.set_permission_ratio == 1 {
                            let k = j / conf.set_permission_ratio;
                            let p = &dir_ps[i][k];
                            let re = modify_permissions(client, k, p);
                            if let Err(e) = re.await {
                                println!("Error!:{:?}", e);
                                io::stdout().flush().unwrap();
                                panic!("Error! set permission: {}, err:{:?}", p, e);
                            }
                        }
                    }
                });
            }
        });

        return true;
    }
}

async fn modify_permissions(client: &mut Client, i: usize, path: &str) -> Result<(), String> {
    if i % 2 == 0 {
        return client.change_permission(path, 0o555).await;
    } else {
        return client.change_permission(path, 0o755).await;
    }
}