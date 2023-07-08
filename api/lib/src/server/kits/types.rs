use super::{ChallengesKit, StatusKit, WebKit};
use crate::io::repo::Repo;

pub trait KitRouter {
    fn router(&self) -> axum::Router;
}

pub struct Kits {
    pub status: StatusKit,
    pub web: WebKit,
    pub challenges: ChallengesKit,
}

impl Kits {
    pub fn new(repo: &Repo) -> Self {
        Self {
            status: StatusKit::new(),
            web: WebKit::new(),
            challenges: ChallengesKit::new(repo.clone()),
        }
    }
}
