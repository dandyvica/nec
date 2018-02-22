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
    fn get_name(&self) -> &str;
}
pub trait Insertable {
    fn add_entry(&mut self, name: &str, index: usize);
}
impl Insertable for HashMap<String, usize> {
    fn add_entry(&mut self, name: &str, index: usize) {
        self.insert(name.to_string(), index);
    }
}
impl Insertable for HashMap<String, Vec<usize>> {
    fn add_entry(&mut self, name: &str, index: usize) {
        self.entry(name.to_string())
            .or_insert(Vec::new())
            .push(index);
    }
}

pub struct NamedObjectsContainer<Element, Collection> {
    /// List of T structs
    list: Vec<Element>,
    /// Hashmap keeping track of the name vs. index of the structure in the previous list
    hmap: HashMap<String, Collection>,
}

/*
pub struct Wrapped<Element> {
    name: String,
    element: Element,
}

impl<Element> Nameable for Wrapped<Element> {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}*/

impl<Element, Collection> NamedObjectsContainer<Element, Collection>
where
    HashMap<String, Collection>: Insertable,
{
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
    pub fn push(&mut self, element: Element)
    where
        Element: Nameable,
    {
        self.list.push(element);
        let index = self.list.len() - 1;

        let last_element = self.list.get(index).unwrap();
        let name = last_element.get_name();

        self.hmap.add_entry(name, index);
    }

    // add an item at the end of the list
    pub fn push_with_name(&mut self, name: &str, element: Element) {
        self.list.push(element);
        let index = self.list.len() - 1;
        self.hmap.add_entry(name, index);
    }

    pub fn names(&self) -> Vec<String> {
        self.hmap.keys().map(|e| e.clone()).collect()
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
    HashMap<String, Collection>: Insertable,
{
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
impl<'a, Element: Nameable, Collection> From<Vec<Element>>
    for NamedObjectsContainer<Element, Collection>
where
    HashMap<String, Collection>: Insertable,
{
    fn from(v: Vec<Element>) -> Self {
        let mut container = NamedObjectsContainer::<Element, Collection>::new();

        for e in v {
            container.push(e);
        }

        container
    }
}

// type aliases
pub type UniqueNamedObjectsContainer<Element> = NamedObjectsContainer<Element, usize>;
pub type UNOC<Element> = UniqueNamedObjectsContainer<Element>;

pub type DuplicateNamedObjectsContainer<Element> = NamedObjectsContainer<Element, Vec<usize>>;
pub type DNOC<Element> = DuplicateNamedObjectsContainer<Element>;

//pub type Test<T> = NamedObjectsContainer<Wrapped<T>, usize>;
