use std::collections::HashMap;

/// Trait that the underlying `hmap` field of the `NamedElementsCollection` struct must implement. Function
/// names are self-explanatory.
pub trait Adjustable {
    fn add_entry(&mut self, name: &str, index: usize);
    fn delete_entry(&mut self, name: &str, index: usize);
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
    /// h.delete_entry("NAME5", 5);
    /// assert_eq!(h.keys().len(), 9);
    /// ```
    fn delete_entry(&mut self, name: &str, _index: usize) {
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
    /// // set a specific data set for testing:
    /// // 0 1 2 3 4 5 6 7 8 9
    /// // A B A A B A A B B A
    /// 
    /// h.insert("A".to_string(), vec![0,2,3,5,6,9]);
    /// h.insert("B".to_string(), vec![1,4,7,8]);
    ///
    /// h.delete_entry("A",0);
    /// assert_eq!(h.get("A").unwrap(), &vec![1,2,4,5,8]);
    /// assert_eq!(h.get("B").unwrap(), &vec![0,3,6,7]);
    /// 
    /// h.delete_entry("A",2);
    /// assert_eq!(h.get("A").unwrap(), &vec![1,3,4,7]);
    /// assert_eq!(h.get("B").unwrap(), &vec![0,2,5,6]);
    /// 
    /// h.delete_entry("A",7);
    /// assert_eq!(h.get("A").unwrap(), &vec![1,3,4]);
    /// assert_eq!(h.get("B").unwrap(), &vec![0,2,5,6]);
    /// 
    /// h.delete_entry("A",3);
    /// assert_eq!(h.get("A").unwrap(), &vec![1,3]);
    /// assert_eq!(h.get("B").unwrap(), &vec![0,2,4,5]);
    /// 
    /// h.delete_entry("A",1);
    /// assert_eq!(h.get("A").unwrap(), &vec![2]);
    /// assert_eq!(h.get("B").unwrap(), &vec![0,1,3,4]);
    /// 
    /// h.delete_entry("A",2);
    /// assert!(!h.contains_key("A"));
    /// assert_eq!(h.get("B").unwrap(), &vec![0,1,2,3]);
    /// 
    /// h.delete_entry("B",0);
    /// assert_eq!(h.get("B").unwrap(), &vec![0,1,2]);
    /// 
    /// h.delete_entry("B",2);
    /// assert_eq!(h.get("B").unwrap(), &vec![0,1]);
    /// 
    /// h.delete_entry("B",0);
    /// assert_eq!(h.get("B").unwrap(), &vec![0]);
    /// 
    /// h.delete_entry("B",0);
    /// assert!(!h.contains_key("B"));
    /// ```
    fn delete_entry(&mut self, name: &str, index: usize) {
        // remove the index from list of indexes. Don't use remove_item() fn for the moment
        self.get_mut(name).unwrap().retain(|&i| i != index);

        // if not more indexes, remove key
        if self.get(name).unwrap().is_empty() {
            self.remove(name);
        }

        // for all indexes above, we need to remove 1
        for v in self.values_mut() {
            for j in v {
                if *j > index {
                    *j -= 1;
                }
            }
        }
    }

    fn replace_entry(&mut self, name: &str, index: usize) {
        //self.insert(name.to_string(), index);
    }
}
