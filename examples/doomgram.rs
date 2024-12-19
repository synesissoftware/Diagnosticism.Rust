// examples/doomgram.rs : example program illustrating use of `DoomGram`

use diagnosticism::diagnostics::DoomGram;

use rand::{
    rngs::StdRng,
    RngCore,
    SeedableRng,
};

use std::{
    thread,
    time::{
        Duration,
        Instant,
    },
};


fn main() {
    let mut rng = <StdRng as SeedableRng>::seed_from_u64(123456789);

    let mut dg = DoomGram::default();

    // do a warmup loop first
    for w in 0..2 {
        for i in 0..20000 {
            let mut v = rng.next_u32() % 1000000;

            if 0 == i % 10 {
                if 0 != i {
                    v = v % i;
                }
            }

            let before = Instant::now();

            if 0 != i % 2000 {
                thread::sleep(Duration::from_nanos(v as u64));
            } else {
                // no wait, so should be very low ns

                thread::sleep(Duration::from_secs(0));
            }

            let after = Instant::now();

            dg.push_event_duration(after - before);
        }

        // output results on second run through
        if 1 == w {
            let before = Instant::now();
            let strip = dg.to_strip();
            let after = Instant::now();

            eprintln!("`#to_strip()` : {strip} (in {:?})", after - before);
            eprintln!("");
            eprintln!("dg={dg:#?}");
        }

        dg.clear();
    }
}
