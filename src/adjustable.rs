use std::collections::HashMap;

/// Trait that the underlying `hmap` field of the `NamedElementsCollection` struct must implement. Function
/// names are self-explanatory.
pub trait Adjustable {
    fn add_entry(&mut self, name: &str, index: usize);
    fn remove_entry(&mut self, name: &str, index: usize);
    fn replace_entry(&mut self, name: &str, index: usize);
}

/// Case of non-duplicated elements.
impl Adjustable for HashMap<String, usize> {
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use nec::adjustable::Adjustable;
    ///
    /// let mut h: HashMap<String, usize> = HashMap::new();
    ///
    /// for i in 0..10 {
    ///     h.add_entry(&format!("NAME{}",i), i);
    /// }
    ///
    /// assert_eq!(h.keys().len(), 10);
    /// ```
    fn add_entry(&mut self, name: &str, index: usize) {
        self.insert(name.to_string(), index);
    }

    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use nec::adjustable::Adjustable;
    ///
    /// let mut h: HashMap<String, usize> = HashMap::new();
    ///
    /// for i in 0..10 {
    ///     h.add_entry(&format!("NAME{}",i), i);
    /// }
    ///
    /// h.remove_entry("NAME5", 5);
    /// assert_eq!(h.keys().len(), 9);
    /// ```
    fn remove_entry(&mut self, name: &str, _index: usize) {
        self.remove(name);
    }

    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use nec::adjustable::Adjustable;
    ///
    /// let mut h: HashMap<String, usize> = HashMap::new();
    ///
    /// for i in 0..10 {
    ///     h.add_entry(&format!("NAME{}",i), i);
    /// }
    ///
    /// for i in 0..10 {
    ///     h.replace_entry(&format!("NAME{}",i), i+1);
    /// }
    /// ;
    /// assert_eq!(h.keys().len(), 10);
    /// assert_eq!(h.get("NAME0").unwrap(), &1);
    /// ```
    fn replace_entry(&mut self, name: &str, index: usize) {
        self.insert(name.to_string(), index);
    }
}

/// Case of duplicated elements possible.
impl Adjustable for HashMap<String, Vec<usize>> {
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use nec::adjustable::Adjustable;
    ///
    /// let mut h: HashMap<String, Vec<usize>> = HashMap::new();
    ///
    /// for j in 0..3 {
    ///     for i in 0..3 {
    ///         h.add_entry(&format!("NAME{}",i), i);
    ///     }
    /// }
    ///
    /// assert_eq!(h.keys().len(), 3);
    /// assert_eq!(h.get("NAME0").unwrap(), &vec![0,0,0]);
    /// assert_eq!(h.get("NAME1").unwrap(), &vec![1,1,1]);
    /// assert_eq!(h.get("NAME2").unwrap(), &vec![2,2,2]);
    /// ```
    fn add_entry(&mut self, name: &str, index: usize) {
        self.entry(name.to_string())
            .or_insert(Vec::new())
            .push(index);
    }

    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use nec::adjustable::Adjustable;
    ///
    /// let mut h: HashMap<String, Vec<usize>> = HashMap::new();
    ///
    /// for i in 0..3 {
    ///     for j in 0..3 {
    ///         h.add_entry(&format!("NAME{}",i), j);
    ///     }
    /// }
    ///
    /// h.remove_entry("NAME0",0);
    /// assert_eq!(h.get("NAME0").unwrap(), &vec![1,2]);
    ///
    /// h.remove_entry("NAME1",1);
    /// assert_eq!(h.get("NAME1").unwrap(), &vec![0,2]);
    ///
    /// h.remove_entry("NAME2",2);
    /// assert_eq!(h.get("NAME2").unwrap(), &vec![0,1]);
    ///
    /// h.remove_entry("NAME2",1);
    /// assert_eq!(h.get("NAME2").unwrap(), &vec![0]);
    ///
    /// // no more key after this call
    /// h.remove_entry("NAME2",0);
    /// assert!(!h.contains_key("NAME2"));
    ///
    /// ```
    fn remove_entry(&mut self, name: &str, index: usize) {
        let i = self.get_mut(name).unwrap().remove(index);
        assert_eq!(index, i);

        // if not more indexes, remove key
        if self.get(name).unwrap().is_empty() {
            self.remove(name);
        }
    }

    fn replace_entry(&mut self, name: &str, index: usize) {
        //self.insert(name.to_string(), index);
    }
}
