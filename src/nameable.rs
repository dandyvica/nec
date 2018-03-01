// Implemented by those structure from which we can get the name
pub trait Nameable {
    fn get_name(&self) -> &str;
}
