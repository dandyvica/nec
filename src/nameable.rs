//! Trait that a structure must implement to associate a name with it.
pub trait Nameable {
    fn get_name(&self) -> &str;
}
