//! Represents a record by its name, description and length. Each record contains a list of contiguous fields
//! which hold values when reading a record-based file.
//!
//! # Examples
use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

/// Macro which builds a vector of Record data fields.
///
/// # Example
/// ```rust,ignore
/// // this builds a vector of record length if rec is a Record
/// # #[macro_use] use rbf::record;
/// # let rec = ::rbf::record::setup::set_up_by_length::<::rbf::record::AsciiMode>();
/// let v = vector_of!(rec, length);
/// ```
#[macro_export]
macro_rules! vector_of {
    ($rec:ident, $field: ident) => {{
        let v: Vec<_> = $rec.flist.iter().map(|f| f.$field.clone()).collect();
        v
    }};
}

pub trait Inserter {
    fn inserter(&mut self, name: &str, index: usize); 
}
impl Inserter for HashMap<String, usize> {
    fn inserter(&mut self, name: &str, index: usize) {
        self.insert(String::from(name), index);
    }
}
impl Inserter for HashMap<String, Vec<usize>> {
    fn inserter(&mut self, name: &str, index: usize) {
        self.insert(String::from(name), vec![index]);
    }
}

#[derive(Clone)]
pub struct NamedObjectsContainer<T, U> {
    /// List of T structs
    pub list: Vec<T>,
    /// Hashmap keeping track of the name vs. index of the structure in the previous list
    pub hmap: HashMap<String, U>,
}

impl<T,U: Inserter> NamedObjectsContainer<T,U> {
    /// Creates a new Container.
    ///
    /// # Arguments
    /// * `allow_dup` - True is items having the same name are allowed
    pub fn new() -> NamedObjectsContainer<T,U> {
        // initialize all relevant members
        NamedObjectsContainer {
            list: Vec::new(),
            hmap: HashMap::new(),
        }        
    }

    /// Adds a Field structure to the end of the record.
    pub fn push(&mut self, name: &str, item: T) {
        // finally, save Field struct
        self.list.push(item);

        // item is pushed at the end of the list. So its index is length of the list - 1
        let index = self.list.len() - 1;

        // if the item's name is already in the list, just add the index to the vector holding the list
        // of indexes for the same name
        /*
        if self.hmap.contains_key(name) {
            self.hmap.get_mut(name).unwrap().push(index);
        }
        // if not, create list of indices and add new index to it
        else {
            self.hmap.insert(String::from(name), vec![index]);
        }*/

        self.hmap.inserter(name, index);
    }

    /// Returns the item corresponding to index
    ///
    /// # Arguments
    /// * `index` - Item index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.list.get(index)
    }

    /// Returns the mutable item corresponding to index
    ///
    /// # Arguments
    /// * `index` - Item index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.list.get_mut(index)
    }

    /// Tests whether the container contains an item by giving its name.
    ///
    /// # Arguments
    /// * `name` - Item name
    pub fn contains_name(&self, name: &str) -> bool {
        self.hmap.contains_key(name)
    }

    /// Returns the number of items in the container.
    pub fn len(&self) -> usize {
        self.list.len()
    }

         
 
}

#[cfg(test)]
mod tests {

   use noc::NamedObjectsContainer;

    struct Atom {
        proton: u8,
        neutron: u8,
    }

    #[test]
    fn basic_test() {

        // setup data
        let mut noc = NamedObjectsContainer::<Atom>::new(false);
        assert_eq!(noc.len(), 0);

        noc.push("H", Atom{ proton:1, neutron:0 });
        noc.push("He", Atom{ proton:2, neutron:2 });
        assert_eq!(noc.len(), 2);

        assert!(noc.contains_name("H"));
        assert!(noc.contains_name("He"));
        assert!(!noc.contains_name("Cl"));

        let H = noc.get(0).unwrap();
        assert_eq!(H.proton, 1);
        assert_eq!(H.neutron, 0);

        assert!(noc.get(10).is_none());


        
    }

   
}
