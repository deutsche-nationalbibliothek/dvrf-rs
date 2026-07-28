use serde::{Deserialize, Serialize};

use crate::{LocatorMap, Locators};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Position {
    Condensed(LocatorMap),
    Expanded(Locators),
}
