//! Represents a container which contains elements which could be associated with a name (String).
//! It mimics the Vector collection, but the ability to retrieve an element by its name. Elements' names
//! could also be duplicated, an getting elements by their name could return several elements.
//! If elements implement the Nameable trait, it's not necessary to provide the element's name. If not,
//! the name is need when pushing an element.
//! # Examples
use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

use adjustable::Adjustable;
use nameable::Nameable;

// As names could ne duplicated, it might be useful to define a unique name.
#[derive(Debug, Clone)]
pub struct ElementName {
    pub original_name: String,
    pub unique_name: Option<String>,
}

// Here, the Indexes type could either by a simple usize index, or a vector of indexes in case of
// several elements having the same name
pub struct NamedObjectsCollection<Element, Indexes> {
    /// List of Element structs
    list: Vec<Element>,
    /// Hashmap keeping track of the name vs. index (or indexes) of the structure in the previous list
    hmap: HashMap<String, Indexes>,
    /// keep track of all element names
    name_list: Vec<ElementName>,
}

impl<Element, Indexes> NamedObjectsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
{
    /// Creates a new collection with named objects.
    pub fn new() -> NamedObjectsCollection<Element, Indexes> {
        NamedObjectsCollection {
            list: Vec::new(),
            hmap: HashMap::new(),
            name_list: Vec::new(),
        }
    }

    /// Tests whether the container contains an item by giving its name.
    ///
    /// # Arguments
    /// * `name` - Element name
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::UNOC;
    ///
    /// struct Atom { p: u8, n: u8, };
    /// let mut noc = UNOC::<Atom>::new();
    ///
    /// noc.push_with_name("Hydrogen", Atom{ p:1, n:1 });
    /// assert!(noc.contains_name("Hydrogen"));
    /// assert!(!noc.contains_name("Helium"));
    /// ```
    pub fn contains_name(&self, name: &str) -> bool {
        self.hmap.contains_key(name)
    }

    /// Returns the element corresponding to index.
    ///
    /// # Arguments
    /// * `index` - Element index
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::UNOC;
    ///
    /// struct Atom { p: u8, n: u8, };
    /// let mut noc = UNOC::<Atom>::new();
    ///
    /// noc.push_with_name("Hydrogen", Atom{ p:1, n:1 });
    /// noc.push_with_name("Helium", Atom{ p:2, n:2 });
    /// assert_eq!(noc.get(0).unwrap().p,1);
    /// ```
    pub fn get(&self, index: usize) -> Option<&Element> {
        self.list.get(index)
    }

    /// Returns the number of items in the container.
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::UNOC;
    ///
    /// struct Atom { p: u8, n: u8, };
    /// let mut noc = UNOC::<Atom>::new();
    ///
    /// noc.push_with_name("Hydrogen", Atom{ p:1, n:1 });
    /// noc.push_with_name("Helium", Atom{ p:2, n:2 });
    /// assert_eq!(noc.len(),2);
    /// ```
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Returns an iterator over the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::DNOC;
    ///
    /// struct Atom { p: u8, n: u8, };
    /// let mut water = DNOC::<Atom>::new();
    ///
    /// water.push_with_name("Hydrogen", Atom{ p:1, n:1 });
    /// water.push_with_name("Hydrogen", Atom{ p:1, n:1 });
    /// water.push_with_name("Oxygen", Atom{ p:8, n:8 });
    ///
    /// let mut iterator = water.iter();
    ///
    /// assert_eq!(iterator.next().unwrap().p, 1);
    /// assert_eq!(iterator.next().unwrap().p, 1);
    /// assert_eq!(iterator.next().unwrap().p, 8);
    /// ```
    pub fn iter(&self) -> Iter<Element> {
        self.list.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Element> {
        self.list.iter_mut()
    }

    /// Clears the collection, leaving it empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::UNOC;
    ///
    /// struct Atom { p: u8, n: u8, };
    /// let mut noc = UNOC::<Atom>::new();
    ///
    /// noc.push_with_name("Hydrogen", Atom{ p:1, n:1 });
    /// noc.push_with_name("Helium", Atom{ p:2, n:2 });
    ///
    /// noc.clear();
    /// assert_eq!(noc.len(),0);
    /// ```
    pub fn clear(&mut self) {
        self.list.clear();
        self.hmap.clear();
        self.name_list.clear();
    }

    /// Adds an item at the end of the list. In this case, as Element type implements the Nameable trait,
    /// the element's name is found by calling the trait's method.
    ///
    /// # Arguments
    /// * `element` - Element structure
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::UNOC;
    ///
    /// struct Atom { name: String, p: u8, n: u8, };
    /// impl noc::nameable::Nameable for Atom {
    ///     fn get_name(&self) -> &str { &self.name }
    /// }
    /// let mut noc = UNOC::<Atom>::new();
    ///
    /// noc.push(Atom{ name: "Hydrogen".to_string(), p:1, n:1 });
    /// noc.push(Atom{ name: "Helium".to_string(), p:2, n:2 });
    /// ```
    pub fn push(&mut self, element: Element)
    where
        Element: Nameable,
    {
        // add new moved element in the list handling elements
        self.list.push(element);

        // get the index of this element, which is the last one
        let index = self.list.len() - 1;

        // get the element name because Element implements Nameable
        let name = self.list.get(index).unwrap().get_name();

        // add index of this element in the HashMap
        self.hmap.add_entry(name, index);

        // save element's name also in the name list
        self.name_list.push(ElementName {
            original_name: String::from(name),
            unique_name: None,
        });
    }

    /// Adds an item at the end of the list. In this case, as Element type doesn't implement the Nameable trait,
    /// the caller must provide the name.
    ///
    /// # Arguments
    /// * `name` - Element name
    /// * `element` - Element structure
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::UNOC;
    ///
    /// struct Atom { p: u8, n: u8, };
    /// let mut noc = UNOC::<Atom>::new();
    ///
    /// noc.push_with_name("Hydrogen", Atom{ p:1, n:1 });
    /// noc.push_with_name("Helium", Atom{ p:2, n:2 });
    /// ```
    pub fn push_with_name(&mut self, name: &str, element: Element) {
        // add element
        self.list.push(element);

        // add index in the hash
        let index = self.list.len() - 1;
        self.hmap.add_entry(name, index);

        // add name in the list
        self.name_list.push(ElementName {
            original_name: String::from(name),
            unique_name: None,
        });
    }

    /// Removes an element from the collection by giving its index.
    /// the caller must provide the name.
    ///
    /// # Arguments
    /// * `index` - Element index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::UNOC;
    ///
    /// struct Atom { p: u8, n: u8, };
    /// let mut noc = UNOC::<Atom>::new();
    ///
    /// for i in 0..10 {
    ///     noc.push_with_name(&format!("Atom{}",i), Atom{ p:i, n:i });
    /// }
    ///
    /// noc.remove(5);
    /// assert_eq!(noc.len(),9);
    /// ```
    pub fn remove(&mut self, index: usize) -> Element {
        // delete from main list
        let e = self.list.remove(index);

        /// remove relevant indexes from hashmap
        let name = self.name_list.get(index).unwrap().original_name.clone();
        self.hmap.remove_entry(&name, index);

        /// remove from list of names
        self.name_list.remove(index);

        e
    }

    /// Returns the list of __original_name__ names, without duplication.
    ///
    /// # Arguments
    /// * `name` - Element name
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::DNOC;
    ///
    /// struct Atom { p: u8, n: u8, };
    /// let mut water = DNOC::<Atom>::new();
    ///
    /// water.push_with_name("Hydrogen", Atom{ p:1, n:1 });
    /// water.push_with_name("Hydrogen", Atom{ p:1, n:1 });
    /// water.push_with_name("Oxygen", Atom{ p:8, n:8 });
    /// assert_eq!(water.names().len(), 2);
    /// ```
    pub fn names(&self) -> Vec<String> {
        self.hmap.keys().map(|e| e.clone()).collect()
    }

    /// Returns Option<&ElementName>.
    ///
    /// # Arguments
    /// * `name` - Element name
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::UNOC;
    ///
    /// struct Atom { p: u8, n: u8, };
    /// let mut noc = UNOC::<Atom>::new();
    ///
    /// noc.push_with_name("Hydrogen", Atom{ p:1, n:1 });
    /// noc.push_with_name("Helium", Atom{ p:2, n:2 });
    /// assert_eq!(noc.get_name(1).unwrap().original_name, "Helium");
    /// ```
    pub fn get_name(&self, index: usize) -> Option<&ElementName> {
        self.name_list.get(index)
    }

    /*
    pub fn check(&self) -> bool {
        let n1 = self.list.len();

        let mut n2 = 0;
        for (k, v) in self.hmap {

        }
        let n2 = self.hamp
    }*/
}

//-----------------------------------------------------------------------
// Specializations
//-----------------------------------------------------------------------

impl<Element> NamedObjectsCollection<Element, Vec<usize>> {
    /// Returns a vector of elements having the same name.
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::DNOC;
    ///
    /// struct Atom { p: u8, n: u8, };
    /// let mut water = DNOC::<Atom>::new();
    ///
    /// water.push_with_name("Hydrogen", Atom{ p:1, n:1 });
    /// water.push_with_name("Hydrogen", Atom{ p:1, n:1 });
    /// water.push_with_name("Oxygen", Atom{ p:8, n:8 });
    ///
    /// let mut v = water.get_by_name("Hydrogen").unwrap();
    ///
    /// assert_eq!(v[0].p, 1);
    /// assert_eq!(v[0].n, 1);
    /// assert_eq!(v[1].p, 1);
    /// assert_eq!(v[1].n, 1);
    /// assert!(water.get_by_name("Helium").is_none());
    /// ```
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
// Traits implementations
//-----------------------------------------------------------------------
impl<Element, Indexes> Index<usize> for NamedObjectsCollection<Element, Indexes> {
    type Output = Element;

    /// Get access to an element by providing its index in the collection.
    ///
    /// # Arguments
    /// * `index` - Element index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    /// ```
    /// use noc::noc::UNOC;
    ///
    /// struct Atom { p: u8, n: u8, };
    /// let mut noc = UNOC::<Atom>::new();
    ///
    /// noc.push_with_name("Hydrogen", Atom{ p:1, n:1 });
    /// noc.push_with_name("Helium", Atom{ p:2, n:2 });
    /// assert_eq!(noc[1].p, 2);
    /// assert_eq!(noc[1].n, 2);
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        // get reference on vector of items
        self.list.get(index).unwrap()
    }
}

//-----------------------------------------------------------------------
// Iterators
//-----------------------------------------------------------------------

// consuming iterator
impl<Element, Indexes> IntoIterator for NamedObjectsCollection<Element, Indexes> {
    type Item = Element;
    type IntoIter = ::std::vec::IntoIter<Element>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

// non-consuming iterator (access items by ref)
impl<'a, Element, Indexes> IntoIterator for &'a NamedObjectsCollection<Element, Indexes> {
    type Item = &'a Element;
    type IntoIter = Iter<'a, Element>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.iter()
    }
}

// non-consuming mutable iterator (access items by mut ref)
impl<'a, Element, Indexes> IntoIterator for &'a mut NamedObjectsCollection<Element, Indexes> {
    type Item = &'a mut Element;
    type IntoIter = IterMut<'a, Element>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.iter_mut()
    }
}

//-----------------------------------------------------------------------
// Clone
//-----------------------------------------------------------------------
impl<Element: Clone, Indexes> Clone for NamedObjectsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
{
    fn clone(&self) -> Self {
        let mut cloned = NamedObjectsCollection::<Element, Indexes>::new();

        // copy other fields which can be potentially already set
        for (i, element) in self.iter().enumerate() {
            cloned.push_with_name(&self.name_list[i].original_name, element.clone());
        }
        cloned
    }
}

//-----------------------------------------------------------------------
// From
//-----------------------------------------------------------------------
impl<'a, Element: Nameable, Indexes> From<Vec<Element>> for NamedObjectsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
{
    fn from(source: Vec<Element>) -> Self {
        let mut container = NamedObjectsCollection::<Element, Indexes>::new();

        for e in source {
            container.push(e);
        }

        container
    }
}

impl<'a, Element: Nameable, Indexes> From<Vec<(String, Element)>>
    for NamedObjectsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
{
    fn from(source: Vec<(String, Element)>) -> Self {
        let mut container = NamedObjectsCollection::<Element, Indexes>::new();

        for e in source {
            container.push_with_name(&e.0, e.1);
        }

        container
    }
}

//-----------------------------------------------------------------------
// Debug
//-----------------------------------------------------------------------
impl<Element: Nameable + fmt::Debug, Indexes: fmt::Debug> fmt::Debug
    for NamedObjectsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("list: {:?}\n", self.list);
        s += &format!("hmap: {:?}\n", self.hmap);
        s += &format!("name_list: {:?}\n", self.name_list);

        write!(f, "{}", s)
    }
}

// type aliases
type UniqueNamedObjectsCollection<Element> = NamedObjectsCollection<Element, usize>;
pub type UNOC<Element> = UniqueNamedObjectsCollection<Element>;

type DuplicateNamedObjectsCollection<Element> = NamedObjectsCollection<Element, Vec<usize>>;
pub type DNOC<Element> = DuplicateNamedObjectsCollection<Element>;
