use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum HorseBelong {
    #[default]
    Left = 0,
    Banei = 21,
}
