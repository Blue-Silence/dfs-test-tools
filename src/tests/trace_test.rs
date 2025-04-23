use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    trace::{
        trace_exec::TraceEngine,
        trace_parse::{parse_trace, TraceEvent},
    },
    ClientGen, Test,
};

#[derive(Deserialize, Debug, Clone)]
struct TestConfig {
    pub trace_path: String,
    pub thread: usize,
    pub max_parallel: usize,

    pub root_path: String,
    pub iter_per_spawn: usize,
}

pub struct TraceTest {
    conf: Option<TestConfig>,
    unique_id: usize,
    all_task_cnt: usize,
    dir_out: String,

    //Then starts the unique part for the test.
    engines: Vec<Arc<Mutex<TraceEngine>>>,
    events: Vec<TraceEvent>,
}

impl TraceTest {
    pub fn new() -> Self {
        TraceTest {
            conf: None,
            unique_id: 0,
            all_task_cnt: 0,
            engines: vec![],
            events: vec![],
            dir_out: "".to_string(),
        }
    }
}

impl Test for TraceTest {
    fn name(&self) -> &'static str {
        "Dir Contention Test"
    }

    fn set_config(&mut self, config: String, unique_id: usize, all_task_cnt: usize, dir_out: String) {
        self.conf = Some(toml::from_str(&config).unwrap());
        self.unique_id = unique_id;
        self.all_task_cnt = all_task_cnt;
        self.dir_out = dir_out;
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

impl TraceTest {
    async fn init_help(&mut self, mut c_gen: ClientGen) -> bool {
        let conf = self.conf.clone().unwrap();

        let inp = std::fs::read_to_string(conf.trace_path.as_str())
            .expect("Should have been able to read the file");
        self.events = parse_trace(inp.as_str()).unwrap();

        for _ in 0..conf.max_parallel {
            self.engines
                .push(std::sync::Arc::new(Mutex::new(TraceEngine::new(
                    c_gen.new_client(),
                ))));
        }
        return true;
    }

    async fn run_help(&mut self) -> bool {
        let conf = self.conf.clone().unwrap();

        async_scoped::TokioScope::scope_and_block(|s| {
            let events = &self.events;
            let unique_id = &self.unique_id;
            let root_path = conf.root_path.as_str();
            for i in 0..conf.max_parallel {
                let engine = self.engines[i].clone();
                let root_path = root_path;
                s.spawn(async move {
                    for j in 0..conf.iter_per_spawn {
                        for event in events {
                            //println!("Execing: {:?}", event);
                            engine
                                .lock()
                                .await
                                .exec(
                                    event.clone(),
                                    format!("_{}_{}_{}", unique_id, i, j).as_str(),
                                    root_path,
                                )
                                .await;
                        }
                    }
                });
            }
        });

        return true;
    }
}
