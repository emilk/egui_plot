//! Helper functions for logarithmic scale formatting.

use crate::grid::GridMark;

/// Helper function to convert a positive number to superscript characters.
fn to_superscript(num: u32) -> String {
    // FIXME(asceenl) This is a quick-and-dirty implementation, that allocates
    // twice. It could be implemented as an iterator without allocating.
    let s = num.to_string();
    s.chars()
        .map(|c| match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            _ => c,
        })
        .collect()
}

/// Splits a float into its mantissa and exponent parts. For any normal number,
/// the absolute value of the mantissa is in the range `[1, 10)`, and the
/// exponent is an integer.
///
/// Edge cases:
///  - If the value is zero, returns `(0.0, 0)`.
///  - For NaN, returns `(NaN, 0)`.
///  - For infinities, returns `(+/- Inf, 0)`.
#[inline]
fn split_float(value: f64) -> (f64, i16) {
    if value == 0.0 {
        return (0.0, 0);
    }
    let abs = value.abs();
    if !abs.is_finite() {
        // Handle NaN and infinity
        return (value, 0);
    }
    let log_abs = abs.log10();
    let exponent = log_abs.floor() as i16;
    let mantissa = value.signum() * 10.0_f64.powf(log_abs - exponent as f64);

    (mantissa, exponent)
}

/// Returns a formatter for logarithmic axes that displays values as powers with
/// superscripts.
///
/// The problem is that Egui does not render a superscript minus
/// sign, resulting in a fallback here to use 1/10², etc. I do not like this,
/// but it is the best I could come up with.
///
/// - Positive exponents: `10⁰`, `10¹`, `10²`, `10³`, etc.
/// - Negative exponents: `1/10¹`, `1/10²`, `1/10³`, etc.
///
/// # Example
/// ```ignore
/// Plot::new("my_plot")
///     .log_y(10.0)
///     .y_axis_formatter(log_formatter_superscript())
///     .show(ui, |plot_ui| { /* ... */ });
/// ```
pub fn log_formatter_superscript() -> impl Fn(GridMark, &std::ops::RangeInclusive<f64>) -> String {
    move |mark, _range| {
        let value = mark.value;

        const BASE_INT: i32 = 10;

        // Calculate the mantissa and exponent, with absolute value of mantissa
        // normalized to the range of [1, 10)
        let (mantissa, exponent) = split_float(value);

        // Round the mantissa to the nearest integer
        let mantissa_rounded = mantissa.round();

        // Check if it's a clean power of the base (mantissa ≈ 1, or mantissa ≈ 10)
        let mantissa_close_to_1 = (mantissa - 1.0).abs() < 0.01;
        let mantissa_close_to_10 = (mantissa - 10.0).abs() < 0.01;
        let clean_power = mantissa_close_to_1 || mantissa_close_to_10;

        if clean_power {
            // Clean power cases, e.g. 10^2, 10^3, 1/10^1, 1/10^2
            match (exponent, mantissa_close_to_1, mantissa_close_to_10) {
                // Special cases for readability, do not show 10^0, just 1
                (0, true, false) | (-1, false, true) => "1".into(),
                // Special cases for readability, do not show 10^1, just 10
                (1, true, false) | (0, false, true) => "10".into(),
                // Negative exponent: 1/10¹, 1/10²
                (e, _, _) if e < 0 => format!("1/{}{}", BASE_INT, to_superscript((-e) as u32)),
                // Positive exponent: 10², 10³, etc.
                (e, _, _) => format!("{BASE_INT}{}", to_superscript(e as u32)),
            }
        } else if mantissa_rounded >= 2.0 && mantissa_rounded <= 9.0 && (mantissa - mantissa_rounded).abs() < 0.1 {
            // It's a simple multiple of a power (2×10⁴, 3×10⁴, etc.)
            let mult_int = mantissa_rounded as i32;

            if exponent >= 0 {
                if exponent == 0 {
                    // Just show the number itself for 10⁰
                    mantissa_rounded.to_string()
                } else {
                    // Positive exponent: 2×10⁴, 3×10⁴
                    format!("{mult_int}×{BASE_INT}{}", to_superscript(exponent as u32))
                }
            } else {
                // Negative exponent: 2/10², 3/10²
                format!("{mult_int}×1/{BASE_INT}{}", to_superscript((-exponent) as u32))
            }
        } else {
            // For intermediate values (like 2.5×10² = 250), try to format nicely
            // Check if it's a simple decimal multiple (2.5, 3.5, etc.)
            let decimal_mult = (mantissa * 2.0).round() / 2.0; // Round to nearest 0.5

            if (mantissa - decimal_mult).abs() < 0.05 && decimal_mult >= 1.5 && decimal_mult <= 9.5 && exponent >= 0 {
                // It's close to a half-integer multiple
                if exponent == 0 {
                    // Just show the decimal number
                    if (decimal_mult - decimal_mult.round()).abs() < 0.01 {
                        format!("{}", decimal_mult.round() as i32)
                    } else {
                        format!("{decimal_mult:.1}")
                    }
                } else {
                    // Show as decimal multiple: 2.5×10²
                    if (decimal_mult - decimal_mult.round()).abs() < 0.01 {
                        format!(
                            "{}×{}{}",
                            decimal_mult.round() as i32,
                            BASE_INT,
                            to_superscript(exponent as u32)
                        )
                    } else {
                        format!("{:.1}×{}{}", decimal_mult, BASE_INT, to_superscript(exponent as u32))
                    }
                }
            } else {
                // Not a simple multiple, fall back to default formatting
                crate::label::format_number(value, 2)
            }
        }
    }
}

/// Returns a formatter for logarithmic axes that displays values in
/// computer-scientific notation.
///
/// The format is easy to type, but is not the way
/// humans have learned to write numbers in scientific notation.
///
/// Example output: `1e0`, `1e1`, `1e2`, `1e3`, etc.
///
/// # Example
/// ```ignore
/// Plot::new("my_plot")
///     .log_y(10.0)
///     .y_axis_formatter(log_formatter_scientific())
///     .show(ui, |plot_ui| { /* ... */ });
/// ```
pub fn log_formatter_computer() -> impl Fn(GridMark, &std::ops::RangeInclusive<f64>) -> String {
    move |mark, _range| {
        let value = mark.value;
        format!("{value:e}")
    }
}

/// Returns a formatter for logarithmic axes that displays values in compact
/// form with suffixes, also known as "engineering notation", or Metric prefix.
///
/// Example output: `1`, `10`, `100`, `1K`, `10K`, `100K`, `1M`, `10M`, etc.
///
/// # Example
/// ```ignore
/// Plot::new("my_plot")
///     .log_y(10.0)
///     .y_axis_formatter(log_formatter_compact())
///     .show(ui, |plot_ui| { /* ... */ });
/// ```
pub fn log_formatter_engineering() -> impl Fn(GridMark, &std::ops::RangeInclusive<f64>) -> String {
    move |mark, _range| {
        let value = mark.value;
        let abs_value = value.abs();

        let (scaled, suffix) = if abs_value >= 1e30 {
            // Quetta
            (value / 1e30, "Q")
        } else if abs_value >= 1e27 {
            // Ronna
            (value / 1e27, "R")
        } else if abs_value >= 1e24 {
            // Yotta
            (value / 1e24, "Y")
        } else if abs_value >= 1e21 {
            // Zetta
            (value / 1e21, "Z")
        } else if abs_value >= 1e18 {
            // Exa
            (value / 1e18, "E")
        } else if abs_value >= 1e15 {
            // Peta
            (value / 1e15, "P")
        } else if abs_value >= 1e12 {
            // Tera
            (value / 1e12, "T")
        } else if abs_value >= 1e9 {
            // Giga
            (value / 1e9, "G")
        } else if abs_value >= 1e6 {
            // Mega
            (value / 1e6, "M")
        } else if abs_value >= 1e3 {
            // Kilo
            (value / 1e3, "k")
        } else if abs_value >= 1.0 {
            (value, "")
        } else if abs_value >= 1e-3 {
            // milli
            (value * 1e3, "m")
        } else if abs_value >= 1e-6 {
            // micro
            (value * 1e6, "µ")
        } else if abs_value >= 1e-9 {
            // nano
            (value * 1e9, "n")
        } else if abs_value >= 1e-12 {
            // pico
            (value * 1e12, "p")
        } else if abs_value >= 1e-15 {
            // femto
            (value * 1e15, "f")
        } else if abs_value >= 1e-18 {
            // atto
            (value * 1e18, "a")
        } else if abs_value >= 1e-21 {
            // zepto
            (value * 1e21, "z")
        } else if abs_value >= 1e-24 {
            // yocto
            (value * 1e24, "y")
        } else if abs_value >= 1e-27 {
            // ronto
            (value * 1e27, "r")
        } else if abs_value >= 1e-30 {
            // quecto
            (value * 1e30, "q")
        } else {
            (value, "")
        };

        // Format with minimal decimal places
        // if scaled.fract().abs() < 0.01 {
        //     format!("{}{}", scaled.round() as i64, suffix)
        // } else if scaled.abs() >= 10.0 {
        //     format!("{scaled:.0}{suffix}")
        // } else {
        //     format!("{scaled}{suffix}")
        // }
        format!("{scaled}{suffix}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_float_roundtrip() {
        // Test that splitting and reconstructing gives back the original value
        let test_values = vec![
            // Positive values
            1.0, 10.0, 123.45, 1234.5, 0.12345, 0.0012345, 1e10, 1e-10, // Negative values
            -1.0, -10.0, -123.45, -1234.5, -0.12345, -0.0012345, -1e10, -1e-10, // Edge cases
            0.0, -0.0, -9.999999,
        ];

        for value in test_values {
            let (mantissa, exponent) = split_float(value);
            let reconstructed = mantissa * 10.0_f64.powi(exponent as i32);

            if value == 0.0 {
                // Special case: zero
                assert_eq!(mantissa, 0.0, "Zero mantissa should be 0.0");
                assert_eq!(exponent, 0, "Zero exponent should be 0");
                assert_eq!(reconstructed, 0.0, "Reconstructed zero should be 0.0");
            } else {
                // Check that mantissa is in range [1, 10) or (-10, -1] for negative
                let abs_mantissa = mantissa.abs();
                assert!(
                    abs_mantissa >= 1.0 && abs_mantissa < 10.0,
                    "Mantissa {mantissa} should be in range [1, 10) for value {value}",
                );

                // Check sign preservation
                assert_eq!(
                    mantissa.is_sign_positive(),
                    value.is_sign_positive(),
                    "Sign should be preserved: value={value}, mantissa={mantissa}"
                );

                // Check roundtrip (with some tolerance for floating point errors)
                let relative_error = ((reconstructed - value) / value).abs();
                assert!(
                    relative_error < 1e-10,
                    "Roundtrip failed for {value}: got {reconstructed}, error={relative_error}"
                );
            }
        }
    }

    #[test]
    fn test_split_float_special_values() {
        // Test NaN
        let (mantissa, exponent) = split_float(f64::NAN);
        assert!(mantissa.is_nan(), "NaN mantissa should be NaN");
        assert_eq!(exponent, 0, "NaN exponent should be 0");

        // Test positive infinity
        let (mantissa, exponent) = split_float(f64::INFINITY);
        assert_eq!(mantissa, f64::INFINITY, "Infinity mantissa should be Infinity");
        assert_eq!(exponent, 0, "Infinity exponent should be 0");

        // Test negative infinity
        let (mantissa, exponent) = split_float(f64::NEG_INFINITY);
        assert_eq!(
            mantissa,
            f64::NEG_INFINITY,
            "Negative infinity mantissa should be -Infinity"
        );
        assert_eq!(exponent, 0, "Negative infinity exponent should be 0");
    }

    #[test]
    fn test_log_formatter_superscript_powers_of_10() {
        let formatter = log_formatter_superscript();
        let range = 1.0..=1e10;

        // Test clean powers of 10 - these should all format appropriately
        let test_cases = vec![
            (1.0, "1"),   // Special case: just "1"
            (10.0, "10"), // Special case: just "10"
            (100.0, "10²"),
            (1_000.0, "10³"), // Previously failed due to FP error
            (10_000.0, "10⁴"),
            (100_000.0, "10⁵"),
            (1_000_000.0, "10⁶"), // Previously failed due to FP error
            (10_000_000.0, "10⁷"),
            (100_000_000.0, "10⁸"),
            (1_000_000_000.0, "10⁹"), // Previously failed due to FP error
        ];

        for (value, expected) in test_cases {
            let mark = GridMark { value, step_size: 10.0 };
            let result = formatter(mark, &range);
            assert_eq!(
                result, expected,
                "Failed for value {value}: got '{result}', expected '{expected}'"
            );
        }
    }

    #[test]
    fn test_log_formatter_superscript_multiples() {
        let formatter = log_formatter_superscript();
        let range = 1.0..=1e10;

        // Test simple multiples like 2×10², 5×10³, etc.
        let test_cases = vec![
            (2.0, "2"),
            (5.0, "5"),
            (20.0, "2×10¹"),
            (50.0, "5×10¹"),
            (200.0, "2×10²"),
            (5000.0, "5×10³"),
        ];

        for (value, expected) in test_cases {
            let mark = GridMark { value, step_size: 1.0 };
            let result = formatter(mark, &range);
            assert_eq!(
                result, expected,
                "Failed for value {value}: got '{result}', expected '{expected}'"
            );
        }
    }

    #[test]
    fn test_log_formatter_superscript_negative_exponents() {
        let formatter = log_formatter_superscript();
        let range = 1e-10..=1.0;

        // Test negative exponents - should use 1/10ⁿ notation
        let test_cases = vec![(0.1, "1/10¹"), (0.01, "1/10²"), (0.001, "1/10³"), (0.0001, "1/10⁴")];

        for (value, expected) in test_cases {
            let mark = GridMark { value, step_size: 10.0 };
            let result = formatter(mark, &range);
            assert_eq!(
                result, expected,
                "Failed for value {value}: got '{result}', expected '{expected}'"
            );
        }
    }
}
