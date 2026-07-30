use std::collections::HashMap;
use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

/// A single error position
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Locator {
    pub(crate) dimension: String,
    pub(crate) address: String,
}

impl Locator {
    /// Creates a new locator
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Locator;
    ///
    /// let locator = Locator::new("foo", "bar");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn new<S>(dimension: S, address: S) -> Self
    where
        S: ToString,
    {
        Self {
            dimension: dimension.to_string(),
            address: address.to_string(),
        }
    }

    /// Returns the dimension of the locator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Locator;
    ///
    /// let locator = Locator::new("foo", "bar");
    /// assert_eq!(locator.dimension(), "foo");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn dimension(&self) -> &str {
        &self.dimension
    }

    /// Returns the address of the locator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Locator;
    ///
    /// let locator = Locator::new("foo", "bar");
    /// assert_eq!(locator.address(), "bar");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn address(&self) -> &str {
        &self.address
    }
}

/// A mapping from dimensions to addresses (condensed position variant)
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LocatorMap(HashMap<String, String>);

impl LocatorMap {
    /// Creates a new locator map.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::LocatorMap;
    ///
    /// let map = LocatorMap::new();
    /// assert_eq!(map.iter().count(), 0);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a new error position in the location map.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::LocatorMap;
    ///
    /// let mut map = LocatorMap::new();
    /// map.insert("line", "7");
    ///
    /// assert!(map.contains_dimension("line"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn insert<S: ToString>(&mut self, dimension: S, address: S) {
        self.0.insert(dimension.to_string(), address.to_string());
    }

    /// Whether the location map contains the given dimension or not.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::LocatorMap;
    ///
    /// let mut map = LocatorMap::new();
    /// map.insert("line", "7");
    /// map.insert("char", "42");
    ///
    /// assert!(map.contains_dimension("line"));
    /// assert!(map.contains_dimension("line"));
    /// assert!(!map.contains_dimension("foo"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn contains_dimension<S>(&self, dimension: S) -> bool
    where
        S: AsRef<str>,
    {
        self.0.contains_key(dimension.as_ref())
    }

    /// Returns an iterator over the location map entries. The map
    /// entries are transformed into [Locator] types.
    ///
    /// # Note
    ///
    /// Please note that no guarantees are made regarding the order in
    /// which the locators are returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::LocatorMap;
    ///
    /// let mut map = LocatorMap::new();
    /// map.insert("line", "7");
    /// map.insert("char", "42");
    ///
    /// for locator in map.iter() {
    ///     match locator.dimension() {
    ///         "line" => assert_eq!(locator.address(), "7"),
    ///         "char" => assert_eq!(locator.address(), "42"),
    ///         _ => unreachable!(),
    ///     }
    /// }
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = Locator> {
        self.0.iter().map(|(k, v)| Locator {
            dimension: k.into(),
            address: v.into(),
        })
    }
}

/// A list of [Locators](Locator) (expanded position variant)
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Locators(Vec<Locator>);

impl Locators {
    /// Creates a new locators list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Locators;
    ///
    /// let locators = Locators::new();
    /// assert!(locators.is_empty());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the locatores list contains the given dimension or not.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Locators;
    ///
    /// let mut locators = Locators::new();
    /// locators.insert("line", "7");
    /// locators.insert("char", "42");
    ///
    /// assert!(locators.contains_dimension("line"));
    /// assert!(locators.contains_dimension("char"));
    /// assert!(!locators.contains_dimension("foo"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn contains_dimension<S>(&self, dimension: S) -> bool
    where
        S: AsRef<str>,
    {
        self.0
            .iter()
            .any(|locator| locator.dimension() == dimension.as_ref())
    }

    /// Inserts a new error position
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Locators;
    ///
    /// let mut locators = Locators::new();
    /// locators.insert("line", "7");
    ///
    /// assert!(locators.contains_dimension("line"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn insert<S: ToString>(&mut self, dimension: S, address: S) {
        self.0.push(Locator {
            dimension: dimension.to_string(),
            address: address.to_string(),
        });
    }

    /// Returns an iterator over locators list entries.
    ///
    /// # Note
    ///
    /// Please note that no guarantees are made regarding the order in
    /// which the locators are returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Locators;
    ///
    /// let mut locators = Locators::new();
    /// locators.insert("line", "7");
    /// locators.insert("char", "42");
    ///
    /// for locator in locators.iter() {
    ///     match locator.dimension() {
    ///         "line" => assert_eq!(locator.address(), "7"),
    ///         "char" => assert_eq!(locator.address(), "42"),
    ///         _ => unreachable!(),
    ///     }
    /// }
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = Locator> {
        self.0.clone().into_iter()
    }
}

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

/// An error that can occur when transforming a position into the
/// condensed variant.
#[derive(Debug, PartialEq, Eq)]
pub struct CondenseError(String);

impl std::error::Error for CondenseError {}

impl Display for CondenseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<Locators> for LocatorMap {
    type Error = CondenseError;

    fn try_from(locators: Locators) -> Result<Self, Self::Error> {
        let mut map = Self::new();

        for Locator { dimension, address } in locators.iter() {
            if map.contains_dimension(&dimension) {
                return Err(CondenseError(format!(
                    "found more than one locator for dimension {dimension}."
                )));
            }

            map.insert(dimension.to_owned(), address.to_owned());
        }

        Ok(map)
    }
}
