
pub trait Test {
    fn name(&self) -> &'static str;
    fn set_config(&mut self, config: String, unique_id: usize, all_task_cnt: usize);
    fn init(&mut self) -> bool;
    fn run(&self) -> bool;
}

pub mod tests;