//pub mod noc1;
pub mod noc3;

#[cfg(test)]
mod tests {

    use noc::{UNOC, Nameable, NamedObjectsContainer};

    struct Atom {
        proton: u8,
        neutron: u8,
    }

    #[test]
    fn basic_test() {

        // setup data
        let mut noc = UNOC::<Atom>::new();
        assert_eq!(noc.len(), 0);

        noc.push("H", Atom{ proton:1, neutron:0 });
        noc.push("He", Atom{ proton:2, neutron:2 });
        assert_eq!(noc.len(), 2);

        assert!(noc.contains_name("H"));
        assert!(noc.contains_name("He"));
        assert!(!noc.contains_name("Cl"));

        let H = noc.get(0).unwrap();
        assert_eq!(H.proton, 1);
        assert_eq!(H.neutron, 0);

        assert!(noc.get(10).is_none());

        let hydrogen = &noc["H"];
        assert_eq!(hydrogen.proton, 1);
        assert_eq!(hydrogen.neutron, 0);    


        
    }

   
}


