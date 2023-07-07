mod courses;
pub use courses::*;

mod modules;
pub use modules::*;

mod challenges;
pub use challenges::*;

mod status;
pub use status::*;

mod web;
pub use web::*;

pub struct Kits {}

impl Kits {
    pub fn init() {}
}
