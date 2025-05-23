use serde::Deserialize;
use std::{fs::File, iter::zip, sync::Arc, time::SystemTime};
use std::io::{self, Write};

use crate::{
    test_log::TestLog, trace::{
        trace_exec::TraceEngine,
        trace_parse::{parse_trace, TraceEvent},
    }, ClientGen, Test
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
    engines: Vec<TraceEngine>,//Vec<Arc<Mutex<TraceEngine>>>,
    logs: Vec<TestLog>,
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
            logs: vec![],
        }
    }
}

impl Test for TraceTest {
    fn name(&self) -> &'static str {
        "Dir Contention Test"
    }

    fn set_config(&mut self, config: String, unique_id: usize, all_task_cnt: usize, dir_out: String, reuse_init: bool) {
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
            /* 
            self.engines
                .push(std::sync::Arc::new(Mutex::new(TraceEngine::new(
                    c_gen.new_client(),
                ))));
            */
            self.engines.push(TraceEngine::new(c_gen.new_client()));
            self.logs.push(TestLog::new(conf.iter_per_spawn * self.events.len()));
        }
        return true;
    }

    async fn run_help(&mut self) -> bool {
        let conf = self.conf.clone().unwrap();

        async_scoped::TokioScope::scope_and_block(|s| {
            let events = &self.events;
            let unique_id = &self.unique_id;
            let root_path = conf.root_path.as_str();

            //for (i, engine) in self.engines.iter_mut().enumerate() {

            //}

            // for i in 0..conf.max_parallel {
            let mut z = zip(self.engines.iter_mut(), self.logs.iter_mut());
            //for (i, engine) in self.engines.iter_mut().enumerate() {
            for (i, (engine, log)) in z.enumerate() {
                //let engine = self.engines[i].clone();
                let root_path = root_path;
                s.spawn(async move {
                    for j in 0..conf.iter_per_spawn {
                        for event in events {
                            //println!("Execing: {:?}", event);
                            let t1 = SystemTime::now();
                            engine
                                .exec(
                                    event.clone(),
                                    format!("_{}_{}_{}", unique_id, i, j).as_str(),
                                    root_path,
                                )
                                .await;
                            let t2 = SystemTime::now();
                            log.push(event2id(&event), t2.duration_since(t1).unwrap().as_micros() as usize);
                        }
                    }
                });
            }
        });

        let mut out_f = File::create(format!("{}/{}.log", self.dir_out, self.unique_id)).unwrap();
        for log in self.logs.iter_mut() {
            loop {
                let s = log.pop();
                if let None = s {
                    break;
                }
                write!(out_f, "{}\n", s.unwrap()).unwrap();
            }
        }

        return true;
    }
}


fn event2id(t: &TraceEvent) -> &'static str {
    match t {
        TraceEvent::Create { path, address } => "1",
        TraceEvent::Mkdir { path } => "2",
        TraceEvent::Close { address } => "3",
        TraceEvent::Open { path, address } => "4",
        TraceEvent::Delete { path } => "5",
        TraceEvent::FileStat { path } => "6",
        TraceEvent::DirStat { path } => "7",
    }
}