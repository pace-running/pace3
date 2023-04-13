use crate::models::runner::{NewRunner, Runner};
#[cfg(test)]
use mockall::automock;

pub type RunnerId = i32;

#[cfg_attr(test, automock)]
pub trait RunnerRepository {
    fn insert_runner<'a>(&self, new_runner: NewRunner<'a>) -> Runner;
    fn find_runner_by_id(&self, id: RunnerId) -> Option<Runner>;
}
