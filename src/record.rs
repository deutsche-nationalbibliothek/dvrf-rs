use serde::{Deserialize, Serialize};

use crate::{Level, Position};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<Level>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Position>,
}
