// src/diagnostics/doomgram.rs : `DoomGram`

use std::time::Duration;
use std::str as std_str;


// TODO: move into a grams-utils place

pub(crate) mod gram_utils {

    pub fn calc_doom(v : u64) -> u32 {

        if v >= 100000000 {
        //    return count_decimal_digits(v);
        } else {
            if v >= 10000 {
                if v >= 1000000 {
                    if v >= 10000000 {
                        return 8;
                    } else {
                        return 7;
                    }
                } else {
                    if v >= 100000 {
                        return 6;
                    } else {
                        return 5;
                    }
                }
            } else {
                if v >= 100 {
                    if v >= 1000 {
                        return 4;
                    } else {
                        return 3;
                    }
                } else {
                    if v >= 10 {
                        return 2;
                    } else {
                        if v > 0 {
                            return 1;
                        } else {
                            return 0;
                        }
                    }
                }
            }
        }

        if 0 == v {
            0
        } else {
            let mut r = 0;
            let mut v = v;

            while 0 != v {
                v /= 10;
                r += 1;
            }

            r
        }
    }

    pub fn gram_doom_to_char(
        doom : u32,
        ch_0 : u8,
        ch_overflow : u8,
        range : &[u8],
    ) -> u8 {
        if 0 == doom {
            ch_0
        } else {
            if doom as usize > range.len() {
                ch_overflow
            } else {
                range[doom as usize - 1]
            }
        }
    }
}


/// Decimal Order-Of-Magnitude frequency histoGRAM
///
/// # Note:
/// This is a Rust port of the equivalent `stlsoft::doomgram` class from the
/// **STLSoft** libraries (https://github.com/synesissoftware/STLSoft-1.11).
#[derive(Debug)]
#[derive(Default)]
pub struct DoomGram {

    event_count         :   usize,

    event_time_total    :   u64,
    has_overflowed      :   bool,

    min_event_time      :   Option<u64>,
    max_event_time      :   Option<u64>,

    num_events_in_1ns   :   u64,
    num_events_in_10ns  :   u64,
    num_events_in_100ns :   u64,
    num_events_in_1us   :   u64,
    num_events_in_10us  :   u64,
    num_events_in_100us :   u64,
    num_events_in_1ms   :   u64,
    num_events_in_10ms  :   u64,
    num_events_in_100ms :   u64,
    num_events_in_1s    :   u64,
    num_events_in_10s   :   u64,
    num_events_ge_100s  :   u64,
}

// API functions

impl DoomGram {
}

// Mutating methods

impl DoomGram {
    /// Clears the instance, resetting all values to the equivalent of a
    /// newly constructed instance.
    pub fn clear(&mut self)
    {
        *self = Default::default();
    }

    /// Pushes an event with the given [`Duration`].
    ///
    /// # Note:
    /// The value obtained from `Duration#as_nanos()` is truncated to `u64`.
    pub fn push_event_duration(
        &mut self,
        duration : Duration,
    )
    {
        self.push_event_time_ns(duration.as_nanos() as u64);
    }

    /// Pushes an event with the given number of nanoseconds.
    pub fn push_event_time_ns(
        &mut self,
        time_in_ns : u64,
    ) -> bool {
        if self.try_add_ns_to_total_and_update_minmax_and_count_(time_in_ns) {

            self.event_count += 1;

            self.push_event_time_ns_(time_in_ns);

            true
        } else {
            false
        }
    }
    /// Pushes an event with the given number of microseconds.
    pub fn push_event_time_us(
        &mut self,
        time_in_us : u64,
    ) -> bool {
        let time_in_ns = 1000 * time_in_us;

        if self.try_add_ns_to_total_and_update_minmax_and_count_(time_in_ns) {

            self.event_count += 1;

            self.push_event_time_ns_(time_in_ns);

            true
        } else {
            false
        }
    }
    /// Pushes an event with the given number of milliseconds.
    pub fn push_event_time_ms(
        &mut self,
        time_in_ms : u64,
    ) -> bool {
        let time_in_ns = 1000 * 1000 * time_in_ms;

        if self.try_add_ns_to_total_and_update_minmax_and_count_(time_in_ns) {

            self.event_count += 1;

            self.push_event_time_ns_(time_in_ns);

            true
        } else {
            false
        }
    }
    /// Pushes an event with the given number of seconds.
    pub fn push_event_time_s(
        &mut self,
        time_in_s : u64,
    ) -> bool {
        let time_in_ns = 1000 * 1000 * 1000 * time_in_s;

        if self.try_add_ns_to_total_and_update_minmax_and_count_(time_in_ns) {

            self.event_count += 1;

            self.push_event_time_ns_(time_in_ns);

            true
        } else {
            false
        }
    }
}

// Non-mutating methods

impl DoomGram {
    /// Number of events counted
    pub fn event_count(&self) -> usize {
        self.event_count
    }

    pub fn event_time_total(&self) -> Option<u64> {
        if self.has_overflowed {
            None
        } else {
            Some(self.event_time_total)
        }
    }
    /// Obtains the total event time (in nanoseconds), regardless of whether
    /// overflow has occurred.
    pub fn event_time_total_raw(&self) -> u64 {
        self.event_time_total
    }

    /// Indicates whether overflow has occurred.
    pub fn has_overflowed(&self) -> bool {
        self.has_overflowed
    }

    pub fn min_event_time(&self) -> Option<u64> {
        self.min_event_time
    }
    pub fn max_event_time(&self) -> Option<u64> {
        self.max_event_time
    }

    /// Number of events counted in the interval [1ns, 10ns).
    pub fn num_events_in_1ns(&self) -> u64 {
        self.num_events_in_1ns
    }
    /// Number of events counted in the interval [10ns, 100ns).
    pub fn num_events_in_10ns(&self) -> u64 {
        self.num_events_in_10ns
    }
    /// Number of events counted in the interval [100ns, 1µs).
    pub fn num_events_in_100ns(&self) -> u64 {
        self.num_events_in_100ns
    }

    /// Number of events counted in the interval [1µs, 10µs).
    pub fn num_events_in_1us(&self) -> u64 {
        self.num_events_in_1us
    }
    /// Number of events counted in the interval [10µs, 100µs).
    pub fn num_events_in_10us(&self) -> u64 {
        self.num_events_in_10us
    }
    /// Number of events counted in the interval [100µs, 1ms).
    pub fn num_events_in_100us(&self) -> u64 {
        self.num_events_in_100us
    }

    /// Number of events counted in the interval [1ms, 10ms).
    pub fn num_events_in_1ms(&self) -> u64 {
        self.num_events_in_1ms
    }
    /// Number of events counted in the interval [10ms, 100ms).
    pub fn num_events_in_10ms(&self) -> u64 {
        self.num_events_in_10ms
    }
    /// Number of events counted in the interval [100ms, 1s).
    pub fn num_events_in_100ms(&self) -> u64 {
        self.num_events_in_100ms
    }

    /// Number of events counted in the interval [1s, 10s).
    pub fn num_events_in_1s(&self) -> u64 {
        self.num_events_in_1s
    }
    /// Number of events counted in the interval [10s, 100s).
    pub fn num_events_in_10s(&self) -> u64 {
        self.num_events_in_10s
    }
    /// Number of events counted in the interval [100s, ∞).
    pub fn num_events_ge_100s(&self) -> u64 {
        self.num_events_ge_100s
    }

    pub fn to_strip(&self) -> String {

        // TODO: this may need to be optimised, as costing around 20µs (2µs
        // release) to process (and it's not helped by a faster
        // `calc_doom()`):
        //
        // - initialise as `[ b'_', b'_', b'_', ... ]` and then don't to call `gram_doom_to_char()` unless count != 0;
        // - have fields in an array rather than separate fields and then can iterate;
        // - T.B.D.

        use gram_utils::{
            calc_doom,
            gram_doom_to_char,
        };

        let mut strip : [u8; 12] = Default::default();

        let ch_0 = b'_';
        let ch_overflow = b'*';
        let range = b"abcdefghijklmnopqrstuvwxyz";

        strip[0] = gram_doom_to_char(calc_doom(self.num_events_in_1ns()), ch_0, ch_overflow, range);
        strip[1] = gram_doom_to_char(calc_doom(self.num_events_in_10ns()), ch_0, ch_overflow, range);
        strip[2] = gram_doom_to_char(calc_doom(self.num_events_in_100ns()), ch_0, ch_overflow, range);
        strip[3] = gram_doom_to_char(calc_doom(self.num_events_in_1us()), ch_0, ch_overflow, range);
        strip[4] = gram_doom_to_char(calc_doom(self.num_events_in_10us()), ch_0, ch_overflow, range);
        strip[5] = gram_doom_to_char(calc_doom(self.num_events_in_100us()), ch_0, ch_overflow, range);
        strip[6] = gram_doom_to_char(calc_doom(self.num_events_in_1ms()), ch_0, ch_overflow, range);
        strip[7] = gram_doom_to_char(calc_doom(self.num_events_in_10ms()), ch_0, ch_overflow, range);
        strip[8] = gram_doom_to_char(calc_doom(self.num_events_in_100ms()), ch_0, ch_overflow, range);
        strip[9] = gram_doom_to_char(calc_doom(self.num_events_in_1s()), ch_0, ch_overflow, range);
        strip[10] = gram_doom_to_char(calc_doom(self.num_events_in_10s()), ch_0, ch_overflow, range);
        strip[11] = gram_doom_to_char(calc_doom(self.num_events_ge_100s()), ch_0, ch_overflow, range);

        let s = unsafe { std_str::from_utf8_unchecked(&strip) };

        s.into()
    }
}

// Implementation

impl DoomGram {
    fn push_event_time_ns_(
        &mut self,
        time_in_ns : u64,
    ) {
        if time_in_ns >= 1000000 {
            // >= 1ms

            if time_in_ns >= 1000000000 {
                // >= 1s

                if time_in_ns >= 100000000000 {
                    self.num_events_ge_100s += 1;
                } else if time_in_ns >= 10000000000 {
                    self.num_events_in_10s += 1;
                } else {
                    self.num_events_in_1s += 1;
                }
            } else {
                // < 1s

                if time_in_ns >= 100000000 {
                    self.num_events_in_100ms += 1;
                } else if time_in_ns >= 10000000 {
                    self.num_events_in_10ms += 1;
                } else {
                    self.num_events_in_1ms += 1;
                }
            }
        } else {
            // < 1ms

            if time_in_ns >= 1000 {
                // >= 1µs

                if time_in_ns >= 100000 {
                    self.num_events_in_100us += 1;
                } else if time_in_ns >= 10000 {
                    self.num_events_in_10us += 1;
                } else {
                    self.num_events_in_1us += 1;
                }
            } else {
                // < 1µs

                if time_in_ns >= 100 {
                    self.num_events_in_100ns += 1;
                } else if time_in_ns >= 10 {
                    self.num_events_in_10ns += 1;
                } else if time_in_ns >= 1 {
                    self.num_events_in_1ns += 1;
                }
            }
        }
    }

    fn try_add_ns_to_total_and_update_minmax_and_count_(
        &mut self,
        time_in_ns : u64
    ) -> bool {

        if self.has_overflowed {
            return false;
        }

        match self.event_time_total.checked_add(time_in_ns) {
            Some(new_total) => {

                self.event_time_total = new_total;

                match self.min_event_time {
                    Some(min_event_time) => {
                        if time_in_ns < min_event_time {
                            self.min_event_time = Some(time_in_ns);
                        }
                    },
                    None => {
                        self.min_event_time = Some(time_in_ns);
                    }
                };

                match self.max_event_time {
                    Some(max_event_time) => {
                        if time_in_ns > max_event_time {
                            self.max_event_time = Some(time_in_ns);
                        }
                    },
                    None => {
                        self.max_event_time = Some(time_in_ns);
                    }
                };

                true
            },
            None => {

                self.has_overflowed = true;

                false
            },
        }
    }
}

// Trait implementations

// NONE DEFINED AT THIS TIME


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::DoomGram;

    #[test]
    fn TEST_DoomGram_Default() {

        let dg = DoomGram::default();

        assert_eq!(0, dg.event_count());

        assert_eq!(Some(0), dg.event_time_total());
        assert_eq!(0, dg.event_time_total_raw());

        assert!(!dg.has_overflowed());

        assert_eq!(None, dg.min_event_time());
        assert_eq!(None, dg.max_event_time());

        assert_eq!(0, dg.num_events_in_1ns());
        assert_eq!(0, dg.num_events_in_10ns());
        assert_eq!(0, dg.num_events_in_100ns());
        assert_eq!(0, dg.num_events_in_1us());
        assert_eq!(0, dg.num_events_in_10us());
        assert_eq!(0, dg.num_events_in_100us());
        assert_eq!(0, dg.num_events_in_1ms());
        assert_eq!(0, dg.num_events_in_10ms());
        assert_eq!(0, dg.num_events_in_100ms());
        assert_eq!(0, dg.num_events_in_1s());
        assert_eq!(0, dg.num_events_in_10s());
        assert_eq!(0, dg.num_events_ge_100s());

        assert_eq!("____________", dg.to_strip());
    }

    #[test]
    fn TEST_DoomGram_SINGLE_TIMING_EVENT() {

        let mut dg = DoomGram::default();

        dg.push_event_time_ms(13);

        assert_eq!(1, dg.event_count());

        assert_eq!(Some(13000000), dg.event_time_total());
        assert_eq!(13000000, dg.event_time_total_raw());

        assert!(!dg.has_overflowed());

        assert_eq!(Some(13000000), dg.min_event_time());
        assert_eq!(Some(13000000), dg.max_event_time());

        assert_eq!(0, dg.num_events_in_1ns());
        assert_eq!(0, dg.num_events_in_10ns());
        assert_eq!(0, dg.num_events_in_100ns());
        assert_eq!(0, dg.num_events_in_1us());
        assert_eq!(0, dg.num_events_in_10us());
        assert_eq!(0, dg.num_events_in_100us());
        assert_eq!(0, dg.num_events_in_1ms());
        assert_eq!(1, dg.num_events_in_10ms());
        assert_eq!(0, dg.num_events_in_100ms());
        assert_eq!(0, dg.num_events_in_1s());
        assert_eq!(0, dg.num_events_in_10s());
        assert_eq!(0, dg.num_events_ge_100s());

        assert_eq!("_______a____", dg.to_strip());
    }

    #[test]
    fn TEST_doomgram_ZERO_TIME_EVENTS() {

        let mut dg = DoomGram::default();

        dg.push_event_time_ns(0);
        dg.push_event_time_us(0);
        dg.push_event_time_ms(0);
        dg.push_event_time_s(0);

        assert_eq!(4, dg.event_count());

        assert_eq!(Some(0), dg.event_time_total());
        assert_eq!(0, dg.event_time_total_raw());

        assert!(!dg.has_overflowed());

        assert_eq!(Some(0), dg.min_event_time());
        assert_eq!(Some(0), dg.max_event_time());

        assert_eq!(0, dg.num_events_in_1ns());
        assert_eq!(0, dg.num_events_in_10ns());
        assert_eq!(0, dg.num_events_in_100ns());
        assert_eq!(0, dg.num_events_in_1us());
        assert_eq!(0, dg.num_events_in_10us());
        assert_eq!(0, dg.num_events_in_100us());
        assert_eq!(0, dg.num_events_in_1ms());
        assert_eq!(0, dg.num_events_in_10ms());
        assert_eq!(0, dg.num_events_in_100ms());
        assert_eq!(0, dg.num_events_in_1s());
        assert_eq!(0, dg.num_events_in_10s());
        assert_eq!(0, dg.num_events_ge_100s());

        assert_eq!("____________", dg.to_strip());
    }

    #[test]
    fn TEST_doomgram_UNIFORM_SPREAD_TIMINGS_1() {

        let mut dg = DoomGram::default();

        dg.push_event_time_ns(  9);
        dg.push_event_time_ns( 80);
        dg.push_event_time_ns(700);
        dg.push_event_time_us(  6);
        dg.push_event_time_us( 50);
        dg.push_event_time_us(400);
        dg.push_event_time_ms(  3);
        dg.push_event_time_ms( 20);
        dg.push_event_time_ms(100);
        dg.push_event_time_s(   9);
        dg.push_event_time_s(  80);
        dg.push_event_time_s( 700);

        assert_eq!(12, dg.event_count());

        assert_eq!(Some(789123456789), dg.event_time_total());
        assert_eq!(789123456789, dg.event_time_total_raw());

        assert!(!dg.has_overflowed());

        assert_eq!(Some(9), dg.min_event_time());
        assert_eq!(Some(700000000000), dg.max_event_time());

        assert_eq!(1, dg.num_events_in_1ns());
        assert_eq!(1, dg.num_events_in_10ns());
        assert_eq!(1, dg.num_events_in_100ns());
        assert_eq!(1, dg.num_events_in_1us());
        assert_eq!(1, dg.num_events_in_10us());
        assert_eq!(1, dg.num_events_in_100us());
        assert_eq!(1, dg.num_events_in_1ms());
        assert_eq!(1, dg.num_events_in_10ms());
        assert_eq!(1, dg.num_events_in_100ms());
        assert_eq!(1, dg.num_events_in_1s());
        assert_eq!(1, dg.num_events_in_10s());
        assert_eq!(1, dg.num_events_ge_100s());

        assert_eq!("aaaaaaaaaaaa", dg.to_strip());
    }

    #[test]
    fn TEST_doomgram_UNIFORM_SPREAD_TIMINGS_2() {

        let mut dg = DoomGram::default();

        dg.push_event_time_ns(     9);
        dg.push_event_time_ns(    80);
        dg.push_event_time_ns(   700);
        dg.push_event_time_ns(  6000);
        dg.push_event_time_ns( 50000);
        dg.push_event_time_ns(400000);
        dg.push_event_time_ms(     3);
        dg.push_event_time_ms(    20);
        dg.push_event_time_ms(   100);
        dg.push_event_time_ms(  9000);
        dg.push_event_time_ms( 80000);
        dg.push_event_time_ms(700000);

        assert_eq!(12, dg.event_count());

        assert_eq!(Some(789123456789), dg.event_time_total());
        assert_eq!(789123456789, dg.event_time_total_raw());

        assert!(!dg.has_overflowed());

        assert_eq!(Some(9), dg.min_event_time());
        assert_eq!(Some(700000000000), dg.max_event_time());

        assert_eq!(1, dg.num_events_in_1ns());
        assert_eq!(1, dg.num_events_in_10ns());
        assert_eq!(1, dg.num_events_in_100ns());
        assert_eq!(1, dg.num_events_in_1us());
        assert_eq!(1, dg.num_events_in_10us());
        assert_eq!(1, dg.num_events_in_100us());
        assert_eq!(1, dg.num_events_in_1ms());
        assert_eq!(1, dg.num_events_in_10ms());
        assert_eq!(1, dg.num_events_in_100ms());
        assert_eq!(1, dg.num_events_in_1s());
        assert_eq!(1, dg.num_events_in_10s());
        assert_eq!(1, dg.num_events_ge_100s());

        assert_eq!("aaaaaaaaaaaa", dg.to_strip());
    }

    #[test]
    fn TEST_doomgram_UNIFORM_SPREAD_TIMINGS_3() {

        let mut dg = DoomGram::default();

        dg.push_event_time_ns(           9);
        dg.push_event_time_ns(          80);
        dg.push_event_time_ns(         700);
        dg.push_event_time_ns(        6000);
        dg.push_event_time_ns(       50000);
        dg.push_event_time_ns(      400000);
        dg.push_event_time_ns(     3000000);
        dg.push_event_time_ns(    20000000);
        dg.push_event_time_ns(   100000000);
        dg.push_event_time_ns(  9000000000);
        dg.push_event_time_ns( 80000000000);
        dg.push_event_time_ns(700000000000);

        assert_eq!(12, dg.event_count());

        assert_eq!(Some(789123456789), dg.event_time_total());
        assert_eq!(789123456789, dg.event_time_total_raw());

        assert!(!dg.has_overflowed());

        assert_eq!(Some(9), dg.min_event_time());
        assert_eq!(Some(700000000000), dg.max_event_time());

        assert_eq!(1, dg.num_events_in_1ns());
        assert_eq!(1, dg.num_events_in_10ns());
        assert_eq!(1, dg.num_events_in_100ns());
        assert_eq!(1, dg.num_events_in_1us());
        assert_eq!(1, dg.num_events_in_10us());
        assert_eq!(1, dg.num_events_in_100us());
        assert_eq!(1, dg.num_events_in_1ms());
        assert_eq!(1, dg.num_events_in_10ms());
        assert_eq!(1, dg.num_events_in_100ms());
        assert_eq!(1, dg.num_events_in_1s());
        assert_eq!(1, dg.num_events_in_10s());
        assert_eq!(1, dg.num_events_ge_100s());

        assert_eq!("aaaaaaaaaaaa", dg.to_strip());
    }

    #[test]
    fn TEST_doomgram_UNIFORM_SPREAD_TIMINGS_4() {

        let mut dg = DoomGram::default();

        dg.push_event_time_us(        6);
        dg.push_event_time_us(       50);
        dg.push_event_time_us(      400);
        dg.push_event_time_us(     3000);
        dg.push_event_time_us(    20000);
        dg.push_event_time_us(   100000);
        dg.push_event_time_us(  9000000);
        dg.push_event_time_us( 80000000);
        dg.push_event_time_us(700000000);

        assert_eq!(9, dg.event_count());

        assert_eq!(Some(789123456000), dg.event_time_total());
        assert_eq!(789123456000, dg.event_time_total_raw());

        assert!(!dg.has_overflowed());

        assert_eq!(Some(6000), dg.min_event_time());
        assert_eq!(Some(700000000000), dg.max_event_time());

        assert_eq!(0, dg.num_events_in_1ns());
        assert_eq!(0, dg.num_events_in_10ns());
        assert_eq!(0, dg.num_events_in_100ns());
        assert_eq!(1, dg.num_events_in_1us());
        assert_eq!(1, dg.num_events_in_10us());
        assert_eq!(1, dg.num_events_in_100us());
        assert_eq!(1, dg.num_events_in_1ms());
        assert_eq!(1, dg.num_events_in_10ms());
        assert_eq!(1, dg.num_events_in_100ms());
        assert_eq!(1, dg.num_events_in_1s());
        assert_eq!(1, dg.num_events_in_10s());
        assert_eq!(1, dg.num_events_ge_100s());

        assert_eq!("___aaaaaaaaa", dg.to_strip());
    }

    #[test]
    fn TEST_doomgram_SEVERAL_DISTINCT_TIMINGS() {

        let mut dg = DoomGram::default();

        dg.push_event_time_ns(23);
        dg.push_event_time_ns(10);
        dg.push_event_time_us(7);
        dg.push_event_time_us(7);
        dg.push_event_time_us(89);
        dg.push_event_time_ms(248);
        dg.push_event_time_s(5);
        dg.push_event_time_s(309);

        assert_eq!(8, dg.event_count());

        assert_eq!(Some(314248103033), dg.event_time_total());
        assert_eq!(314248103033, dg.event_time_total_raw());

        assert!(!dg.has_overflowed());

        assert_eq!(Some(10), dg.min_event_time());
        assert_eq!(Some(309000000000), dg.max_event_time());

        assert_eq!(0, dg.num_events_in_1ns());
        assert_eq!(2, dg.num_events_in_10ns());
        assert_eq!(0, dg.num_events_in_100ns());
        assert_eq!(2, dg.num_events_in_1us());
        assert_eq!(1, dg.num_events_in_10us());
        assert_eq!(0, dg.num_events_in_100us());
        assert_eq!(0, dg.num_events_in_1ms());
        assert_eq!(0, dg.num_events_in_10ms());
        assert_eq!(1, dg.num_events_in_100ms());
        assert_eq!(1, dg.num_events_in_1s());
        assert_eq!(0, dg.num_events_in_10s());
        assert_eq!(1, dg.num_events_ge_100s());

        assert_eq!("_a_aa___aa_a", dg.to_strip());
    }

    #[test]
    fn TEST_doomgram_SEVERAL_INTERSECTING_TIMINGS() {

        let mut dg = DoomGram::default();

        dg.push_event_time_ns(11);
        dg.push_event_time_ns(19);
        dg.push_event_time_ns(19);
        dg.push_event_time_us(7);
        dg.push_event_time_us(7);
        dg.push_event_time_us(89);
        dg.push_event_time_ms(248);
        dg.push_event_time_ms(4321);
        dg.push_event_time_s(5);
        dg.push_event_time_s(309);

        assert_eq!(10, dg.event_count());

        assert_eq!(Some(318569103049), dg.event_time_total());
        assert_eq!(318569103049, dg.event_time_total_raw());

        assert!(!dg.has_overflowed());

        assert_eq!(Some(11), dg.min_event_time());
        assert_eq!(Some(309000000000), dg.max_event_time());

        assert_eq!(0, dg.num_events_in_1ns());
        assert_eq!(3, dg.num_events_in_10ns());
        assert_eq!(0, dg.num_events_in_100ns());
        assert_eq!(2, dg.num_events_in_1us());
        assert_eq!(1, dg.num_events_in_10us());
        assert_eq!(0, dg.num_events_in_100us());
        assert_eq!(0, dg.num_events_in_1ms());
        assert_eq!(0, dg.num_events_in_10ms());
        assert_eq!(1, dg.num_events_in_100ms());
        assert_eq!(2, dg.num_events_in_1s());
        assert_eq!(0, dg.num_events_in_10s());
        assert_eq!(1, dg.num_events_ge_100s());

        assert_eq!("_a_aa___aa_a", dg.to_strip());
    }

    #[test]
    fn TEST_doomgram_OVERFLOW_BY_SECONDS() {

        let mut dg = DoomGram::default();

        // u64 max:
        //
        // 18,446,744,073,709,551,615 ns
        //     18,446,744,073,709,551 µs
        //         18,446,744,073,709 ms
        //             18,446,744,073  s

        // 18446744073 max

        // add in max # seconds
        {
            let r = dg.push_event_time_s(18446744073);

            assert!(r);
        }

        // add in 0 more
        {
            let r = dg.push_event_time_s(0);

            assert!(r);
        }

        // add in 1 more to overflow
        {
            let r = dg.push_event_time_s(1);

            assert!(!r);
        }
    }

    #[test]
    fn TEST_doomgram_OVERFLOW_BY_MICROSECONDS() {

        let mut dg = DoomGram::default();

        // u64 max:
        //
        // 18,446,744,073,709,551,615 ns
        //     18,446,744,073,709,551 µs
        //         18,446,744,073,709 ms
        //             18,446,744,073  s

        // 18446744073 max

        // add in max-1 # microseconds
        {
            let r = dg.push_event_time_us(18446744073709550);

            assert!(r);
        }

        // add in 1 more to max
        {
            let r = dg.push_event_time_us(1);

            assert!(r);
        }

        // add in 0 more
        {
            let r = dg.push_event_time_us(0);

            assert!(r);
        }

        // add in 1 more to overflow
        {
            let r = dg.push_event_time_us(1);

            assert!(!r);
        }
    }
}
