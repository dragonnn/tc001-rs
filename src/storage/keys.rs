use alloc::string::String;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Key {
    Wifi(u8),
}
