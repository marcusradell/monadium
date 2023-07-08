use super::ChallengesKit;
use crate::io::repo::Repo;

pub trait KitRouter {
    fn router(&self) -> axum::Router;
}

pub struct Kits {
    pub challenges: ChallengesKit,
}

impl Kits {
    pub fn new(repo: &Repo) -> Self {
        Self {
            challenges: ChallengesKit::new(repo.clone()),
        }
    }
}
