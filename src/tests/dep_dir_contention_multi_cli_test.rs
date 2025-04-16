use serde::Deserialize;
use tokio::fs::{self, create_dir};
use std::io::{self, Write};

use crate::Test;

#[derive(Deserialize, Debug, Clone)]
struct TestConfig {
    pub thread: usize,
    pub max_parallel: usize,
    pub dir_size: usize,
    pub stat_cnt_per_spwan: usize,
    pub target_tag: Vec<usize>,

    pub root_path: String,
    pub seed: u64,
}

pub struct DirContentionMultiCliTest {
    conf: Option<TestConfig>,
    unique_id: usize,

    //Then starts the unique part for the test.
    file_ps: Vec<Vec<String>>,
}

impl DirContentionMultiCliTest {
    pub fn new() -> Self {
        DirContentionMultiCliTest {
            conf: None,
            unique_id: 0,
            file_ps: vec![],
        }
    }
}

impl Test for DirContentionMultiCliTest {
    fn name(&self) -> &'static str {
        "Dir Contention Test"
    }

    fn set_config(&mut self, config: String, unique_id: usize, _: usize) {
        self.conf = Some(toml::from_str(&config).unwrap());
        self.unique_id = unique_id;
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
        .block_on(async {
            self.init_help().await
        })
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
        .block_on(async {
            self.run_help().await
        })
    }
}

impl DirContentionMultiCliTest {
    async fn init_help(&mut self) -> bool {
        let conf = self.conf.clone().unwrap();
        //let mut file_ps: Vec<Vec<String>> = vec![];

        for dir_id in 0..conf.max_parallel {
            let root_path = conf.root_path.as_str();
            let unique_id = self.unique_id;
            let dir_path = dir_path_gen(root_path, unique_id, dir_id);
            let re = create_dir(dir_path.as_str());
            if let Err(e) = re.await {
                panic!("Error! mkdir {}, err:{:?}", dir_path, e);
            }

            //file_ps.push(vec![]);

            for file_path in file_path_gen(root_path, unique_id, dir_id, conf.dir_size) {
                let re = fs::File::create(file_path.as_str());
                if let Err(e) = re.await {
                    panic!("Error! file create {}, err:{:?}", file_path, e);
                }
            }
        }

        let mut rng = StdRng::seed_from_u64(conf.seed);

        let choosen_unique_id = if conf.target_tag.len() == 0 {
            self.unique_id.clone()
        } else {
            conf.target_tag.choose(&mut rng).unwrap().clone()
        };

        for k in 0..conf.max_parallel {
            self.file_ps
                .push(file_path_gen(conf.root_path.as_str(), choosen_unique_id, k, conf.dir_size).choose_multiple(&mut rng, conf.stat_cnt_per_spwan).map(|s| s.clone()).collect());
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
                    for j in 0..conf.stat_cnt_per_spwan {
                        let re = fs::metadata(&file_ps[j]);
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

use rand::rngs::StdRng;
use rand::seq::SliceRandom; // 用于随机选择切片中的元素
use rand::SeedableRng;


fn dir_path_gen(root_path: &str, unique_id: usize, dir_id: usize) -> String {
    format!("{}/dir_{}_{}", root_path, unique_id, dir_id)
}

fn file_path_gen(root_path: &str, unique_id: usize, dir_id: usize, dir_size: usize) -> Vec<String> {
    let mut re = vec![];
    let dir_path = dir_path_gen(root_path, unique_id, dir_id);

    for j in 0..dir_size {
        let file_path = format!("{}/file_{}", dir_path, j);
        re.push(file_path);
    }

    return re;
}