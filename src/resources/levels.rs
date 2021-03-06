use crate::{
    config::LevelConfig,
};

/// Level resource and info.
#[derive(Debug)]
pub struct LevelResource {
    pub current_level: Option<usize>,
    pub levels: Vec<LevelConfig>,
    pub finished: bool
}

impl Default for LevelResource {
    fn default() -> Self {
        Self {
            current_level: None,
            levels: vec![],
            finished: false,
        }
    }
}
