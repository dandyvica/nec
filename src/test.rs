use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

pub trait Inserter {
    fn f(&self, i: usize) -> Self; 
}
impl Inserter for usize {
    fn f(&self, i: usize) -> usize {
        i
    }
}
impl Inserter for Vec<usize> {
    fn f(&self, i: usize) -> Vec<usize> {
        vec![i]
    }
}

pub struct S<T,U> {
    list: Vec<T>,   
    hmap: HashMap<String, U>,
}

impl<T, U: Inserter> S<T,U> {
    /// Creates a new Container.
    ///
    /// # Arguments
    /// * `allow_dup` - True is items having the same name are allowed
    pub fn new() -> S<T> {
        // initialize all relevant members
        NamedObjectsContainer {
            list: Vec::new(),
            hmap: HashMap::new(),
        }        
    }

    /// Adds a Field structure to the end of the record.
    pub fn push<T>(&mut self) {
        let index = 2;
        let f = T.f();

        //self.hmap.inserter(name, index);
    }

}