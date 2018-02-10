//! Represents a record by its name, description and length. Each record contains a list of contiguous fields
//! which hold values when reading a record-based file.
//!
//! # Examples
use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};
//use std::slice::{Iter, IterMut};


pub trait Nameable<T> {
    //
    type Item;

    // add an item at the end of the container
    fn push(&mut self, name: &str, item: T);

    // get an item or a list of items, depending on the container (unique or duplicate)
    fn get_by_name(&self, name: &str) -> Self::Item;

}


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


#[derive(Clone)]
pub struct NamedObjectsContainer<T,U> {
    /// List of T structs
    pub list: Vec<T>,
    /// Hashmap keeping track of the name vs. index of the structure in the previous list
    pub hmap: HashMap<String, U>,
}

impl<T,U> NamedObjectsContainer<T,U> {
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

impl<'a, T: 'a> Nameable<T> for NamedObjectsContainer<T, Vec<usize>> {
    // 
    type Item = Vec<&'a T>;

    /// Adds a Field structure to the end of the record.
    fn push(&mut self, name: &str, item: T) {
        // finally, save Field struct
        self.list.push(item);

        // item is pushed at the end of the list. So its index is length of the list - 1
        let index = self.list.len() - 1;

        // if the item's name is already in the list, just add the index to the vector holding the list
        // of indexes for the same name
        if self.hmap.contains_key(name) {
            self.hmap.get_mut(name).unwrap().push(index);
        }
        // if not, create list of indices and add new index to it
        else {
            self.hmap.insert(String::from(name), vec![index]);
        }
    }

    fn get_by_name(&self, name: &str) -> Self::Item {
        // get vector of indexes from hmap
        let indexes = self.hmap.get(name).expect("no entry found for key");

        // build the vector of items corresponding to previous indexes
        let items: Vec<_> = indexes.iter().map(|i| *self.list.get(*i).unwrap()).collect();

        // return vector
        items
    }
}

impl<'a,T> Nameable<T> for NamedObjectsContainer<T, usize> {
    //
    type Item = &'a T;

    /// Adds a Field structure to the end of the record.
    fn push(&mut self, name: &str, item: T) {
        // finally, save Field struct
        self.list.push(item);

        // item is pushed at the end of the list. So its index is length of the list - 1
        let index = self.list.len() - 1;

        // if the item's name is already in the list, just add the index to the vector holding the list
        self.hmap.insert(String::from(name), index);
    }

    fn get_by_name(&'a self, name: &str) -> Self::Item {
        // get the single index from hmap. This index allows to find the corresponding item
        let index = self.hmap.get(name).expect("no entry found for key");

        // get item
        &self.list[*index]
    }
}



// type aliases
pub type UniqueNamedObjectsContainer<T> = NamedObjectsContainer<T, usize>;
pub type UNOC<T> = UniqueNamedObjectsContainer<T>;

pub type DuplicateNamedObjectsContainer<T> = NamedObjectsContainer<T, Vec<usize>>;
pub type DNOC<T> = DuplicateNamedObjectsContainer<T>;


