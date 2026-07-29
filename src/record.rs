use serde::{Deserialize, Serialize};

use crate::{Level, Position};

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

    pub fn with_message<S>(&mut self, message: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.message = Some(message.as_ref().to_string());
        self
    }

    pub fn with_type<S>(&mut self, r#type: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.types.push(r#type.as_ref().to_string());
        self
    }

    pub fn with_level<L>(&mut self, level: L) -> &mut Self
    where
        L: Into<Level>,
    {
        self.level = Some(level.into());
        self
    }
}
