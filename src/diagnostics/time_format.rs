// src/diagnostics/time_format.rs : duration formatting

// NOTE: this work was brought in from **asynkio** via **Diagnosticism.Python**
// 0.16.0

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


/// Formats a nanosecond count as a compact human-readable duration string.
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
/// The formatted duration string. Zero is always `"0s"` with no sign.
///
/// [dp]: https://github.com/synesissoftware/Diagnosticism.Python
pub fn nanoseconds_to_string(
    nanoseconds : i64,
    format_spec : &str,
) -> String {
    let mut v = nanoseconds;

    let sign = if v < 0 {
        v = -v;

        "-"
    } else if format_spec.contains('+') {
        "+"
    } else {
        ""
    };

    if v == 0 {
        return String::from("0s");
    }

    let (oom, divisor) = scale_index(v);

    let suffix = SUFFIXES[oom / 3];

    if oom < 3 {
        return fmt(sign, v, 0, suffix);
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

    fmt(sign, whole, frac, suffix)
}


fn scale_index(n : i64) -> (usize, i64) {
    debug_assert!(n > 0);

    if n >= 100_000_000_000 {
        return (11, SCALES[11]);
    }

    let mut l = 0;
    let mut h = 11;

    let mut count = 0;

    while l <= h {
        count += 1;

        debug_assert!(count < 5);

        let m = (h + l) / 2;

        let b = SCALES[m];

        if n == b {
            return (m, b);
        }

        if n < b {
            h = m;

            continue;
        }

        debug_assert!(n > b);

        if n < b * 10 {
            return (m, b);
        }

        l = m;
    }

    (11, SCALES[11])
}


fn fmt(
    sign : &str,
    whole : i64,
    frac : i64,
    suffix : &str,
) -> String {
    if frac == 0 {
        return format!("{sign}{whole}{suffix}");
    }

    if whole > 999 {
        return format!("{sign}{whole}{suffix}");
    }

    if whole > 99 {
        return format!("{sign}{whole}.{frac}{suffix}");
    }

    if whole > 9 {
        return format!("{sign}{whole}.{frac:02}{suffix}");
    }

    format!("{sign}{whole}.{frac}{suffix}")
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
    fn TEST_zero() {
        assert_ns(0, "", "0s");
        assert_ns(0, "+", "0s");
    }


    #[test]
    fn TEST_one_second() {
        assert_ns(1_000_000_000, "", "1s");
    }


    #[test]
    fn TEST_123_milliseconds() {
        assert_ns(123_000_000, "", "123ms");
    }


    #[test]
    fn TEST_123_456_789_nanoseconds() {
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
    fn TEST_observed_edge_cases() {
        assert_ns( 999_772_000, "",  "999.7ms");
        assert_ns( 999_800_000, "",  "999.8ms");
        assert_ns( 999_974_000, "",  "999.9ms");

        assert_ns(-999_772_000, "", "-999.7ms");
        assert_ns(-999_800_000, "", "-999.8ms");
        assert_ns(-999_974_000, "", "-999.9ms");
    }


    #[rustfmt::skip]
    #[test]
    fn TEST_with_plus_sign() {
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
