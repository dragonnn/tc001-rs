use alloc::string::String;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Key<'a> {
    Wifi(&'a str),
    TransitionState,
    IndicatorsState,
}
