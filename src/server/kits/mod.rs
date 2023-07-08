mod courses;
use axum::Router;
pub use courses::*;

mod modules;
pub use modules::*;

mod challenges;
pub use challenges::*;

mod status;
pub use status::*;

mod web;
pub use web::*;

use super::io::repo::Repo;

pub trait Kit {
    fn router(&self) -> Router;
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
