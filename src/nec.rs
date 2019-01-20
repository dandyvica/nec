//! Represents a collection containing elements which could be associated with a name.
//! It mimics the `Vector` collection, but with the ability to retrieve an element by its name.
//!
//! Element names could also be duplicated, and getting an element providing its name could possibly return several elements.
//! The name should be provided when pushing an element into the collection.
//!
//! # Examples
//! ```
//! use nec::nec::DNEC;
//!
//! struct Atom { proton: u8, neutron: u8, };
//! let mut water = DNEC::<Atom>::new();
//!
//! water.push("Hydrogen", Atom{ proton:1, neutron:0 });
//! water.push("Hydrogen", Atom{ proton:1, neutron:0 });
//! water.push("Oxygen", Atom{ proton:8, neutron:8 });
//!
//! assert_eq!(water.get_by_name("Hydrogen").unwrap().len(), 2);
//! ```
//! ```
//! use nec::nec::UNEC;
//!
//! struct Atom { proton: u8, neutron: u8, };
//! let mut water = UNEC::<Atom>::new();
//!
//! water.push("Hydrogen", Atom{ proton:1, neutron:0 });
//! water.push("Hydrogen", Atom{ proton:1, neutron:0 });
//! water.push("Oxygen", Atom{ proton:8, neutron:8 });
//!
//! assert_eq!(water.len(), 2);
//! ```

use std::collections::HashMap;
use std::convert::From;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

use adjustable::Adjustable;

/// Encompassing structure for storing not only the element object, by also its connected data: its original name
/// and a possible unique name which can be defined in case of duplicated elements. This unique name
/// could be then used to refer to a particular element in the collection.
#[derive(Debug, Clone)]
pub struct ElementBundle<Element> {
    // keep element inside this struct
    pub elem: Element,
    // keep its original name and its alternate unique name if any
    pub name: String,
}

/// Just a trick to restrict the list of admissible type for `Indexes` type parameter below.
pub trait Indexable {}
impl Indexable for usize {}
impl Indexable for Vec<usize> {}

/// Named elements collection. The `Indexes` type parameter could either by a simple `usize` index in case of non-duplicated elements,
/// or a `Vec<usize>` for storing elements having the same name. The `Indexable` trait bound is used to restrict the set
/// of admissible types: `usize` or `Vec<usize>`.
//#[derive(Clone)]
pub struct NamedElementsCollection<Element, Indexes: Indexable> {
    /// List of Element structs
    pub list: Vec<ElementBundle<Element>>,
    /// Hashmap keeping track of the name vs. index (or indexes) of the structure in the previous list
    pub hmap: HashMap<String, Indexes>,
}

impl<Element, Indexes> NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
    Indexes: Indexable,
{
    /// Creates a new empty collection of named elements.
    pub fn new() -> NamedElementsCollection<Element, Indexes> {
        NamedElementsCollection {
            // list is a vector of elements
            list: Vec::new(),

            // hmap is a hashmap of indexes of the list elements
            hmap: HashMap::new(),
        }
    }

    /// Tests whether the collection contains an item by providing its name.
    ///
    /// # Arguments
    /// * `name` - Element's name
    ///
    /// # Examples
    ///
    /// ```
    /// use nec::nec::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// assert!(molecule.contains_name("Hydrogen"));
    /// assert!(!molecule.contains_name("Helium"));
    /// ```
    pub fn contains_name(&self, name: &str) -> bool {
        self.hmap.contains_key(name)
    }

    /// Returns the reference on the element corresponding to `index`.
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
    /// use nec::nec::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// molecule.push("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule.get(0).unwrap().elem.proton,1);
    /// ```
    pub fn get(&self, index: usize) -> Option<&ElementBundle<Element>> {
        self.list.get(index)
    }

    /// Returns the mutable reference on the element corresponding to `index`.
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
    /// use nec::nec::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// molecule.push("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule.get(0).unwrap().elem.proton,1);
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut ElementBundle<Element>> {
        self.list.get_mut(index)
    }

    /// Returns the number of elements in the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use nec::nec::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// molecule.push("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule.len(),2);
    /// ```
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Tests whether the collection contains elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use nec::nec::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// assert!(!molecule.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Returns an iterator over the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use nec::nec::DNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut water = DNEC::<Atom>::new();
    ///
    /// water.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// water.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// water.push("Oxygen", Atom{ proton:8, neutron:8 });
    ///
    /// let mut iter = water.iter();
    ///
    /// assert_eq!(iter.next().unwrap().elem.proton, 1);
    /// assert_eq!(iter.next().unwrap().elem.proton, 1);
    /// assert_eq!(iter.next().unwrap().elem.proton, 8);
    /// ```
    pub fn iter(&self) -> NecIter<Element> {
        self.into_iter()
    }

    /// Returns a mutable iterator over the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use nec::nec::DNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut water = DNEC::<Atom>::new();
    ///
    /// water.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// water.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// water.push("Oxygen", Atom{ proton:7, neutron:7 });
    ///
    /// let mut iter = water.iter_mut().skip(2);
    /// let mut oxygen = iter.next().unwrap();
    ///
    /// // fix error on number of protons & neutrons
    /// oxygen.elem.proton = 8;
    /// oxygen.elem.neutron = 8;
    /// ```
    pub fn iter_mut(&mut self) -> NecIterMut<Element> {
        self.into_iter()
    }

    /// Clears the collection, leaving it empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use nec::nec::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// molecule.push("Helium", Atom{ proton:2, neutron:2 });
    ///
    /// molecule.clear();
    /// assert_eq!(molecule.len(),0);
    /// ```
    pub fn clear(&mut self) {
        self.list.clear();
        self.hmap.clear();
    }

    // pub fn insert(&mut self, index: usize, name: &str, element: Element) {
    //     // insert element at index in the vector
    //     self.list.insert(
    //         index,
    //         ElementBundle {
    //             elem: element,
    //             name: String::from(name),
    //         },
    //     );

    //     // manage indexes in the hash
    //     let name = self.list.get(index).unwrap().name.clone();
    //     self.hmap.insert_element(&name, index);

    //     e
    // }

    /// Returns the list of elements' names, without duplication.
    ///
    /// # Arguments
    /// * `name` - Element's name
    ///
    /// # Examples
    ///
    /// ```
    /// use nec::nec::DNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut water = DNEC::<Atom>::new();
    ///
    /// water.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// water.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// water.push("Oxygen", Atom{ proton:8, neutron:8 });
    /// assert_eq!(water.names().len(), 2);
    /// ```
    pub fn names(&self) -> Vec<String> {
        self.hmap.keys().map(|e| e.clone()).collect()
    }

    /// Returns the name of the element at `index`.
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
    /// use nec::nec::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// molecule.push("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule.get_name(1).unwrap(), "Helium");
    /// ```
    pub fn get_name(&self, index: usize) -> Option<&String> {
        match self.list.get(index) {
            Some(v) => Some(&v.name),
            None => None,
        }
    }

    /// Adds an item at the end of the collection.
    ///
    /// # Arguments
    /// * `name` - Element's name
    /// * `element` - Element structure
    ///
    /// # Examples
    ///
    /// ```
    /// use nec::nec::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// for i in 0..10_u8 {
    ///     molecule.push(&format!("Atom{}",i), Atom{ proton:i, neutron:i });
    /// }
    ///
    /// for i in 0..10_u8 {
    ///     molecule.push(&format!("Atom{}",i), Atom{ proton:i, neutron:i });
    /// }
    /// assert_eq!(molecule.len(), 10);
    ///
    /// for i in 0..10_u8 {
    ///     assert_eq!(molecule[i as usize].elem.proton, i);
    /// }
    pub fn push(&mut self, name: &str, element: Element) {
        match self.hmap.already_in(name) {
            // if name is already in our list, just replace the element
            Some(index) => {
                // and replace the element struct
                self.list[index] = ElementBundle {
                    elem: element,
                    name: String::from(name),
                };

                // and also replace in hmap
                self.hmap.replace_element(name, index);
            }
            // if not, just add the element
            None => {
                // add element
                self.list.push(ElementBundle {
                    elem: element,
                    name: String::from(name),
                });

                // add index in the hash
                let index = self.list.len() - 1;
                self.hmap.add_element(name, index);
            }
        }
    }


    /// Removes an element from the collection by providing its index.
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
    /// use nec::nec::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// for i in 0..10 {
    ///     molecule.push(&format!("Atom{}",i), Atom{ proton:i, neutron:i });
    /// }
    ///
    /// molecule.remove(5);
    /// assert_eq!(molecule.len(),9);
    /// assert_eq!(molecule.get_name(0).unwrap(),"Atom0");
    /// assert_eq!(molecule.get_name(8).unwrap(),"Atom9");
    /// ```
    pub fn remove(&mut self, index: usize) -> ElementBundle<Element> {
        // delete from main list
        let e = self.list.remove(index);

        // remove relevant indexes from hashmap
        let name = self.list.get(index).unwrap().name.clone();
        self.hmap.delete_element(&name, index);

        e
    }    
}

//-----------------------------------------------------------------------
// Specializations
//-----------------------------------------------------------------------
impl<Element> NamedElementsCollection<Element, Vec<usize>> {
    /// Returns a vector of elements having the same name. If `name`is not found, `None`is returned.
    ///
    /// # Arguments
    /// * `name` - Element's name
    ///
    /// # Examples
    ///
    /// ```
    /// use nec::nec::DNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut water = DNEC::<Atom>::new();
    ///
    /// water.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// water.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// water.push("Oxygen", Atom{ proton:8, neutron:8 });
    ///
    /// let mut v = water.get_by_name("Hydrogen").unwrap();
    ///
    /// assert_eq!(v.len(), 2);
    /// assert_eq!(v[0].proton, 1);
    /// assert_eq!(v[0].neutron, 0);
    /// assert_eq!(v[1].proton, 1);
    /// assert_eq!(v[1].neutron, 0);
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
    Indexes: Indexable,
{
    type Output = ElementBundle<Element>;

    /// Gets access to an element by providing its index in the collection.
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
    /// use nec::nec::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// molecule.push("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule[1].elem.proton, 2);
    /// assert_eq!(molecule[1].elem.neutron, 2);
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        // get reference on vector of items
        self.get(index).unwrap()
    }
}

impl<Element, Indexes> IndexMut<usize> for NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
    Indexes: Indexable,
{
    /// Gets access to an element by providing its index in the collection.
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
    /// use nec::nec::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// molecule.push("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule[1].elem.proton, 2);
    /// assert_eq!(molecule[1].elem.neutron, 2);
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut ElementBundle<Element> {
        // get reference on vector of items
        self.get_mut(index).unwrap()
    }
}

impl<'a, Element> Index<&'a str> for NamedElementsCollection<Element, usize> {
    type Output = ElementBundle<Element>;

    /// Gets access to an element by providing its name in the collection. This only works for
    /// non-duplicated named objets (UNEC) because this trait returns a reference and cannot work with
    /// duplicated elements.
    ///
    /// # Arguments
    /// * `name` - Element's name.
    ///
    /// # Panics
    ///
    /// Panics if `name` is not in the collection.
    ///
    /// # Examples
    /// ```
    /// use nec::nec::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut molecule = UNEC::<Atom>::new();
    ///
    /// molecule.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// molecule.push("Helium", Atom{ proton:2, neutron:2 });
    /// assert_eq!(molecule["Hydrogen"].elem.proton, 1);
    /// assert_eq!(molecule["Hydrogen"].elem.neutron, 0);
    /// ```
    fn index(&self, name: &str) -> &Self::Output {
        // get reference on vector of items
        let index = *self.hmap.get(name).unwrap();
        &self.list.get(index).unwrap()
    }
}

//-----------------------------------------------------------------------
// Iterators
//-----------------------------------------------------------------------

/// Structure helper for consuming iterator.
pub struct NecIntoIterator<Element> {
    iter: ::std::vec::IntoIter<ElementBundle<Element>>,
}

impl<Element, Indexes> IntoIterator for NamedElementsCollection<Element, Indexes>
where
    Indexes: Indexable,
{
    type Item = ElementBundle<Element>;
    type IntoIter = NecIntoIterator<Element>;

    fn into_iter(self) -> Self::IntoIter {
        NecIntoIterator {
            iter: self.list.into_iter(),
        }
    }
}

impl<Element> Iterator for NecIntoIterator<Element> {
    type Item = ElementBundle<Element>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// StruDebugcture helper for non-consuming iterator.
pub struct NecIter<'a, Element: 'a> {
    iter: Iter<'a, ElementBundle<Element>>,
}

impl<'a, Element, Indexes> IntoIterator for &'a NamedElementsCollection<Element, Indexes>
where
    Indexes: Indexable,
{
    type Item = &'a ElementBundle<Element>;
    type IntoIter = NecIter<'a, Element>;

    fn into_iter(self) -> Self::IntoIter {
        NecIter {
            iter: self.list.iter(),
        }
    }
}

impl<'a, Element> Iterator for NecIter<'a, Element> {
    type Item = &'a ElementBundle<Element>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// Structure helper for non-consuming mutable iterator.
pub struct NecIterMut<'a, Element: 'a> {
    iter: IterMut<'a, ElementBundle<Element>>,
}

impl<'a, Element, Indexes> IntoIterator for &'a mut NamedElementsCollection<Element, Indexes>
where
    Indexes: Indexable,
{
    type Item = &'a mut ElementBundle<Element>;
    type IntoIter = NecIterMut<'a, Element>;

    fn into_iter(self) -> Self::IntoIter {
        NecIterMut {
            iter: self.list.iter_mut(),
        }
    }
}

impl<'a, Element> Iterator for NecIterMut<'a, Element> {
    type Item = &'a mut ElementBundle<Element>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

//-----------------------------------------------------------------------
// Clone
//-----------------------------------------------------------------------
impl<Element: Clone, Indexes> Clone for NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
    Indexes: Indexable,
{
    /// Builds a collection clone from an original one.
    ///
    /// # Examples
    /// ```
    /// use nec::nec::DNEC;
    /// #[derive(Clone)]
    /// struct Atom { proton: u8, neutron: u8, };
    /// let mut water = DNEC::<Atom>::new();
    ///
    /// water.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// water.push("Hydrogen", Atom{ proton:1, neutron:0 });
    /// water.push("Oxygen", Atom{ proton:8, neutron:8 });
    ///
    /// let mut peroxyide = water.clone();
    /// peroxyide.push("Oxygen", Atom{ proton:8, neutron:8 });
    /// assert_eq!(peroxyide.len(), 4);
    /// ```
    fn clone(&self) -> Self {
        let mut cloned = NamedElementsCollection::<Element, Indexes>::new();

        // copy other fields which can be potentially already set
        for e in self {
            cloned.push(&e.name, e.elem.clone());
        }

        cloned
    }
}

//-----------------------------------------------------------------------
// From
//-----------------------------------------------------------------------
impl<Element, Indexes> From<Vec<(String, Element)>> for NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
    Indexes: Indexable,
{
    /// Builds a collection from a vector of `(String, Element)` tuples.
    ///
    /// # Examples
    /// ```
    /// use nec::nec::UNEC;
    ///
    /// struct Atom { proton: u8, neutron: u8, };
    ///
    /// let v: Vec<_> = (0..10).map(|i| (format!("ATOM{}",i), Atom{ proton:i, neutron:i })).collect();
    ///
    /// let molecule = UNEC::<Atom>::from(v);
    /// assert_eq!(molecule[1].elem.proton, 1);
    /// assert_eq!(molecule[9].elem.neutron, 9);
    /// ```
    fn from(source: Vec<(String, Element)>) -> Self {
        let mut collection = NamedElementsCollection::<Element, Indexes>::new();

        for e in source {
            collection.push(&e.0, e.1);
        }

        collection
    }
}

//-----------------------------------------------------------------------
// Debug
//-----------------------------------------------------------------------
impl<Element: fmt::Debug, Indexes: fmt::Debug> fmt::Debug
    for NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
    Indexes: Indexable,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("list: {:?}\n", self.list);
        s += &format!("hmap: {:?}\n", self.hmap);

        write!(f, "{}", s)
    }
}

//-----------------------------------------------------------------------
// Default
//-----------------------------------------------------------------------
impl<Element, Indexes> Default for NamedElementsCollection<Element, Indexes>
where
    HashMap<String, Indexes>: Adjustable,
    Indexes: Indexable,
{
    /// Creates an empty collection.
    ///
    /// # Examples
    /// ```
    /// use nec::nec::UNEC;
    ///
    /// let collection: UNEC::<()> = Default::default();
    /// assert!(collection.is_empty());
    /// ```
    fn default() -> Self {
        NamedElementsCollection::new()
    }
}

//-----------------------------------------------------------------------
// AsRef
//-----------------------------------------------------------------------

// type aliases
type UniqueNamedElementsCollection<Element> = NamedElementsCollection<Element, usize>;

/// Named elements collection where no name duplication is possible. Adding an element with the same name
/// just replaces the previous one.
///
/// # Examples
/// ```
/// use nec::nec::UNEC;
///
/// struct Atom { proton: u8, neutron: u8, };
/// let mut molecule = UNEC::<Atom>::new();
///
/// molecule.push("Hydrogen", Atom { proton: 1, neutron: 0, });
/// molecule.push("Hydrogen", Atom { proton: 1, neutron: 0, });
/// molecule.push("Hydrogen", Atom { proton: 1, neutron: 0, });
/// assert_eq!(molecule.get(0).unwrap().elem.neutron, 0);
///
/// // this last push is the winner
/// molecule.push("Hydrogen", Atom { proton: 1, neutron: 1, });
/// assert_eq!(molecule.len(), 1);
/// assert_eq!(molecule.get(0).unwrap().elem.neutron, 1);
/// ```
pub type UNEC<Element> = UniqueNamedElementsCollection<Element>;

type DuplicateNamedElementsCollection<Element> = NamedElementsCollection<Element, Vec<usize>>;

/// Named elements collection where name duplication is allowed. Adding an element with the same name
/// just adds an element.
///
/// # Examples
/// ```
/// use nec::nec::DNEC;
///
/// struct Atom { proton: u8, neutron: u8, };
/// let mut molecule = DNEC::<Atom>::new();
///
/// molecule.push("Hydrogen", Atom { proton: 1, neutron: 0, });
/// molecule.push("Hydrogen", Atom { proton: 1, neutron: 0, });
/// molecule.push("Hydrogen", Atom { proton: 1, neutron: 0, });
/// molecule.push("Hydrogen", Atom { proton: 1, neutron: 1, });
/// assert_eq!(molecule.len(), 4);
/// ```
pub type DNEC<Element> = DuplicateNamedElementsCollection<Element>;
