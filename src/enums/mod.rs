pub mod content;
pub mod document;
pub mod list;

pub trait LocalizedEnum {
    fn localize(&self) -> &str;
}
