// examples/ellipsis.rs : demonstrates use of `Ellipsis`

#![allow(dead_code)]


use diagnosticism::diagnostics::Ellipsis;

use std::collections::{
    BTreeMap,
    BTreeSet,
    HashMap,
};



/// Large structure that provides the internals of `Thing`
#[derive(Clone)]
#[derive(Debug)]
struct LargeInternal {
    f1 : i64,
    f2 : f32,
    f3 : BTreeMap<&'static str, &'static str>,
    f4 : BTreeSet<u64>,
    f5 : Vec<HashMap<BTreeMap<i32, String>, BTreeSet<u64>>>,
}

#[derive(Debug)]
struct Thing1 {
    name :      String,
    internals : Option<LargeInternal>,
}

struct Thing2 {
    name :      String,
    internals : Option<LargeInternal>,
}

impl std::fmt::Debug for Thing2 {
    fn fmt(
        &self,
        f : &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if f.alternate() {
            f.debug_struct("Thing2")
                .field("name", &self.name)
                .field("internals", &self.internals)
                .finish()
        } else {
            let ellipsis = Ellipsis::default();

            f.debug_struct("Thing2")
                .field("name", &self.name)
                .field("internals", &ellipsis)
                .finish()
        }
    }
}


fn main() {
    let internals_clone = {
        let thing1 = Thing1 {
            name :      "i-am-a-public-thing".into(),
            internals : Some(LargeInternal {
                f1 : -123,
                f2 : f32::MIN,
                f3 : BTreeMap::from([
                    // insert list
                    ("a", "A"),
                    ("b", "B"),
                    ("c", "C"),
                    ("d", "D"),
                    ("e", "E"),
                    ("f", "F"),
                ]),
                f4 : BTreeSet::from([
                    // insert list
                    1,
                    2,
                    11,
                    12345678,
                    4,
                ]),
                f5 : vec![
                    // insert list:
                ],
            }),
        };

        eprintln!("");
        eprintln!("Terse `Debug` form of `thing1`: {thing1:?}");

        eprintln!("");
        eprintln!("Verbose `Debug` form of `thing1`: {thing1:#?}");

        thing1.internals.unwrap().clone()
    };

    {
        let thing2 = Thing2 {
            name :      "i-am-a-public-thing".into(),
            internals : Some(internals_clone),
        };

        eprintln!("");
        eprintln!("Terse `Debug` form of `thing2`: {thing2:?}");

        eprintln!("");
        eprintln!("Verbose `Debug` form of `thing2`: {thing2:#?}");
    }
}
