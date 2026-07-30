use serde::{Deserialize, Serialize};

use crate::{Level, Locator, LocatorMap, Position};

/// Representation of an error (record)
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
    /// Creates a new empty record
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Record;
    ///
    /// let record = Record::new();
    /// assert!(record.is_empty());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if the record is empty (has no information)
    ///
    /// Note that, according to DVRF 1.0, an error neither requires a
    /// message, a type, a level, nor a position. These records
    /// containing no information are considered as “empty”.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Record;
    ///
    /// let record = Record::new().with_message("foo");
    /// assert!(!record.is_empty());
    ///
    /// let record = Record::new();
    /// assert!(record.is_empty());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_empty(&self) -> bool {
        self.message.is_none()
            && self.position.is_none()
            && self.types.is_empty()
            && self.level.is_none()
    }

    /// Returns the message of the record
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Record;
    ///
    /// let record = Record::new().with_message("foo");
    /// assert_eq!(record.message().unwrap(), "foo");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    /// Sets the message of the record
    ///
    /// Note that an empty string is internally mapped to `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Record;
    ///
    /// let record = Record::new().with_message("foo");
    /// assert_eq!(record.message().unwrap(), "foo");
    ///
    /// let record = Record::new().with_message("");
    /// assert!(record.message().is_none());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with_message<S>(mut self, message: S) -> Self
    where
        S: AsRef<str>,
    {
        let message = message.as_ref();
        if !message.is_empty() {
            self.message = Some(message.to_string());
        } else {
            self.message = None;
        }

        self
    }

    /// Returns iterator over the error types.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Record;
    ///
    /// let record = Record::new()
    ///     .with_type("T001")
    ///     .with_type("T002")
    ///     .with_type("T003");
    ///
    /// let mut iter = record.types();
    /// assert_eq!(iter.next().unwrap(), "T001");
    /// assert_eq!(iter.next().unwrap(), "T002");
    /// assert_eq!(iter.next().unwrap(), "T003");
    /// assert!(iter.next().is_none());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn types(&self) -> impl Iterator<Item = &String> {
        self.types.iter()
    }

    /// Adds a new type to the record.
    ///
    /// # Note
    ///
    /// An empty string won't be added to the list of types.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::Record;
    ///
    /// let record = Record::new().with_type("T001");
    /// let types: Vec<_> = record.types().collect();
    /// assert_eq!(types, vec!["T001"]);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with_type<S>(mut self, r#type: S) -> Self
    where
        S: AsRef<str>,
    {
        let value = r#type.as_ref();

        if !value.is_empty() {
            self.types.push(value.to_string());
        }

        self
    }

    /// Returns the level of the record.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::{Level, Record};
    ///
    /// let record = Record::new().with_level(Level::Warning);
    /// assert_eq!(*record.level().unwrap(), Level::Warning);
    ///
    /// # assert!(Record::new().level().is_none());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn level(&self) -> Option<&Level> {
        self.level.as_ref()
    }

    /// Sets the error level of the record.
    /// # Example
    ///
    /// ```rust
    /// use dvrf::{Level, Record};
    ///
    /// let record = Record::new().with_level(Level::Error);
    /// assert_eq!(*record.level().unwrap(), Level::Error);
    ///
    /// let record = Record::new().with_level(Level::Warning);
    /// assert_eq!(*record.level().unwrap(), Level::Warning);
    ///
    /// let record = Record::new().with_level(Level::Info);
    /// assert_eq!(*record.level().unwrap(), Level::Info);
    ///
    /// # assert!(Record::new().level().is_none());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with_level<L>(mut self, level: L) -> Self
    where
        L: Into<Level>,
    {
        self.level = Some(level.into());
        self
    }

    /// Returns the position of the error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dvrf::{Document, Locator};
    ///
    /// let document = Document::from_path("tests/data/example1.json")?;
    /// let record = document.records().next().unwrap();
    /// let position = record.position().unwrap();
    ///
    /// for locator in position.locators() {
    ///     match locator.dimension() {
    ///         "jsonpointer" => assert_eq!(locator.address(), "/åå"),
    ///         "char" => assert_eq!(locator.address(), "7"),
    ///         "line" => assert_eq!(locator.address(), "1"),
    ///         _ => unreachable!(),
    ///     }
    /// }
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn position(&self) -> Option<&Position> {
        self.position.as_ref()
    }

    /// Add a new position to the record
    ///
    /// # Note
    ///
    /// This function respects the underlying position variant
    /// (condensed or full/expandend). By default the condensed
    /// variant is used.
    ///
    /// # Example
    /// ```rust
    /// use dvrf::{Locator, Record};
    ///
    /// let record = Record::new().with_position("line", "7");
    /// let position = record.position().unwrap();
    /// let locators = position.locators().collect::<Vec<_>>();
    /// assert_eq!(locators, vec![Locator::new("line", "7")]);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
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
