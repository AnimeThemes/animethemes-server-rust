pub mod content;
pub mod list;

pub trait LocalizedEnum {
    fn localize(&self) -> &str;
}
