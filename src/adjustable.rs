use std::collections::HashMap;

pub trait Adjustable {
    fn add_entry(&mut self, name: &str, index: usize);
    fn remove_entry(&mut self, name: &str, index: usize);
    fn replace_entry(&mut self, name: &str, index: usize);
}

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

impl Adjustable for HashMap<String, Vec<usize>> {
    fn add_entry(&mut self, name: &str, index: usize) {
        self.entry(name.to_string())
            .or_insert(Vec::new())
            .push(index);
    }

    fn remove_entry(&mut self, name: &str, index: usize) {
        let i = self.get_mut(name).unwrap().remove(index);
        //let i = indexes.remove(index);
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
