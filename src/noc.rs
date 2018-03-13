//! Represents a collection containing elements which could be associated with a name (String).
//! It mimics the Vector collection, but the ability to retrieve an element by its name.
//!
//! Element names could also be duplicated, an getting an element using its name could possibly return several elements.
//! If elements implement the `Nameable` trait, it's not necessary to provide the element's name. If not,
//! the name should be provided when pushing an element.
//! # Examples
use std::collections::HashMap;
use std::fmt;
use std::convert::From;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

use adjustable::Adjustable;
use nameable::Nameable;

#[derive(Debug, Clone)]
pub struct ElementBundle<Element> {
    // keep element inside this struct
    elem: Element,
    // keep its original name and its alternate unique name if any
    name: (String, Option<String>),
}

// Here, the Indexes type could either by a simple usize index, or a vector of indexes in case of
// several elements having the same name
pub struct NamedElementsCollection<Element, Indexes> {
    /// List of Element structs
    list: Vec<ElementBundle<Element>>,
    /// Hashmap keeping track of the name vs. index (or indexes) of the structure in the previous list
    hmap: HashMap<String, Indexes>,
}

impl<Element, Indexes> NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
{
    /// Creates a new collection with named objects.
    pub fn new() -> NamedElementsCollection<Element, Indexes> {
        NamedElementsCollection {
            list: Vec::new(),
            hmap: HashMap::new(),
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
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// assert!(molecule.contains_name("Hydrogen"));
    /// assert!(!molecule.contains_name("Helium"));
    /// ```
    pub fn contains_name(&self, name: &str) -> bool {
        self.hmap.contains_key(name)
    }

    /// Returns the reference on element corresponding to index.
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
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// molecule.push_with_name("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule.get(0).unwrap().proton,1);
    /// ```
    pub fn get(&self, index: usize) -> Option<&Element> {
        match self.list.get(index) {
            Some(v) => Some(&v.elem),
            None => None,
        }
    }

    /// Returns the mutable reference on element corresponding to index.
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
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// molecule.push_with_name("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule.get(0).unwrap().proton,1);
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Element> {
        match self.list.get_mut(index) {
            Some(v) => Some(&mut v.elem),
            None => None,
        }
    }

    /// Returns the number of items in the container.
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// molecule.push_with_name("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule.len(),2);
    /// ```
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Returns an iterator over the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::DNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut water = DNEC::<Atom>::new();
    ///
    /// water.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// water.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// water.push_with_name("Oxygen", Atom{ proton:8, neutron:8 });
    ///
    /// let mut iterator = water.iter();
    ///
    /// assert_eq!(iterator.next().unwrap().proton, 1);
    /// assert_eq!(iterator.next().unwrap().proton, 1);
    /// assert_eq!(iterator.next().unwrap().proton, 8);
    /// ```

    pub fn iter(&self) -> NecIter<Element> {
        self.into_iter()
    }

    /// Returns a mutable iterator over the collection.
    ///
    pub fn iter_mut(&mut self) -> NecIterMut<Element> {
        self.into_iter()
    }

    /// Clears the collection, leaving it empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// molecule.push_with_name("Helium", Atom{ proton:2, neutron:2 });
    ///
    /// molecule.clear();
    /// assert_eq!(molecule.len(),0);
    /// ```
    pub fn clear(&mut self) {
        self.list.clear();
        self.hmap.clear();
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
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { name: String, proton: u8, neutron: u8, };
    /// impl noc::nameable::Nameable for Atom {
    ///     fn get_name(&self) -> &str { &self.name }
    /// }
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push(Atom{ name: "Hydrogen".to_string(), proton:1, neutron:1 });
    /// molecule.push(Atom{ name: "Helium".to_string(), proton:2, neutron:2 });
    /// ```
    pub fn push(&mut self, element: Element)
    where
        Element: Nameable,
    {
        // add new moved element in the list handling elements
        let name = String::from(element.get_name());
        let name2 = name.clone();
        self.list.push(ElementBundle {
            elem: element,
            name: (name, None),
        });

        // get the index of this element, which is the last one
        let index = self.list.len() - 1;

        // get the element name because Element implements Nameable
        //let name = self.list.get(index).unwrap().get_name();
        //let name = self.list.last().unwrap().get_name();

        // add index of this element in the HashMap
        self.hmap.add_entry(&name2, index);
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
    /// ```norun
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// molecule.push_with_name("Helium", Atom{ proton:2, neutron:2 });
    /// ```
    fn _push_with_name(&mut self, name: &str, element: Element) {
        // add element
        self.list.push(ElementBundle {
            elem: element,
            name: (String::from(name), None),
        });

        // add index in the hash
        let index = self.list.len() - 1;
        self.hmap.add_entry(name, index);
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
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// for i in 0..10 {
    ///     molecule.push_with_name(&format!("Atom{}",i), Atom{ proton:i, neutron:i });
    /// }
    ///
    /// molecule.remove(5);
    /// assert_eq!(molecule.len(),9);
    /// ```
    pub fn remove(&mut self, index: usize) -> ElementBundle<Element> {
        // delete from main list
        let e = self.list.remove(index);

        // remove relevant indexes from hashmap
        let name = self.list.get(index).unwrap().name.0.clone();
        self.hmap.remove_entry(&name, index);

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
    /// use noc::noc::DNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut water = DNEC::<Atom>::new();
    ///
    /// water.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// water.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// water.push_with_name("Oxygen", Atom{ proton:8, neutron:8 });
    /// assert_eq!(water.names().len(), 2);
    /// ```
    pub fn names(&self) -> Vec<String> {
        self.hmap.keys().map(|e| e.clone()).collect()
    }

    /// Returns the original name of the element at index.
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
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// molecule.push_with_name("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule.get_name(1).unwrap(), "Helium");
    /// ```
    pub fn get_name(&self, index: usize) -> Option<&String> {
        match self.list.get(index) {
            Some(v) => Some(&v.name.0),
            None => None,
        }
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
impl<Element> NamedElementsCollection<Element, usize> {
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
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// for i in 0..10_u8 {
    ///     molecule.push_with_name(&format!("Atom{}",i), Atom{ proton:i, neutron:i });
    /// }
    ///
    /// for i in 0..10_u8 {
    ///     molecule.push_with_name(&format!("Atom{}",i), Atom{ proton:i, neutron:i });
    /// }
    /// assert_eq!(molecule.len(), 10);
    ///
    /// for i in 0..10_u8 {
    ///     assert_eq!(molecule[i as usize].proton, i);
    /// }
    /// ```
    pub fn push_with_name(&mut self, name: &str, element: Element) {
        // if name is already in our list, just replace the element
        if self.hmap.contains_key(name) {
            // get index of this element
            let index = *self.hmap.get(name).unwrap();

            // and replace the element tuple
            self.list[index] = ElementBundle {
                elem: element,
                name: (String::from(name), None),
            };

            // and also replace in hmap
            self.hmap.replace_entry(name, index);
        }
        // otherwise, business as usual
        else {
            self._push_with_name(name, element);
        }
    }
}

impl<Element> NamedElementsCollection<Element, Vec<usize>> {
    pub fn push_with_name(&mut self, name: &str, element: Element) {
        // add element
        /*
        self.list.push(element);

        // add index in the hash
        let index = self.list.len() - 1;
        self.hmap.add_entry(name, index);

        // add name in the list
        self.name_list.push(ElementName {
            original_name: String::from(name),
            unique_name: None,
        });*/
        self._push_with_name(name, element);
    }
}

impl<Element> NamedElementsCollection<Element, Vec<usize>> {
    /// Returns a vector of elements having the same name.
    ///
    /// # Examples
    ///
    /// ```
    /// use noc::noc::DNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut water = DNEC::<Atom>::new();
    ///
    /// water.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// water.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// water.push_with_name("Oxygen", Atom{ proton:8, neutron:8 });
    ///
    /// let mut v = water.get_by_name("Hydrogen").unwrap();
    ///
    /// assert_eq!(v.len(), 2);
    /// assert_eq!(v[0].proton, 1);
    /// assert_eq!(v[0].neutron, 1);
    /// assert_eq!(v[1].proton, 1);
    /// assert_eq!(v[1].neutron, 1);
    /// assert!(water.get_by_name("Helium").is_none());
    /// ```
    pub fn get_by_name(&self, name: &str) -> Option<Vec<&Element>> {
        if !self.hmap.contains_key(name) {
            return None;
        }

        let indexes = self.hmap.get(name).unwrap();
        let v: Vec<_> = indexes
            .iter()
            .map(|i| &self.list.get(*i).unwrap().elem)
            .collect();

        Some(v)
    }
}

//-----------------------------------------------------------------------
// Index trait
//-----------------------------------------------------------------------
impl<Element, Indexes> Index<usize> for NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
{
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
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// molecule.push_with_name("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule[1].proton, 2);
    /// assert_eq!(molecule[1].neutron, 2);
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        // get reference on vector of items
        self.get(index).unwrap()
    }
}

impl<Element, Indexes> IndexMut<usize> for NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
{
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
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// molecule.push_with_name("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule[1].proton, 2);
    /// assert_eq!(molecule[1].neutron, 2);
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut Element {
        // get reference on vector of items
        self.get_mut(index).unwrap()
    }
}

impl<'a, Element> Index<&'a str> for NamedElementsCollection<Element, usize> {
    type Output = Element;

    /// Get access to an element by providing its name in the collection. This only works for
    /// non-duplicated named objets (UNEC) because this trait returns a reference.
    ///
    /// # Arguments
    /// * `name` - Element name.
    ///
    /// # Panics
    ///
    /// Panics if `name` is not in the collection.
    ///
    /// # Examples
    /// ```
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push_with_name("Hydrogen", Atom{ proton:1, neutron:1 });
    /// molecule.push_with_name("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule["Hydrogen"].proton, 1);
    /// assert_eq!(molecule["Hydrogen"].neutron, 1);
    /// ```
    fn index(&self, name: &str) -> &Self::Output {
        // get reference on vector of items
        let index = *self.hmap.get(name).unwrap();
        &self.list.get(index).unwrap().elem
    }
}

//-----------------------------------------------------------------------
// Iterators
//-----------------------------------------------------------------------
// Defines a dedicated structure for managing iterators.
pub struct NecIntoIterator<Element> {
    iter: ::std::vec::IntoIter<ElementBundle<Element>>,
}

impl<Element, Indexes> IntoIterator for NamedElementsCollection<Element, Indexes> {
    type Item = Element;
    type IntoIter = NecIntoIterator<Element>;

    fn into_iter(self) -> Self::IntoIter {
        NecIntoIterator {
            iter: self.list.into_iter(),
        }
    }
}

impl<Element> Iterator for NecIntoIterator<Element> {
    type Item = Element;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => Some(v.elem),
            None => None,
        }
    }
}

pub struct NecIter<'a, Element: 'a> {
    iter: Iter<'a, ElementBundle<Element>>,
}

impl<'a, Element, Indexes> IntoIterator for &'a NamedElementsCollection<Element, Indexes> {
    type Item = &'a Element;
    type IntoIter = NecIter<'a, Element>;

    fn into_iter(self) -> Self::IntoIter {
        NecIter {
            iter: self.list.iter(),
        }
    }
}

impl<'a, Element> Iterator for NecIter<'a, Element> {
    type Item = &'a Element;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => Some(&v.elem),
            None => None,
        }
    }
}

pub struct NecIterMut<'a, Element: 'a> {
    iter: IterMut<'a, ElementBundle<Element>>,
}

impl<'a, Element, Indexes> IntoIterator for &'a mut NamedElementsCollection<Element, Indexes> {
    type Item = &'a Element;
    type IntoIter = NecIterMut<'a, Element>;

    fn into_iter(self) -> Self::IntoIter {
        NecIterMut {
            iter: self.list.iter_mut(),
        }
    }
}

impl<'a, Element> Iterator for NecIterMut<'a, Element> {
    type Item = &'a Element;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => Some(&v.elem),
            None => None,
        }
    }
}

/*

pub struct NecIteratorA<'a,Element: 'a> {
    iter: &'a ::std::vec::IntoIter<(Element, String, Option<String>)>,
}

impl<'a, Element, Indexes> IntoIterator for &'a NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
{
    type Item = &'a Element;
    type IntoIter = NecIteratorA<'a,Element>;

    fn into_iter(self) -> Self::IntoIter {
        NecIteratorA {
            iter: &self.list.into_iter(),
        }
    }
}

impl<'a, Element> Iterator for NecIteratorA<'a,Element> {
    type Item = &'a Element;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => Some(&v.elem),
            None => None,
        }
    }
}

*/

/*
impl<'a, Element, Indexes> IntoIterator for &'a NamedElementsCollection<Element, Indexes> {
    type Item = &'a Element;
    type IntoIter = Iter<'a, Element>;

    /// Non-consuming iterator.
    ///
    /// # Examples
    /// ```
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// for i in 0..10 {
    ///     molecule.push_with_name(&format!("ATOM{}",i), Atom { proton: i as u8, neutron: i as u8, })
    /// }
    ///
    /// let mut i: u8 = 0;
    /// for atom in &molecule {
    ///       assert_eq!(atom.proton, i);
    ///       assert_eq!(atom.neutron, i);
    ///       assert_eq!(molecule.get_name(i as usize).unwrap().original_name, format!("ATOM{}",i));
    ///       i += 1;
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.list.iter()
    }
}

impl<'a, Element, Indexes> IntoIterator for &'a mut NamedElementsCollection<Element, Indexes> {
    type Item = &'a mut Element;
    type IntoIter = IterMut<'a, Element>;

    /// Non-consuming mutable iterator.
    ///
    /// # Examples
    /// ```
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// for i in 0..10 {
    ///     molecule.push_with_name(&format!("ATOM{}",i), Atom { proton: i as u8, neutron: i as u8, })
    /// }
    ///
    /// let mut i: u8 = 0;
    /// for atom in &mut molecule {
    ///       atom.proton = 0;
    ///       atom.neutron = 0;
    /// }
    ///
    /// for atom in &molecule {
    ///       assert_eq!(atom.proton, 0);
    ///       assert_eq!(atom.neutron, 0);
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.list.iter_mut()
    }
}
*/

//-----------------------------------------------------------------------
// Clone
//-----------------------------------------------------------------------
impl<Element: Clone, Indexes> Clone for NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
{
    fn clone(&self) -> Self {
        let mut cloned = NamedElementsCollection::<Element, Indexes>::new();

        // copy other fields which can be potentially already set
        for (i, element) in self.iter().enumerate() {
            cloned._push_with_name(&self.get_name(i).unwrap(), element.clone());
        }
        cloned
    }
}

//-----------------------------------------------------------------------
// From
//-----------------------------------------------------------------------
impl<'a, Element: Nameable, Indexes> From<Vec<Element>>
    for NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
{
    /// Builds a collection from a vector of elements. Elements should implement
    /// the __Nameable__ trait.
    ///
    /// # Examples
    /// ```
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { name: String, proton: u8, neutron: u8, };
    /// impl noc::nameable::Nameable for Atom {
    ///     fn get_name(&self) -> &str { &self.name }
    /// }
    ///
    /// let v: Vec<_> =
    ///         (0..10).map(|i| Atom{ name: format!("ATOM{}",i) ,proton:i, neutron:i }).collect();
    ///
    /// let molecule = UNEC::<Atom>::from(v);
    /// assert_eq!(molecule[1].proton, 1);
    /// assert_eq!(molecule[9].neutron, 9);
    /// ```
    fn from(source: Vec<Element>) -> Self {
        let mut container = NamedElementsCollection::<Element, Indexes>::new();

        for e in source {
            container.push(e);
        }

        container
    }
}

impl<'a, Element, Indexes> From<Vec<(String, Element)>>
    for NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
{
    /// Builds a collection from a vector of elements.
    ///
    /// # Examples
    /// ```
    /// use noc::noc::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    ///
    /// let v: Vec<_> = (0..10).map(|i| (format!("ATOM{}",i), Atom{ proton:i, neutron:i })).collect();
    ///
    /// let molecule = UNEC::<Atom>::from(v);
    /// assert_eq!(molecule[1].proton, 1);
    /// assert_eq!(molecule[9].neutron, 9);
    /// ```
    fn from(source: Vec<(String, Element)>) -> Self {
        let mut container = NamedElementsCollection::<Element, Indexes>::new();

        for e in source {
            container._push_with_name(&e.0, e.1);
        }

        container
    }
}

//-----------------------------------------------------------------------
// Debug
//-----------------------------------------------------------------------
impl<Element: Nameable + fmt::Debug, Indexes: fmt::Debug> fmt::Debug
    for NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("list: {:?}\n", self.list);
        s += &format!("hmap: {:?}\n", self.hmap);
        //s += &format!("name_list: {:?}\n", self.name_list);

        write!(f, "{}", s)
    }
}

// type aliases
type UniqueNamedElementsCollection<Element> = NamedElementsCollection<Element, usize>;

/// Named objects collection where no name duplication is possible. Adding an element with the same name
/// just replaces the previous one.
///
/// # Examples
/// ```
/// use noc::noc::UNEC;
///
/// struct Atom { proton: u8, neutron: u8, };
/// let mut molecule = UNEC::<Atom>::new();
///
/// molecule.push_with_name("Hydrogen", Atom { proton: 1, neutron: 0, });
/// molecule.push_with_name("Hydrogen", Atom { proton: 1, neutron: 0, });
/// molecule.push_with_name("Hydrogen", Atom { proton: 1, neutron: 0, });
/// // this last push is the winner
/// molecule.push_with_name("Hydrogen", Atom { proton: 1, neutron: 1, });
/// assert_eq!(molecule.len(), 1);
/// ```
pub type UNEC<Element> = UniqueNamedElementsCollection<Element>;

type DuplicateNamedElementsCollection<Element> = NamedElementsCollection<Element, Vec<usize>>;
pub type DNEC<Element> = DuplicateNamedElementsCollection<Element>;
