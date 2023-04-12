use crate::models::runner::{NewRunner, Runner};

pub type RunnerId = i32;

pub trait RunnerRepository {
    fn insert_runner(&self, new_runner: NewRunner) -> Runner;
    fn find_runner_by_id(&self, id: RunnerId) -> Option<Runner>;
}
