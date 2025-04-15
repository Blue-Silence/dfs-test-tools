
pub trait Test {
    fn name(&self) -> &'static str;
    fn set_config(&mut self, config: String, unique_id: String);
    fn init(&mut self) -> bool;
    fn run(&self) -> bool;
}

pub mod tests;