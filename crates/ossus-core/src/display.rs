//! Rendering of untrusted values inside diagnostics.
//!
//! A diagnostic about a hostile file is read by a human, usually in a terminal,
//! while they decide whether to admit that file. Bounding only the *length* of
//! the offending value is not enough: a manifest key or enum value may carry
//! ANSI escape sequences that repaint the screen, carriage returns that erase
//! the preceding text, newlines that forge additional diagnostic lines, or
//! bidirectional overrides that reverse how the value reads. The value must be
//! bounded in length *and* rendered inert before it reaches any output stream.

use std::fmt::Write as _;

/// Number of source characters preserved from an untrusted value.
const LIMIT: usize = 64;

/// Characters that are invisible or reorder surrounding text without being
/// classified as control characters by `char::is_control`.
///
/// This is the Trojan Source set (CVE-2021-42574) — bidirectional overrides and
/// isolates — together with the zero-width and line/paragraph separators. It is
/// a deliberately fixed list rather than a Unicode-category lookup, because no
/// Unicode table crate may enter this workspace for a display concern.
const fn is_invisible_or_reordering(character: char) -> bool {
    matches!(
        character,
        '\u{200B}'..='\u{200F}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202A}'..='\u{202E}'
            | '\u{2066}'..='\u{2069}'
            | '\u{FEFF}'
    )
}

/// Returns true when a character must not be written verbatim to a terminal.
///
/// Deliberately not a `const fn`: `char::is_control` only became usable in a
/// `const` context in Rust 1.97, and the workspace MSRV is 1.85.
#[must_use]
pub fn is_display_unsafe(character: char) -> bool {
    // `is_control` covers C0, DEL, and C1 — every byte that can begin or carry
    // an ANSI control sequence.
    character.is_control() || is_invisible_or_reordering(character)
}

fn push_escaped(target: &mut String, character: char) {
    if is_display_unsafe(character) {
        // `write!` to a String cannot fail; the result is discarded rather than
        // unwrapped because the workspace forbids `unwrap` and `expect`.
        let _ = write!(target, "\\u{{{:x}}}", character as u32);
    } else {
        target.push(character);
    }
}

/// Escapes anything that could alter how surrounding output renders, without
/// imposing a length bound.
///
/// Use this for values that already carry their own bound and whose full text
/// the reader needs — a filesystem path, for example, where truncating to the
/// diagnostic limit would hide which file was actually rejected.
#[must_use]
pub fn escaped_display_value(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        push_escaped(&mut escaped, character);
    }
    escaped
}

/// Bounds an untrusted value to a fixed length and escapes anything that could
/// alter how the surrounding diagnostic renders.
///
/// The length bound is applied to the *source* characters, so the caller still
/// sees up to [`LIMIT`] characters of the real value; escaping then expands each
/// unsafe character to `\u{...}`. The result is therefore still bounded, ASCII
/// for every escaped character, and a deterministic function of the input.
#[must_use]
pub fn bounded_display_value(value: &str) -> String {
    let mut characters = value.chars();
    let mut bounded = String::new();
    for character in characters.by_ref().take(LIMIT) {
        push_escaped(&mut bounded, character);
    }
    if characters.next().is_some() {
        bounded.push('…');
    }
    bounded
}

#[cfg(test)]
mod tests {
    use super::{LIMIT, bounded_display_value, escaped_display_value, is_display_unsafe};

    #[test]
    fn escaping_without_a_bound_keeps_long_values_whole() {
        let path = format!("/home/{}/manifest.toml", "a".repeat(200));

        assert_eq!(escaped_display_value(&path), path);
    }

    #[test]
    fn escaping_without_a_bound_still_neutralizes_escapes() {
        assert_eq!(
            escaped_display_value("/tmp/\u{1b}[2K\rfake: valid"),
            "/tmp/\\u{1b}[2K\\u{d}fake: valid"
        );
    }

    #[test]
    fn safe_values_pass_through_unchanged() {
        assert_eq!(bounded_display_value("source.commit"), "source.commit");
        assert_eq!(bounded_display_value("café"), "café");
    }

    #[test]
    fn long_values_are_truncated_with_an_ellipsis() {
        let rendered = bounded_display_value(&"x".repeat(10_000));

        assert_eq!(rendered, format!("{}…", "x".repeat(LIMIT)));
    }

    #[test]
    fn ansi_and_newlines_are_rendered_inert() {
        let rendered = bounded_display_value("evil\u{1b}[31m\nforged");

        assert_eq!(rendered, "evil\\u{1b}[31m\\u{a}forged");
        assert!(!rendered.contains('\u{1b}'));
        assert!(!rendered.contains('\n'));
    }

    #[test]
    fn bidirectional_overrides_and_zero_widths_are_escaped() {
        let rendered = bounded_display_value("a\u{202E}b\u{200B}c\u{2069}d");

        assert_eq!(rendered, "a\\u{202e}b\\u{200b}c\\u{2069}d");
    }

    #[test]
    fn carriage_return_cannot_erase_the_diagnostic_prefix() {
        let rendered = bounded_display_value("\rvalid");

        assert_eq!(rendered, "\\u{d}valid");
    }

    #[test]
    fn escaping_never_exceeds_the_bounded_expansion() {
        // Every source character is escaped to at most `\u{10ffff}` (10 chars).
        let rendered = bounded_display_value(&"\u{1b}".repeat(1_000));

        assert!(rendered.chars().count() <= LIMIT * 10 + 1);
    }

    #[test]
    fn the_ellipsis_itself_is_not_treated_as_unsafe() {
        assert!(!is_display_unsafe('…'));
    }
}
