use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LocatorMap(HashMap<String, String>);

impl Deref for LocatorMap {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LocatorMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Locator {
    pub(crate) dimension: String,
    pub(crate) address: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Locators(Vec<Locator>);

impl Deref for Locators {
    type Target = Vec<Locator>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Locators {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<LocatorMap> for Locators {
    fn from(map: LocatorMap) -> Self {
        Self(
            map.0
                .into_iter()
                .map(|(k, v)| Locator {
                    dimension: k,
                    address: v,
                })
                .collect(),
        )
    }
}
