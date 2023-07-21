use super::{ChallengesKit, StatusKit};
use crate::io::Repo;

pub trait KitRouter {
    fn router(&self) -> axum::Router;
}

pub struct Kits {
    pub status: StatusKit,
    pub challenges: ChallengesKit,
}

impl Kits {
    pub fn new(repo: &Repo) -> Self {
        Self {
            status: StatusKit::new(),
            challenges: ChallengesKit::new(repo.clone()),
        }
    }
}
