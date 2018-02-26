use std::collections::HashMap;

pub trait Adjustable {
    fn add_entry(&mut self, name: &str, index: usize);
    fn remove_entry(&mut self, name: &str, index: usize);
}
impl Adjustable for HashMap<String, usize> {
    fn add_entry(&mut self, name: &str, index: usize) {
        self.insert(name.to_string(), index);
    }

    fn remove_entry(&mut self, name: &str, _index: usize) {
        self.remove(name);
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
}
