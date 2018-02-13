//! Represents a record by its name, description and length. Each record contains a list of contiguous fields
//! which hold values when reading a record-based file.
//!
//! # Examples
use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::marker::PhantomData;
//use std::slice::{Iter, IterMut};

// Implemented by those structure from which we can get the name
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



pub struct NamedObjectsContainer<Item, Collection> {
    /// List of T structs
    pub list: Vec<Item>,
    /// Hashmap keeping track of the name vs. index of the structure in the previous list
    pub hmap: HashMap<String, Collection>,
}

impl<Item, Collection> NamedObjectsContainer<Item, Collection> {
    
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

    /// Returns the item corresponding to index
    ///
    /// # Arguments
    /// * `index` - Element index
    fn _get_item_by_index(&self, index: usize) -> Option<&Item> {
        self.list.get(index)
    }

    fn _push(&mut self, item: Item) -> usize {
        self.list.push(item);
        self.list.len() - 1
    }

    fn _get_item_by_name(&self, name: &str) -> Option<&Collection> {
        self.hmap.get(name)
    }    
   
}

// 1st case: no duplicates, Element implements Nameable.
impl<'a, Element: Nameable> NamedObjectsContainer<Element, &'a Element> {
    
    /// Creates a new Container.
    ///
    /// # Arguments
    /// * `allow_dup` - True is items having the same name are allowed
    pub fn new() -> NamedObjectsContainer<Element, &'a Element> {
        // initialize all relevant members
        NamedObjectsContainer {
            list: Vec::new(),
            hmap: HashMap::new(),
        }        
    }

    /// Returns the item corresponding to index
    ///
    /// # Arguments
    /// * `index` - Element index
    pub fn get(&self, index: usize) -> Option<&Element> {
        self._get_item_by_index(index)
    }    

    pub fn push(&'a mut self, element: Element) {
        // extract name from element
        let name = element.get_name();

        // no duplicates: so check first if element is already there
        if self.contains_name(&name) {
            panic!("already in container");
        }

        // finally, save Field struct
        //self._push(element);
        //self.list.push(element);

        // item is pushed at the end of the list. So its index is length of the list - 1
        let index = self._push(element);

        // if the item's name is already in the list, just add the index to the vector holding the list
        // of indexes for the same name
        let elem_ref = self.list.get(index).unwrap();

        self.hmap.insert(name, &elem_ref);
    }
}

// 2nd case: no duplicates, Element doesn't implement Nameable.
impl<'a, Element> NamedObjectsContainer<Item<Element>, &'a Element> {
    /// Creates a new Container.
    ///
    /// # Arguments
    /// * `allow_dup` - True is items having the same name are allowed
    pub fn new() -> NamedObjectsContainer<Item<Element>, &'a Element> {
        // initialize all relevant members
        NamedObjectsContainer {
            list: Vec::new(),
            hmap: HashMap::new(),
        }        
    }

    /// Returns the item corresponding to index
    ///
    /// # Arguments
    /// * `index` - Element index
    pub fn get(&self, index: usize) -> Option<&Element> {
        match self._get_item_by_index(index) {
            Some(v) => Some(&v.element),
            None => None,
        }
    }     

    pub fn push(&'a mut self, name: &str, element: Element) {
        // create new itemNamedObjectsContainer
        let item = Item{ name: String::from(name), element: element };

        // finally, save Field struct
        //self._push(item);
        //self.list.push(item);

        // item is pushed at the end of the list. So its index is length of the list - 1
        let index = self._push(item);

        // if the item's name is already in the list, just add the index to the vector holding the list
        // of indexes for the same name
        let elem_ref = &(*self.list.get(index).unwrap()).element;

        self.hmap.insert(String::from(name), &elem_ref);
    }    
}

// 3rd case: duplicates, Element implements Nameable.
impl<'a, Element: Nameable> NamedObjectsContainer<Element, Vec<&'a Element>> {
    /// Creates a new Container.
    ///
    /// # Arguments
    /// * `allow_dup` - True is items having the same name are allowed
    pub fn new() -> NamedObjectsContainer<Element, Vec<&'a Element>> {
        // initialize all relevant members
        NamedObjectsContainer {
            list: Vec::new(),
            hmap: HashMap::new(),
        }        
    }

    pub fn push(&'a mut self, element: Element) {
        // extract name from element
        let name = element.get_name();

        // item is pushed at the end of the list. So its index is length of the list - 1
        let index = self._push(element);

        // if the item's name is already in the list, just add the index to the vector holding the list
        // of indexes for the same name
        let elem_ref = self.list.get(index).unwrap();
        self.hmap.entry(name).or_insert(Vec::new()).push(&elem_ref);

        /*
        if self.hmap.contains_key(&name) {
            self.hmap.get_mut(&name).unwrap().push(&elem_ref);
        }
        // if not, create list of indices and add new index to it
        else {
            self.hmap.insert(name, vec![&elem_ref]);
        }*/
    }    
}

// 4th case: duplicates, Element doesn't implement Nameable.
impl<'a, Element> NamedObjectsContainer<Item<Element>, Vec<&'a Element>> {
    /// Creates a new Container.
    ///
    /// # Arguments
    /// * `allow_dup` - True is items having the same name are allowed
    pub fn new() -> NamedObjectsContainer<Item<Element>, Vec<&'a Element>> {
        // initialize all relevant members
        NamedObjectsContainer {
            list: Vec::new(),
            hmap: HashMap::new(),
        }        
    }
}