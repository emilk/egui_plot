use egui::Color32;
use egui::FontId;
use egui::Painter;

// Utility function to find a truncated candidate to fit a text label into a
// given width. If the width is large enough for the text, a string with the
// full text will be returned. If the width is too small to display the full
// text, it finds the longest text with "..." appended at the end that we can
// display within the given width. If the width is too small to display the
// first character followed by "..." then we return an empty string.
pub(crate) fn find_name_candidate(name: &str, width: f32, painter: &Painter, font_id: &FontId) -> String {
    let galley = painter.layout_no_wrap(name.to_owned(), font_id.clone(), Color32::BLACK);

    if galley.size().x <= width || name.is_empty() {
        return name.to_owned();
    }

    // If we don't have enough space for the name to be displayed in the span, we
    // search for the longest candidate that fits, where a candidate is a
    // truncated version of the name followed by "...".
    let chars: Vec<char> = name.chars().collect();

    // First test the minimum candidate which is the first letter followed by "..."
    let mut min_candidate = chars[0].to_string();
    min_candidate.push_str("...");
    let galley = painter.layout_no_wrap(min_candidate.clone(), font_id.clone(), Color32::BLACK);
    if galley.size().x > width {
        return String::new();
    }

    // Then do a binary search to find the longest possible candidate
    let mut low = 1;
    let mut high = chars.len();
    let mut best = String::new();

    while low <= high {
        let mid = usize::midpoint(low, high);
        let mut candidate: String = chars[..mid].iter().collect();
        candidate.push_str("...");

        let candidate_width = painter
            .layout_no_wrap(candidate.clone(), font_id.clone(), Color32::BLACK)
            .size()
            .x;

        if candidate_width <= width {
            best = candidate;
            low = mid + 1;
        } else {
            high = mid.saturating_sub(1);
            if high == 0 {
                break;
            }
        }
    }

    best
}

/// Initialize logging so that the testing framework can capture log output.
///
/// Call this at the top of a test function to see log output from that test.
/// The logging output will only be shown when the following conditions are met:
/// - The test fails
/// - The RUST_LOG environment variable is set to a level at or below the message level
///
/// When running a specific test:
/// ```sh
/// RUST_LOG=info cargo test auto_bounds_true
/// ```
///
/// If the something causes the test to panic so hard that it nevers shows logging output,
/// you can use `--nocapture` to see log output as it happens:
/// ```sh
/// RUST_LOG=info cargo test auto_bounds_true -- --nocapture
/// ```
#[allow(dead_code)] // It's ok if it's not currently being used. We just want it to be available.
#[cfg(test)]
pub(crate) fn init_test_logger() {
    use std::io::Write as _;
    let _ = env_logger::builder()
        .is_test(true)
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());
            writeln!(
                buf,
                "[{level_style}{}{level_style:#} {}:{}]   {}",
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .try_init();
}
