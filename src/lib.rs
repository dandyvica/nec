pub mod adjustable;
pub mod nec;

// setup module to build test data
mod setup {

    // setup data structures for all tests
    #[derive(Clone)]
    pub struct Atom {
        pub proton: usize,
        pub neutron: usize,
    }

}

#[cfg(test)]
mod tests {

    use nec::{DNEC, UNEC};
    use setup::Atom;

    //use adjustable::Adjustable;
    //use nameable::Nameable;

    // generic setup function for data, either with or without name
    #[test]
    fn test_unec() {
        // build test data
        let v1: Vec<_> = (0..100)
            .map(|i| {
                (
                    format!("NAME{}", i),
                    Atom {
                        proton: i,
                        neutron: i,
                    },
                )
            })
            .collect();

        // initial test
        let mut nec = UNEC::<Atom>::new();
        assert_eq!(nec.len(), 0);
        assert!(nec.get(0).is_none());

        //---------------------------------------------------------------------------
        // From trait
        //---------------------------------------------------------------------------
        nec = UNEC::<Atom>::from(v1);

        for i in 0..100 {
            let s = &nec[i];
            assert_eq!(s.name, format!("NAME{}", i));
            assert_eq!(s.elem.proton, i);
            assert_eq!(s.elem.neutron, i);
        }

        //---------------------------------------------------------------------------
        // Iterators
        //---------------------------------------------------------------------------
        let mut i = 0;
        for e in &nec {
            assert_eq!(e.name, format!("NAME{}", i));
            assert_eq!(e.elem.proton, i);
            assert_eq!(e.elem.neutron, i);
            i += 1;
        }

        i = 0;
        for e in &mut nec {
            assert_eq!(e.name, format!("NAME{}", i));
            assert_eq!(e.elem.proton, i);
            assert_eq!(e.elem.neutron, i);
            i += 1;
        }

        //---------------------------------------------------------------------------
        // use some iterator adapters
        //---------------------------------------------------------------------------
        for (i, s) in nec.iter().enumerate() {
            assert_eq!(s.name, format!("NAME{}", i));
            assert_eq!(s.elem.proton, i);
            assert_eq!(s.elem.neutron, i);
        }

        {
            let v: Vec<_> = nec.iter().filter(|e| e.elem.proton % 2 == 0).collect();
            assert_eq!(v.len(), 50);
        }

        //---------------------------------------------------------------------------
        // contains, get_name()
        //---------------------------------------------------------------------------
        assert!(nec.contains_name("NAME5"));
        assert!(!nec.contains_name("NAME100"));

        assert_eq!(nec.get_name(10).unwrap(), "NAME10");

        //---------------------------------------------------------------------------
        // clone
        //---------------------------------------------------------------------------
        let nec2 = nec.clone();
        assert_eq!(nec.len(), 100);

        for i in 0..100 {
            let e = nec2.get(i).unwrap();
            assert_eq!(e.name, format!("NAME{}", i));
            assert_eq!(e.elem.proton, i);
            assert_eq!(e.elem.neutron, i);
        }

        //---------------------------------------------------------------------------
        // names
        //---------------------------------------------------------------------------
        let names = nec2.names();
        assert!(names.contains(&"NAME5".to_string()));
        assert!(!names.contains(&"NAME100".to_string()));

        //---------------------------------------------------------------------------
        // indexes
        //---------------------------------------------------------------------------
        {
            let mut element50 = nec.get(50).unwrap();
            assert_eq!(&element50.name, "NAME50");

            element50 = &nec[50];
            assert_eq!(&element50.name, "NAME50");
        }

        //---------------------------------------------------------------------------
        // clear
        //---------------------------------------------------------------------------
        nec.clear();
        assert_eq!(nec.len(), 0);
    }

    #[test]
    fn test_dnec() {
        // initial test
        let mut nec = DNEC::<Atom>::new();
        assert_eq!(nec.len(), 0);
        assert!(nec.get(0).is_none());

        //---------------------------------------------------------------------------
        // Fill nec with duplicate data
        //---------------------------------------------------------------------------
        for i in 0..50 {
            nec.push(
                "A",
                Atom {
                    proton: i,
                    neutron: i,
                },
            );
        }
        for i in 50..100 {
            nec.push(
                "B",
                Atom {
                    proton: i,
                    neutron: i,
                },
            );
        }

        //---------------------------------------------------------------------------
        // Iterators
        //---------------------------------------------------------------------------
        let mut i = 0;
        for e in &nec {
            // assert_eq!(e.name, format!("NAME{}", i));
            assert_eq!(e.elem.proton, i);
            assert_eq!(e.elem.neutron, i);
            i += 1;
        }

        i = 0;
        for e in &mut nec {
            //assert_eq!(e.name, format!("NAME{}", i));
            assert_eq!(e.elem.proton, i);
            assert_eq!(e.elem.neutron, i);
            i += 1;
        }

        //---------------------------------------------------------------------------
        // use some iterator adapters
        //---------------------------------------------------------------------------
        for (i, s) in nec.iter().enumerate() {
            //assert_eq!(s.name, format!("NAME{}", i));
            assert_eq!(s.elem.proton, i);
            assert_eq!(s.elem.neutron, i);
        }

        {
            let v: Vec<_> = nec.iter().filter(|e| e.elem.proton % 2 == 0).collect();
            assert_eq!(v.len(), 50);
        }

        //---------------------------------------------------------------------------
        // contains, get_name()
        //---------------------------------------------------------------------------
        assert!(nec.contains_name("A"));
        assert!(nec.contains_name("B"));
        assert!(!nec.contains_name("C"));

        assert_eq!(nec.get_name(10).unwrap(), "A");

        //---------------------------------------------------------------------------
        // clone
        //---------------------------------------------------------------------------
        let nec2 = nec.clone();
        assert_eq!(nec.len(), 100);

        for i in 0..100 {
            let e = nec2.get(i).unwrap();
            //assert_eq!(nec.get_name(i).unwrap().original_name, format!("NAME{}", i));
            assert_eq!(e.elem.proton, i);
            assert_eq!(e.elem.neutron, i);
        }

        //---------------------------------------------------------------------------
        // names
        //---------------------------------------------------------------------------
        let names = nec2.names();
        assert!(names.contains(&"A".to_string()));
        assert!(!names.contains(&"C".to_string()));

        //---------------------------------------------------------------------------
        // get_name()
        //---------------------------------------------------------------------------
        {
            let element50_name = nec.get_name(50).unwrap().clone();
            assert_eq!(&element50_name, "B");
        }

        //---------------------------------------------------------------------------
        // remove
        //---------------------------------------------------------------------------
        assert_eq!(nec.get_name(49).unwrap(), "A");
        nec.remove(0);
        assert_eq!(nec.len(), 99);

        // test several locations
        assert_eq!(nec.get_name(0).unwrap(), "A");
        assert_eq!(nec.get_name(49).unwrap(), "B");

        //---------------------------------------------------------------------------
        // clear
        //---------------------------------------------------------------------------
        nec.clear();
        assert_eq!(nec.len(), 0);
    }
}
