use serde::{Deserialize, Serialize};

use crate::{Level, Locator, LocatorMap, Position};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    types: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<Level>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Position>,
}

impl Record {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_message<S>(mut self, message: S) -> Self
    where
        S: AsRef<str>,
    {
        let message = message.as_ref();
        if !message.is_empty() {
            self.message = Some(message.to_string());
        }
        self
    }

    pub fn with_type<S>(mut self, r#type: S) -> Self
    where
        S: AsRef<str>,
    {
        self.types.push(r#type.as_ref().to_string());
        self
    }

    pub fn with_level<L>(mut self, level: L) -> Self
    where
        L: Into<Level>,
    {
        self.level = Some(level.into());
        self
    }

    pub fn with_position<S>(mut self, dimension: S, address: S) -> Self
    where
        S: AsRef<str>,
    {
        let dimension = dimension.as_ref().to_string();
        let address = address.as_ref().to_string();

        if self.position.is_none() {
            self.position =
                Some(Position::Condensed(LocatorMap::default()));
        }

        match self.position {
            Some(Position::Condensed(ref mut map)) => {
                map.insert(dimension, address);
            }
            Some(Position::Expanded(ref mut locators)) => {
                locators.push(Locator { dimension, address });
            }
            _ => unreachable!(),
        }

        self
    }
}
