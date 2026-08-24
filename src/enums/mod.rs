pub mod content;
pub mod document;
pub mod features;
pub mod list;

pub trait LocalizedEnum {
    fn localize(&self) -> &str;
}
