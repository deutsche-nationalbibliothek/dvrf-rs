mod document;
mod error;
mod level;
mod locator;
mod position;
mod record;

pub use document::Document;
pub use error::Error;
pub use level::Level;
pub use locator::{Locator, LocatorMap, Locators};
pub use position::Position;
pub use record::Record;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2, 1 + 1);
    }
}
