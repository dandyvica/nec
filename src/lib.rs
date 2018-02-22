pub mod noc;

#[cfg(test)]
mod tests {

    use noc::{Insertable, Nameable, NamedObjectsContainer, UNOC};

    #[derive(Clone)]
    struct S {
        name: String,
        x: usize,
        y: usize,
    }

    impl Nameable for S {
        fn get_name(&self) -> &str {
            &self.name
        }
    }

    fn setup_data(n: usize) -> Vec<S> {
        let mut v = Vec::new();

        for i in 0..n {
            v.push(S {
                name: format!("NAME{}", i),
                x: i,
                y: i,
            });
        }

        v
    }

    #[test]
    fn setup() {
        // build test data
        let mut v = setup_data(100);

        // initial test
        let mut noc = UNOC::<S>::new();
        assert_eq!(noc.len(), 0);
        assert!(noc.get(0).is_none());

        // From trait
        noc = UNOC::<S>::from(v);

        for i in 0..100 {
            let s = &noc[i];
            assert_eq!(s.name, format!("NAME{}", i));
            assert_eq!(s.x, i);
            assert_eq!(s.y, i);
        }

       // Clone
        let noc2 = noc.clone();

        // Iterators
        for (i, s) in noc.into_iter().enumerate() {
            assert_eq!(s.name, format!("NAME{}", i));
            assert_eq!(s.x, i);
            assert_eq!(s.y, i);
        }

 

        assert!(noc2.contains_name("NAME5"));
        assert!(!noc2.contains_name("NAME100"));

        for i in 0..100 {
            let s = noc2.get(i).unwrap();
            assert_eq!(s.name, format!("NAME{}", i));
            assert_eq!(s.x, i);
            assert_eq!(s.y, i);
        }

        let names = noc2.names();
        assert!(names.contains(&"NAME5".to_string()));
        assert!(!names.contains(&"NAME100".to_string()));


        /*
        noc.push(Atom{ name:"H".to_string(), proton:1, neutron:0 });
        noc.push(Atom{ name:"He".to_string(), proton:2, neutron:2 });
        assert_eq!(noc.len(), 2);

        assert!(noc.contains_name("H"));
        assert!(noc.contains_name("He"));
        assert!(!noc.contains_name("Cl"));

        let H = noc.get(0).unwrap();
        assert_eq!(H.proton, 1);
        assert_eq!(H.neutron, 0);

        assert!(noc.get(10).is_none());

        let hydrogen = noc.get_by_index(0).unwrap();
        assert_eq!(hydrogen.proton, 1);
        assert_eq!(hydrogen.neutron, 0); 
*/
    }

}
