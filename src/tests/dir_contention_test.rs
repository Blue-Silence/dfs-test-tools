use serde::Deserialize;
use tokio::fs::{self, create_dir};
use std::io::{self, Write};

use crate::Test;

#[derive(Deserialize, Debug, Clone)]
struct TestConfig {
    pub thread: usize,
    pub max_parallel: usize,
    pub max_iter: usize,
    pub root_path: String,
    pub skew_dir_cnt: usize,
    pub seed: u64,
}

pub struct DirContentionTest {
    conf: Option<TestConfig>,
    unique_id: String,

    //Then starts the unique part for the test.
    file_ps: Vec<Vec<String>>,
}

impl DirContentionTest {
    pub fn new() -> Self {
        DirContentionTest {
            conf: None,
            unique_id: "".to_string(),
            file_ps: vec![],
        }
    }
}

impl Test for DirContentionTest {
    fn name(&self) -> &'static str {
        "Dir Contention Test"
    }

    fn set_config(&mut self, config: String, unique_id: String) {
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

impl DirContentionTest {
    async fn init_help(&mut self) -> bool {
        let conf = self.conf.clone().unwrap();
        let mut file_ps = vec![];

        for i in 0..conf.max_parallel {
            let dir_path = format!("{}/dir_{}_{}", conf.root_path, self.unique_id, i);
            let re = create_dir(dir_path.as_str());
            if let Err(e) = re.await {
                panic!("Error! mkdir {}, err:{:?}", dir_path, e);
            }

            file_ps.push(vec![]);
            for j in 0..conf.max_iter {
                let file_path = format!("{}/file_{}", dir_path, j);
                let re = fs::File::create(file_path.as_str());
                if let Err(e) = re.await {
                    panic!("Error! file create {}, err:{:?}", file_path, e);
                }
                file_ps[i].push(file_path);
            }
        }

        let choosen_dir_id = if conf.skew_dir_cnt == 0 {
            (0..conf.max_parallel).collect()
        } else {
            get_random_numbers_in_range(0, conf.max_parallel - 1, conf.skew_dir_cnt, conf.seed)
        };

        for k in 0..conf.max_parallel {
            self.file_ps
                .push(file_ps[choosen_dir_id[k & choosen_dir_id.len()]].to_owned());
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
                    for j in 0..conf.max_iter {
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

fn get_random_numbers_in_range(min: usize, max: usize, n: usize, seed: u64) -> Vec<usize> {
    let mut rng = StdRng::seed_from_u64(seed);
    let range: Vec<usize> = (min..=max).collect();
    let mut selected_numbers = Vec::with_capacity(n);
    range
        .choose_multiple(&mut rng, n)
        .for_each(|&num| selected_numbers.push(num));
    selected_numbers
}
