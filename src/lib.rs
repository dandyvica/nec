pub mod noc;

#[cfg(test)]
mod tests {

    use noc::{UNOC, Nameable, Insertable, NamedObjectsContainer};

    struct Atom {
        name: String,
        proton: u8,
        neutron: u8,
    }

    impl Nameable for Atom {
        fn get_name(&self) -> String { self.name.clone() }
    }

    #[test]
    fn setup() {

        // setup data
        let mut noc = UNOC::<Atom>::new();
        assert_eq!(noc.len(), 0);

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


        
    }

   
}


