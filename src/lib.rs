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

    // setup data ctors for tests
    pub fn build_with_name(n: usize) -> Vec<WithName> {
        let mut v = Vec::new();

        for i in 0..n {
            v.push(WithName {
                name: format!("NAME{}", i),
                x: i,
                y: i,
            });
        }

        v
    }

    pub fn build_without_name(n: usize) -> Vec<(String, WithoutName)> {
        let mut v = Vec::new();

        for i in 0..n {
            v.push((format!("NAME{}", i), WithoutName { x: i, y: i }));
        }

        v
    }

}

#[cfg(test)]
mod tests {

    use setup::{build_with_name, build_without_name, WithName, WithoutName};
    use noc::{NamedObjectsCollection, UNOC};
    //use adjustable::Adjustable;
    use nameable::Nameable;

    #[derive(Clone)]
    struct S1 {
        name: String,
        x: usize,
        y: usize,
    }

    impl Nameable for S1 {
        fn get_name(&self) -> &str {
            &self.name
        }
    }

    #[derive(Clone)]
    struct S2 {
        x: usize,
        y: usize,
    }

    #[test]
    fn test_unoc_with_name() {
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
        // use some iterator adaptors
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
        // contains
        //---------------------------------------------------------------------------
        assert!(noc.contains_name("NAME5"));
        assert!(!noc.contains_name("NAME100"));

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

}
