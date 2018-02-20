//! Represents a record by its name, description and length. Each record contains a list of contiguous fields
//! which hold values when reading a record-based file.
//!
//! # Examples
use std::collections::LinkedList;
use std::collections::HashMap;

use std::fmt;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};


// Implemented by those structure from which we can get the name
pub trait Nameable {
    fn get_name(&self) -> String;
}
pub trait Insertable {
    fn add_entry(&mut self, name: String, index: usize);
}
impl Insertable for HashMap<String, usize> {
    fn add_entry(&mut self, name: String, index: usize) {
        self.insert(name, index);
    }
}
impl Insertable for HashMap<String, Vec<usize>> {
    fn add_entry(&mut self, name: String, index: usize) {
        self.entry(name).or_insert(Vec::new()).push(index);
    }
}


pub struct NamedObjectsContainer<Element, Collection> {
    /// List of T structs
    pub list: Vec<Element>,
    /// Hashmap keeping track of the name vs. index of the structure in the previous list
    pub hmap: HashMap<String, Collection>,
}

impl<Element: Nameable, Collection> NamedObjectsContainer<Element, Collection>
 where 
    HashMap<String,Collection>: Insertable {

    /// Creates a new Container with named objects.
    pub fn new() -> NamedObjectsContainer<Element, Collection> {
        // initialize all relevant members
        NamedObjectsContainer {
            list: Vec::new(),
            hmap: HashMap::new(),
        }        
    }    
    
    /// Tests whether the container contains an item by giving its name.
    ///
    /// # Arguments
    /// * `name` - Item name
    pub fn contains_name(&self, name: &str) -> bool {
        self.hmap.contains_key(name)
    }
    
    /// Returns the item corresponding to index
    ///
    /// # Arguments
    /// * `index` - Element index
    pub fn get(&self, index: usize) -> Option<&Element> {
        self.list.get(index)
    }     

    /// Returns the number of items in the container.
    pub fn len(&self) -> usize {
        self.list.len()
    }

    // add an item at the end of the list
    pub fn push(&mut self, element: Element) {
        let name = element.get_name();
        self.list.push(element);
        let index = self.list.len() - 1;
        self.hmap.add_entry(name, index);
    }

    fn _get_item_by_name(&self, name: &str) -> Option<&Collection> {
        self.hmap.get(name)
    }    
   
}

//-----------------------------------------------------------------------
// [] access
//-----------------------------------------------------------------------
impl<Element, Collection> Index<usize> for NamedObjectsContainer<Element, Collection> {
    type Output = Element;

    fn index(&self, index: usize) -> &Self::Output {
        // get reference on vector of items
        self.list.get(index).unwrap()
    }
}

impl<Element> NamedObjectsContainer<Element, usize> {
    pub fn get_by_index(&self, index: usize) -> Option<&Element> {
        self.list.get(index)
    }
}

impl<Element> NamedObjectsContainer<Element, Vec<usize>> {
    pub fn get_by_name(&self, name: &str) -> Option<Vec<&Element>> {
        if !self.hmap.contains_key(name) {
            return None;
        }

        let indexes = self.hmap.get(name).unwrap();
        let v: Vec<_> = indexes.iter().map(|i| self.list.get(*i).unwrap()).collect();
        
        Some(v)
    }
}

//-----------------------------------------------------------------------
// Iterators
//-----------------------------------------------------------------------

// consuming iterator
impl<Element, Collection> IntoIterator for NamedObjectsContainer<Element, Collection> {
    type Item = Element;
    type IntoIter = ::std::vec::IntoIter<Element>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

// non-consuming iterator (access items by ref)
impl<'a, Element, Collection> IntoIterator for &'a NamedObjectsContainer<Element, Collection> {
    type Item = &'a Element;
    type IntoIter = Iter<'a, Element>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.iter()
    }
}

// non-consuming mutable iterator (access items by mut ref)
impl<'a, Element, Collection> IntoIterator for &'a mut NamedObjectsContainer<Element, Collection> {
    type Item = &'a mut Element;
    type IntoIter = IterMut<'a, Element>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.iter_mut()
    }
}

//-----------------------------------------------------------------------
// Clone
//-----------------------------------------------------------------------
impl<Element: Nameable + Clone, Collection> Clone for NamedObjectsContainer<Element, Collection>
 where 
    HashMap<String,Collection>: Insertable {

    fn clone(&self) -> Self {
        let mut cloned = NamedObjectsContainer::<Element, Collection>::new();

        // copy other fields which can be potentially already set
        for element in self {
            cloned.push(element.clone());
        }
        cloned
    }
}

//-----------------------------------------------------------------------
// From
//-----------------------------------------------------------------------
impl<'a, Element: Nameable, Collection> From<Vec<Element>> for NamedObjectsContainer<Element, Collection>
 where 
    HashMap<String,Collection>: Insertable {
    
    fn from(v: Vec<Element>) -> Self {
        let mut container = NamedObjectsContainer::<Element, Collection>::new();

        for e in v {
            container.push(e);
        }

        container
    }
}

// type aliases
pub type UniqueNamedObjectsContainer<T> = NamedObjectsContainer<T, usize>;
pub type UNOC<T> = UniqueNamedObjectsContainer<T>;

pub type DuplicateNamedObjectsContainer<T> = NamedObjectsContainer<T, Vec<usize>>;
pub type DNOC<T> = DuplicateNamedObjectsContainer<T>;

