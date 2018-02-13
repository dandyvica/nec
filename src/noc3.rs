//! Represents a record by its name, description and length. Each record contains a list of contiguous fields
//! which hold values when reading a record-based file.
//!
//! # Examples
use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};


// Implemented by those structure from which we can get the name
pub trait Nameable {
    fn get_name(&self) -> String;
}


pub struct NamedObjectsContainer<Element, Collection> {
    /// List of T structs
    pub list: Vec<Element>,
    /// Hashmap keeping track of the name vs. index of the structure in the previous list
    pub hmap: HashMap<String, Collection>,
}

impl<Element: Nameable, Collection> NamedObjectsContainer<Element, Collection> {

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

    fn _push(&mut self, element: Element) -> usize {
        self.list.push(element);
        self.list.len() - 1
    }

    fn _get_item_by_name(&self, name: &str) -> Option<&Collection> {
        self.hmap.get(name)
    }    
   
}

// 1st case: no duplicates, Element implements Nameable.
impl<'a, Element: Nameable> NamedObjectsContainer<Element, &'a Element> {
  

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

// 2nd case: duplicates, Element implements Nameable.
impl<'a, Element: Nameable> NamedObjectsContainer<Element, Vec<&'a Element>> {

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

//-----------------------------------------------------------------------
// [] access
//-----------------------------------------------------------------------
impl<'a, Element, Collection> Index<&'a str> for NamedObjectsContainer<Element, Collection> {
    type Output = Collection;

    fn index(&self, name: &str) -> &Self::Output {
        // get reference on vector of items
        self.hmap.get(name).unwrap()
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
impl<'a, Element: Nameable + Clone> Clone for NamedObjectsContainer<Element, &'a Element> {

    fn clone(&self) -> NamedObjectsContainer<Element, &'a Element> {
        let cloned = {

            let c = NamedObjectsContainer::<Element, &Element>::new();
            let e = self.get(0).unwrap().clone();
            { c.push(e); }

            c
        };
        
        
        

        // copy other fields which can be potentially already set
        /*for ref element in self {
            cloned.push(*element.clone());
        } */  
        


        cloned
    }
}

