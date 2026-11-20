// src/diagnostics/time_format/format.rs : `nanoseconds_to_string()`

use super::nanoseconds_str::NanosecondsStr;


const SCALES : [i64; 12] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
];

const SUFFIXES : [&str; 4] = [
    "ns",
    "µs",
    "ms",
    "s",
];


// API functions

/// Formats a nanosecond count as a compact human-readable duration string,
/// returning a [`NanosecondsStr`].
///
/// The output adapts the unit (`ns`, `µs`, `ms`, `s`) and decimal precision
/// to keep roughly three significant digits in the numeric portion.
///
/// Behaviour matches [`Diagnosticism.Python`][dp] 0.16.0
/// `nanoseconds_to_string()`.
///
/// # Parameters
///
/// * `nanoseconds` — the duration, in nanoseconds;
/// * `format_spec` — formatting options; the only recognised flag is `+`,
///   which causes positive values to include an explicit leading sign;
///   other characters are ignored;
///
/// # Returns
///
/// A [`NanosecondsStr`] holding the formatted duration. Zero is always
/// `"0s"` with no sign.
///
/// [dp]: https://github.com/synesissoftware/Diagnosticism.Python
pub fn nanoseconds_to_string(
    nanoseconds : i64,
    format_spec : &str,
) -> NanosecondsStr {
    let mut v = nanoseconds;

    let sign_byte = if v < 0 {
        v = -v;

        Some(b'-')
    } else if format_spec.contains('+') {
        Some(b'+')
    } else {
        None
    };

    if v == 0 {
        return NanosecondsStr::from_buffer(b"0s");
    }

    let (oom, divisor) = scale_index(v);

    let suffix = SUFFIXES[oom / 3];

    if oom < 3 {
        return format_parts(sign_byte, v, 0, suffix);
    }

    let divisor_0 = divisor / 1_000;

    let i = oom % 3;

    let divisor_1 = if i == 0 {
        1_000
    } else if i == 1 {
        100
    } else {
        10
    };

    v /= divisor_0;

    let whole = v / divisor_1;
    let frac = v - (whole * divisor_1);

    format_parts(sign_byte, whole, frac, suffix)
}


// Helper functions

fn scale_index(n : i64) -> (usize, i64) {
    debug_assert!(n > 0);

    let oom = if n >= 100_000_000_000 {
        11
    } else {
        n.ilog10() as usize
    };

    (oom, SCALES[oom])
}


fn format_parts(
    sign_byte : Option<u8>,
    whole : i64,
    frac : i64,
    suffix : &str,
) -> NanosecondsStr {
    let mut buf = [0u8; 24];
    let mut pos = 0usize;

    if let Some(b) = sign_byte {
        buf[pos] = b;

        pos += 1;
    }

    pos = write_u64(&mut buf, whole as u64, pos);

    if frac != 0 && whole <= 999 {
        buf[pos] = b'.';

        pos += 1;

        if whole > 99 {
            pos = write_u64(&mut buf, frac as u64, pos);
        } else if whole > 9 {
            pos = write_frac_min_width_2(&mut buf, pos, frac);
        } else {
            pos = write_u64(&mut buf, frac as u64, pos);
        }
    }

    pos = write_bytes(&mut buf, pos, suffix.as_bytes());

    NanosecondsStr::from_buffer(&buf[..pos])
}


fn write_u64(
    buf : &mut [u8],
    mut n : u64,
    mut pos : usize,
) -> usize {
    if n == 0 {
        buf[pos] = b'0';

        return pos + 1;
    }

    let start = pos;

    while n > 0 {
        buf[pos] = (n % 10) as u8 + b'0';

        n /= 10;

        pos += 1;
    }

    buf[start..pos].reverse();

    pos
}


fn write_frac_min_width_2(
    buf : &mut [u8],
    pos : usize,
    frac : i64,
) -> usize {
    debug_assert!((0..100).contains(&frac));

    if frac >= 10 {
        write_u64(buf, frac as u64, pos)
    } else {
        buf[pos] = b'0';
        buf[pos + 1] = (frac as u8) + b'0';

        pos + 2
    }
}


fn write_bytes(
    buf : &mut [u8],
    pos : usize,
    bytes : &[u8],
) -> usize {
    buf[pos..pos + bytes.len()].copy_from_slice(bytes);

    pos + bytes.len()
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::nanoseconds_to_string;


    fn assert_ns(
        nanoseconds : i64,
        format_spec : &str,
        expected : &str,
    ) {
        assert_eq!(
            expected,
            nanoseconds_to_string(nanoseconds, format_spec),
        );
    }


    #[test]
    fn TEST_ZERO() {
        assert_ns(0, "", "0s");
        assert_ns(0, "+", "0s");
    }


    #[test]
    fn TEST_ONE_SECOND() {
        assert_ns(1_000_000_000, "", "1s");
    }


    #[test]
    fn TEST_123_MILLISECONDS() {
        assert_ns(123_000_000, "", "123ms");
    }


    #[test]
    fn TEST_123_456_789_NANOSECONDS() {
        assert_ns(123_456_789, "", "123.4ms");
    }


    #[test]
    fn TEST_STRINGS() {
        #[rustfmt::skip]
        let cases = [
            (                     0, "0s"),
            (                     9, "9ns"),
            (                    89, "89ns"),
            (                   789, "789ns"),
            (                 6_789, "6.789µs"),
            (                56_789, "56.78µs"),
            (               456_789, "456.7µs"),
            (             3_456_789, "3.456ms"),
            (            23_456_789, "23.45ms"),
            (           123_456_789, "123.4ms"),
            (         9_123_456_789, "9.123s"),
            (        89_123_456_789, "89.12s"),
            (       789_123_456_789, "789.1s"),
            (                    80, "80ns"),
            (                   700, "700ns"),
            (                 6_000, "6µs"),
            (                50_000, "50µs"),
            (               400_000, "400µs"),
            (             3_000_000, "3ms"),
            (            20_000_000, "20ms"),
            (           100_000_000, "100ms"),
            (         9_000_000_000, "9s"),
            (        10_000_000_000, "10s"),
            (       200_000_000_000, "200s"),
            (     3_000_000_000_000, "3000s"),
            (    40_000_000_000_000, "40000s"),
            (   500_000_000_000_000, "500000s"),
            ( 6_000_000_000_000_000, "6000000s"),
            (70_000_000_000_000_000, "70000000s"),
            (        11_111_111_111, "11.11s"),
            (       222_222_222_222, "222.2s"),
            (     3_333_333_333_333, "3333s"),
            (    44_444_444_444_444, "44444s"),
            (   555_555_555_555_555, "555555s"),
            ( 6_666_666_666_666_666, "6666666s"),
            (77_777_777_777_777_777, "77777777s"),
        ];

        for (nanoseconds, expected) in cases {
            assert_ns(nanoseconds, "", expected);
        }
    }


    #[test]
    fn TEST_NEGATIVE_VALUES_STRINGS() {
        #[rustfmt::skip]
        let cases = [
            (                 -9, "-9ns"),
            (                -89, "-89ns"),
            (               -789, "-789ns"),
            (             -6_789, "-6.789µs"),
            (            -56_789, "-56.78µs"),
            (           -456_789, "-456.7µs"),
            (         -3_456_789, "-3.456ms"),
            (        -23_456_789, "-23.45ms"),
            (       -123_456_789, "-123.4ms"),
            (     -9_123_456_789, "-9.123s"),
            (                -80, "-80ns"),
            (               -700, "-700ns"),
            (             -6_000, "-6µs"),
            (            -50_000, "-50µs"),
            (           -400_000, "-400µs"),
            (         -3_000_000, "-3ms"),
            (        -20_000_000, "-20ms"),
            (       -100_000_000, "-100ms"),
            (     -9_000_000_000, "-9s"),
            (    -10_000_000_000, "-10s"),
            (   -200_000_000_000, "-200s"),
            ( -3_000_000_000_000, "-3000s"),
            (-40_000_000_000_000, "-40000s"),
        ];

        for (nanoseconds, expected) in cases {
            assert_ns(nanoseconds, "", expected);
        }
    }


    #[rustfmt::skip]
    #[test]
    fn TEST_OBSERVED_EDGE_CASES() {
        assert_ns( 999_772_000, "",  "999.7ms");
        assert_ns( 999_800_000, "",  "999.8ms");
        assert_ns( 999_974_000, "",  "999.9ms");

        assert_ns(-999_772_000, "", "-999.7ms");
        assert_ns(-999_800_000, "", "-999.8ms");
        assert_ns(-999_974_000, "", "-999.9ms");
    }


    #[rustfmt::skip]
    #[test]
    fn TEST_WITH_PLUS_SIGN() {
        assert_ns( 999_772_000, "",  "999.7ms");
        assert_ns( 999_800_000, "",  "999.8ms");
        assert_ns( 999_974_000, "",  "999.9ms");

        assert_ns(-999_772_000, "", "-999.7ms");
        assert_ns(-999_800_000, "", "-999.8ms");
        assert_ns(-999_974_000, "", "-999.9ms");

        assert_ns( 999_772_000, "",  "999.7ms");
        assert_ns( 999_800_000, "",  "999.8ms");
        assert_ns( 999_974_000, "",  "999.9ms");

        assert_ns( 999_772_000, "+", "+999.7ms");
        assert_ns( 999_800_000, "+", "+999.8ms");
        assert_ns( 999_974_000, "+", "+999.9ms");

        assert_ns(-999_772_000, "+", "-999.7ms");
        assert_ns(-999_800_000, "+", "-999.8ms");
        assert_ns(-999_974_000, "+", "-999.9ms");
    }
}


// ///////////////////////////// end of file //////////////////////////// //
