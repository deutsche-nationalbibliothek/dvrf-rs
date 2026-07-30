use std::convert::Infallible;

use serde::{Deserialize, Serialize};

use crate::{Error, Locator, LocatorMap, Locators};

/// The position of an error
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Position {
    Condensed(LocatorMap),
    Expanded(Locators),
}

impl Position {
    /// Returns an iterator over the error positions. The position
    /// information (dimension and address) is given as a Locator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::{Locator, Record};
    ///
    /// let record = Record::new().with_position("line", "23");
    ///
    /// let mut iter = record.position().unwrap().locators();
    /// assert_eq!(iter.next(), Some(Locator::new("line", "23")));
    /// assert_eq!(iter.next(), None);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn locators(&self) -> Box<dyn Iterator<Item = Locator> + '_> {
        match self {
            Self::Expanded(list) => Box::new(list.iter()),
            Self::Condensed(map) => Box::new(map.iter()),
        }
    }

    /// Returns true if the underlying representation is the condensed
    /// form.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::{LocatorMap, Position};
    ///
    /// let mut map = LocatorMap::new();
    /// map.insert("line", "3");
    ///
    /// let position = Position::from(map);
    /// assert!(position.is_condensed());
    /// # assert!(!position.is_expanded());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn is_condensed(&self) -> bool {
        matches!(self, Self::Condensed(_))
    }

    /// Returns true if the underlying representation is the expanded
    /// form.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::{Locator, Locators, Position};
    ///
    /// let mut locators = Locators::new();
    /// locators.push(Locator::new("line", "3"));
    ///
    /// let position = Position::from(locators);
    /// assert!(position.is_expanded());
    /// # assert!(!position.is_condensed());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn is_expanded(&self) -> bool {
        matches!(self, Self::Expanded(_))
    }

    /// Transforms the position into the extended variant.
    ///
    /// # Note
    ///
    /// The transformation must always be possible, since a [LocatorMap]
    /// can be easily mapped to [Locators]. This function returns the
    /// same position if it is already in the extended format.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::{LocatorMap, Position};
    ///
    /// let mut map = LocatorMap::new();
    /// map.insert("line", "3");
    ///
    /// let position = Position::from(map);
    /// assert!(position.is_condensed());
    ///
    /// let position = position.try_expand()?;
    /// assert!(position.is_expanded());
    ///
    /// let position = position.try_expand()?;
    /// assert!(position.is_expanded());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn try_expand(self) -> Result<Self, Infallible> {
        Ok(match self {
            Self::Condensed(map) => Self::Expanded(map.into()),
            Self::Expanded(_) => self,
        })
    }

    /// Try to transforms the position into the condensed form.
    ///
    /// # Note
    ///
    /// The transformation must always be possible, since a [LocatorMap]
    /// can be easily mapped to [Locators]. This function returns the
    /// same position if it is already in the extended format.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::{LocatorMap, Position};
    ///
    /// let mut map = LocatorMap::new();
    /// map.insert("line", "3");
    ///
    /// let position = Position::from(map);
    /// assert!(position.is_condensed());
    ///
    /// let position = position.try_expand()?;
    /// assert!(position.is_expanded());
    ///
    /// let position = position.try_expand()?;
    /// assert!(position.is_expanded());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn try_condense(self) -> Result<Self, Error> {
        match self {
            Self::Condensed(_) => Ok(self),
            Self::Expanded(locators) => {
                Ok(Self::Condensed(LocatorMap::try_from(locators)?))
            }
        }
    }
}

impl From<LocatorMap> for Position {
    fn from(map: LocatorMap) -> Self {
        Self::Condensed(map)
    }
}

impl From<Locators> for Position {
    fn from(locators: Locators) -> Self {
        Self::Expanded(locators)
    }
}
