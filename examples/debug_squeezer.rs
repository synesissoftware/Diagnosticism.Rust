// examples/debug_squeezer.rs : example program illustrating use of `DebugSqueezer`

#![allow(dead_code)]

// Illustrates the use of [`DebugSqueezer`].

use diagnosticism::diagnostics::{
    DebugSqueezer,
};

use std::{
    collections::{
        BTreeMap,
        HashMap,
    },
    fmt as std_fmt,
};


type NestedMap = BTreeMap<i32, HashMap<i32, i32>>;

fn make_maps() -> (
    BTreeMap<i32, NestedMap>, // btm
    HashMap<i32, NestedMap>,  // hm
) {
    let mut btm = BTreeMap::new();
    let mut hm = HashMap::new();

    btm.insert(
        0,
        NestedMap::from([
            // insert list:
            (
                1,
                HashMap::from([
                    // insert list:
                    (2, 3),
                    (4, 5),
                ]),
            ),
            (
                6,
                HashMap::from([
                    // insert list:
                    (7, 8),
                    (9, 10),
                ]),
            ),
        ]),
    );
    hm.insert(
        11,
        NestedMap::from([
            // insert list:
            (
                12,
                HashMap::from([
                    // insert list:
                    (13, 14),
                    (15, 16),
                ]),
            ),
            (
                17,
                HashMap::from([
                    // insert list:
                    (18, 19),
                    (20, 21),
                ]),
            ),
        ]),
    );

    (btm, hm)
}

#[derive(Debug)]
struct WithoutSqueezer {
    btm : BTreeMap<i32, NestedMap>,
    hm :  HashMap<i32, NestedMap>,
}

impl WithoutSqueezer {
    pub fn new() -> Self {
        let (btm, hm) = make_maps();

        Self {
            btm,
            hm,
        }
    }
}

#[derive(Default)]
struct WithSqueezer {
    btm :           BTreeMap<i32, NestedMap>,
    hm :            HashMap<i32, NestedMap>,
    squeeze_width : usize,
}

impl WithSqueezer {
    fn new(squeeze_width : usize) -> Self {
        let (btm, hm) = make_maps();

        Self {
            btm,
            hm,
            squeeze_width,
        }
    }
}

/*
fn touch_formatter<'a>(
    f: &std_fmt::Formatter<'_>,
) -> &'static str {
    ""
}
 */

impl std_fmt::Debug for WithSqueezer {
    fn fmt(
        &self,
        f : &mut std_fmt::Formatter<'_>,
    ) -> std_fmt::Result {
        f.debug_struct("WithSqueezer")
            .field(
                "btm",
                &DebugSqueezer::new(&self.btm, self.squeeze_width),
            )
            .field(
                "hm",
                &DebugSqueezer::new(&self.hm, self.squeeze_width),
            )
            .finish()
    }
}


fn main() {
    {
        let wo_sqz = WithoutSqueezer::new();

        println!("`Debug` form of `WithoutSqueezer` is '{wo_sqz:?}'");
    }

    {
        let squeeze_width = 50;
        let w_sqz = WithSqueezer::new(squeeze_width);

        println!("`Debug` form of `WithSqueezer` ({squeeze_width}) is '{w_sqz:?}'");
    }

    {
        let squeeze_width = 20;
        let w_sqz = WithSqueezer::new(squeeze_width);

        println!("`Debug` form of `WithSqueezer` ({squeeze_width}) is '{w_sqz:?}'");
    }

    {
        let squeeze_width = 10;
        let w_sqz = WithSqueezer::new(squeeze_width);

        println!("`Debug` form of `WithSqueezer` ({squeeze_width}) is '{w_sqz:?}'");
    }

    {
        let squeeze_width = 8;
        let w_sqz = WithSqueezer::new(squeeze_width);

        println!("`Debug` form of `WithSqueezer` ({squeeze_width}) is '{w_sqz:?}'");
    }

    {
        let squeeze_width = 7;
        let w_sqz = WithSqueezer::new(squeeze_width);

        println!("`Debug` form of `WithSqueezer` ({squeeze_width}) is '{w_sqz:?}'");
    }

    {
        let squeeze_width = 6;
        let w_sqz = WithSqueezer::new(squeeze_width);

        println!("`Debug` form of `WithSqueezer` ({squeeze_width}) is '{w_sqz:?}'");
    }

    {
        let squeeze_width = 5;
        let w_sqz = WithSqueezer::new(squeeze_width);

        println!("`Debug` form of `WithSqueezer` ({squeeze_width}) is '{w_sqz:?}'");
    }

    {
        let squeeze_width = 4;
        let w_sqz = WithSqueezer::new(squeeze_width);

        println!("`Debug` form of `WithSqueezer` ({squeeze_width}) is '{w_sqz:?}'");
    }

    {
        let squeeze_width = 3;
        let w_sqz = WithSqueezer::new(squeeze_width);

        println!("`Debug` form of `WithSqueezer` ({squeeze_width}) is '{w_sqz:?}'");
    }

    {
        let squeeze_width = 2;
        let w_sqz = WithSqueezer::new(squeeze_width);

        println!("`Debug` form of `WithSqueezer` ({squeeze_width}) is '{w_sqz:?}'");
    }
}
