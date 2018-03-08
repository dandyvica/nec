pub mod noc;
pub mod adjustable;
pub mod nameable;

// setup module to build test data
mod setup {

    // setup data structures for all tests
    #[derive(Clone)]
    pub struct WithName {
        pub name: String,
        pub x: usize,
        pub y: usize,
    }

    impl ::nameable::Nameable for WithName {
        fn get_name(&self) -> &str {
            &self.name
        }
    }

    #[derive(Clone)]
    pub struct WithoutName {
        pub x: usize,
        pub y: usize,
    }

    pub fn build_unoc_data<E, V, F>(n: usize, create: F) -> Vec<V>
    where
        F: Fn(usize) -> V,
    {
        let mut v = Vec::new();

        for i in 0..n {
            v.push(create(i));
        }

        v
    }

    // setup data ctors for tests
    pub fn build_with_name(n: usize) -> Vec<WithName> {
        (0..n)
            .map(|i| WithName {
                name: format!("NAME{}", i),
                x: i,
                y: i,
            })
            .collect()
    }

    pub fn build_without_name(n: usize) -> Vec<(String, WithoutName)> {
        (0..n)
            .map(|i| (format!("NAME{}", i), WithoutName { x: i, y: i }))
            .collect()
    }

}

#[cfg(test)]
mod tests {

    use noc::{DNOC, UNOC};
    use setup::{build_with_name, build_without_name, WithName, WithoutName};

    //use adjustable::Adjustable;
    use nameable::Nameable;

    // generic setup function for data, either with or without name
    #[test]
    fn test_unoc() {
        // build test data
        let v1 = build_with_name(100);

        // initial test
        let mut noc = UNOC::<WithName>::new();
        assert_eq!(noc.len(), 0);
        assert!(noc.get(0).is_none());

        //---------------------------------------------------------------------------
        // From trait
        //---------------------------------------------------------------------------
        noc = UNOC::<WithName>::from(v1);

        for i in 0..100 {
            let s = &noc[i];
            assert_eq!(s.name, format!("NAME{}", i));
            assert_eq!(s.x, i);
            assert_eq!(s.y, i);
        }

        //---------------------------------------------------------------------------
        // Iterators
        //---------------------------------------------------------------------------
        let mut i = 0;
        for e in &noc {
            assert_eq!(e.name, format!("NAME{}", i));
            assert_eq!(e.x, i);
            assert_eq!(e.y, i);
            i += 1;
        }

        i = 0;
        for e in &mut noc {
            assert_eq!(e.name, format!("NAME{}", i));
            assert_eq!(e.x, i);
            assert_eq!(e.y, i);
            i += 1;
        }

        //---------------------------------------------------------------------------
        // use some iterator adapters
        //---------------------------------------------------------------------------
        for (i, s) in noc.iter().enumerate() {
            assert_eq!(s.name, format!("NAME{}", i));
            assert_eq!(s.x, i);
            assert_eq!(s.y, i);
        }

        {
            let v: Vec<_> = noc.iter().filter(|e| e.x % 2 == 0).collect();
            assert_eq!(v.len(), 50);
        }

        //---------------------------------------------------------------------------
        // contains, get_name()
        //---------------------------------------------------------------------------
        assert!(noc.contains_name("NAME5"));
        assert!(!noc.contains_name("NAME100"));

        assert_eq!(noc.get_name(10).unwrap().original_name, "NAME10");

        //---------------------------------------------------------------------------
        // clone
        //---------------------------------------------------------------------------
        let noc2 = noc.clone();
        assert_eq!(noc.len(), 100);

        for i in 0..100 {
            let e = noc2.get(i).unwrap();
            assert_eq!(e.name, format!("NAME{}", i));
            assert_eq!(e.x, i);
            assert_eq!(e.y, i);
        }

        //---------------------------------------------------------------------------
        // names
        //---------------------------------------------------------------------------
        let names = noc2.names();
        assert!(names.contains(&"NAME5".to_string()));
        assert!(!names.contains(&"NAME100".to_string()));

        //---------------------------------------------------------------------------
        // indexes
        //---------------------------------------------------------------------------
        {
            let mut element50 = noc.get(50).unwrap();
            assert_eq!(&element50.name, "NAME50");

            element50 = &noc[50];
            assert_eq!(&element50.name, "NAME50");
        }

        //---------------------------------------------------------------------------
        // clear
        //---------------------------------------------------------------------------
        noc.clear();
        assert_eq!(noc.len(), 0);
    }

    #[test]
    fn test_dnoc() {
        // initial test
        let mut noc = DNOC::<WithoutName>::new();
        assert_eq!(noc.len(), 0);
        assert!(noc.get(0).is_none());

        //---------------------------------------------------------------------------
        // Fill noc with duplicate data
        //---------------------------------------------------------------------------
        for i in 0..50 {
            noc.push_with_name("A", WithoutName { x: i, y: i });
        }
        for i in 50..100 {
            noc.push_with_name("B", WithoutName { x: i, y: i });
        }

        //---------------------------------------------------------------------------
        // Iterators
        //---------------------------------------------------------------------------
        let mut i = 0;
        for e in &noc {
            // assert_eq!(e.name, format!("NAME{}", i));
            assert_eq!(e.x, i);
            assert_eq!(e.y, i);
            i += 1;
        }

        i = 0;
        for e in &mut noc {
            //assert_eq!(e.name, format!("NAME{}", i));
            assert_eq!(e.x, i);
            assert_eq!(e.y, i);
            i += 1;
        }

        //---------------------------------------------------------------------------
        // use some iterator adapters
        //---------------------------------------------------------------------------
        for (i, s) in noc.iter().enumerate() {
            //assert_eq!(s.name, format!("NAME{}", i));
            assert_eq!(s.x, i);
            assert_eq!(s.y, i);
        }

        {
            let v: Vec<_> = noc.iter().filter(|e| e.x % 2 == 0).collect();
            assert_eq!(v.len(), 50);
        }

        //---------------------------------------------------------------------------
        // contains, get_name()
        //---------------------------------------------------------------------------
        assert!(noc.contains_name("A"));
        assert!(noc.contains_name("B"));
        assert!(!noc.contains_name("C"));

        assert_eq!(noc.get_name(10).unwrap().original_name, "A");

        //---------------------------------------------------------------------------
        // clone
        //---------------------------------------------------------------------------
        let noc2 = noc.clone();
        assert_eq!(noc.len(), 100);

        for i in 0..100 {
            let e = noc2.get(i).unwrap();
            //assert_eq!(noc.get_name(i).unwrap().original_name, format!("NAME{}", i));
            assert_eq!(e.x, i);
            assert_eq!(e.y, i);
        }

        //---------------------------------------------------------------------------
        // names
        //---------------------------------------------------------------------------
        let names = noc2.names();
        assert!(names.contains(&"A".to_string()));
        assert!(!names.contains(&"C".to_string()));

        //---------------------------------------------------------------------------
        // indexes
        //---------------------------------------------------------------------------
        {
            let mut element50_name = noc.get_name(50).unwrap().original_name.clone();
            assert_eq!(&element50_name, "B");
        }

        //---------------------------------------------------------------------------
        // clear
        //---------------------------------------------------------------------------
        noc.clear();
        assert_eq!(noc.len(), 0);
    }
}
