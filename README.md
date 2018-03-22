# Named elements collection

Represents a collection containing elements which could be associated with a name.
It mimics the `Vector` collection, but with the ability to retrieve an element by its name.

Element names could also be duplicated, and getting an element providing its name could possibly return several elements.
If the `Element` type implements the `Nameable` trait, it's not necessary to provide the element's name. If not,
the name should be provided when pushing an element into the collection.

# Examples
```rust
use nec::nec::DNEC;
struct Atom { proton: u8, neutron: u8, };
let mut water = DNEC::<Atom>::new();
water.push_with_name("Hydrogen", Atom{ proton:1, neutron:0 });
water.push_with_name("Hydrogen", Atom{ proton:1, neutron:0 });
water.push_with_name("Oxygen", Atom{ proton:8, neutron:8 });
```