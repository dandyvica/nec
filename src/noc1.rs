//! Represents a record by its name, description and length. Each record contains a list of contiguous fields
//! which hold values when reading a record-based file.
//!
//! # Examples
use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::marker::PhantomData;
//use std::slice::{Iter, IterMut};

pub trait Nameable {
    fn get_name(&self) -> String;
}

// Encompassing structure to keep tie between name and element
pub struct Item<Element> {
    name: String,
    element: Element,
}

pub struct Unicity;
pub struct Multiplicity;



pub struct NamedObjectsContainer<'a,T:'a, Mode> {
    /// List of T structs
    pub list: Vec<Item<T>>,
    /// Hashmap keeping track of the name vs. index of the structure in the previous list
    pub hmap: HashMap<String, Vec<&'a T>>,
    /// Reader mode struct, just a place holder
    pub mode: PhantomData<Mode>,    
}

impl<'a,T,Mode> NamedObjectsContainer<'a,T,Mode> {
    /// Creates a new Container.
    ///
    /// # Arguments
    /// * `allow_dup` - True is items having the same name are allowed
    pub fn new() -> NamedObjectsContainer<'a,T,Mode> {
        // initialize all relevant members
        NamedObjectsContainer {
            list: Vec::new(),
            hmap: HashMap::new(),
            mode: PhantomData,
        }        
    }

    pub fn push_with_name(&'a mut self, name: &str, element: T) {
        // create new itemNamedObjectsContainer
        let item = Item{ name: String::from(name), element: element };

        // finally, save Field struct
        self.list.push(item);

        // item is pushed at the end of the list. So its index is length of the list - 1
        let index = self.list.len() - 1;

        // if the item's name is already in the list, just add the index to the vector holding the list
        // of indexes for the same name
        let elem_ref = &(*self.list.get(index).unwrap()).element;

        if self.hmap.contains_key(name) {
            self.hmap.get_mut(name).unwrap().push(&elem_ref);
        }
        // if not, create list of indices and add new index to it
        else {
            self.hmap.insert(String::from(name), vec![&elem_ref]);
        }
    }

    /// Add element at the end of the container. As T implements Nameable, no need to pass the name.
    ///
    /// # Arguments
    /// * `element` - Element to add
    pub fn push(&'a mut self, element: T) where T: Nameable {
        self.push_with_name(&element.get_name(), element);

    }

    /// Returns the name of the element corresponding to index
    ///
    /// # Arguments
    /// * `index` - Element index
    pub fn name(&self, index: usize) -> Option<&str> {
        match self.list.get(index) {
            Some(v) => Some(&v.name),
            None => None,
        }
    }

    /// Returns the item corresponding to index
    ///
    /// # Arguments
    /// * `index` - Element index
    pub fn get(&self, index: usize) -> Option<&T> {
        match self.list.get(index) {
            Some(v) => Some(&v.element),
            None => None,
        }
    }

    /// Returns the mutable item corresponding to index
    ///NamedObjectsContainer
    /// # Arguments
    /// * `index` - Item index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match self.list.get_mut(index) {
            Some(v) => Some(&mut v.element),
            None => None,
        }    
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

    /*pub fn get_by_name(&self, name: &str) -> Option<&Vec<&T>> {
        self.hmap.get(name)
    }*/

 
}


impl<'a, T> Index<&'a str> for NamedObjectsContainer<'a,T,Unicity> {
    type Output = T;

    fn index(&self, name: &str) -> &Self::Output {
        // get reference on vector of items
        let v = &*self.hmap.get(name).unwrap();

        // get inner item 
        let inner_item = v.get(0).unwrap();

        // return reference on element
        inner_item
    }
}

/*
impl<'a, T> IndexMut<&'a str> for NamedObjectsContainer<'a,T,Unicity> {

    fn index_mut(&mut self, name: &str) -> &mut T {
        // get reference on vector of items
        let v = &mut *self.hmap.get_mut(name).unwrap();

        // get inner item 
        let inner_item = v.get_mut(0).unwrap();

        // return reference on element
        inner_item
    }
}
*/

impl<'a,T> Index<&'a str> for NamedObjectsContainer<'a,T,Multiplicity> {
    type Output = Vec<&'a T>;

    fn index(&self, name: &str) -> &Self::Output {
        // get reference on vector of items
        self.hmap.get(name).unwrap()
    }
}

