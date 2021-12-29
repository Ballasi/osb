/// Trait defining `Event`s
pub trait Event {
    fn to_line(&self) -> String;
    fn set_depth(&mut self, depth: usize);
    fn get_start_time(&self) -> i32;
    fn get_end_time(&self) -> i32;
}
