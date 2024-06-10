//! HTML escaping.
//!
//! This module contains one unit struct, which can be used to HTML-escape a
//! string of text (for use in a format string).

use std::fmt;

use unicode_segmentation::UnicodeSegmentation;

/// Wrapper struct which will emit the HTML-escaped version of the contained
/// string when passed to a format string.
pub(crate) struct Escape<'a>(pub &'a str);

impl<'a> fmt::Display for Escape<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Because the internet is always right, turns out there's not that many
        // characters to escape: http://stackoverflow.com/questions/7381974
        let Escape(s) = *self;
        let pile_o_bits = s;
        let mut last = 0;
        for (i, ch) in s.char_indices() {
            let s = match ch {
                '>' => "&gt;",
                '<' => "&lt;",
                '&' => "&amp;",
                '\'' => "&#39;",
                '"' => "&quot;",
                _ => continue,
            };
            fmt.write_str(&pile_o_bits[last..i])?;
            fmt.write_str(s)?;
            // NOTE: we only expect single byte characters here - which is fine as long as we
            // only match single byte characters
            last = i + 1;
        }

        if last < s.len() {
            fmt.write_str(&pile_o_bits[last..])?;
        }
        Ok(())
    }
}

/// Wrapper struct which will emit the HTML-escaped version of the contained
/// string when passed to a format string.
///
/// This is only safe to use for text nodes. If you need your output to be
/// safely contained in an attribute, use [`Escape`]. If you don't know the
/// difference, use [`Escape`].
pub(crate) struct EscapeBodyText<'a>(pub &'a str);

impl<'a> fmt::Display for EscapeBodyText<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Because the internet is always right, turns out there's not that many
        // characters to escape: http://stackoverflow.com/questions/7381974
        let EscapeBodyText(s) = *self;
        let pile_o_bits = s;
        let mut last = 0;
        for (i, ch) in s.char_indices() {
            let s = match ch {
                '>' => "&gt;",
                '<' => "&lt;",
                '&' => "&amp;",
                _ => continue,
            };
            fmt.write_str(&pile_o_bits[last..i])?;
            fmt.write_str(s)?;
            // NOTE: we only expect single byte characters here - which is fine as long as we
            // only match single byte characters
            last = i + 1;
        }

        if last < s.len() {
            fmt.write_str(&pile_o_bits[last..])?;
        }
        Ok(())
    }
}

/// Wrapper struct which will emit the HTML-escaped version of the contained
/// string when passed to a format string. This function also word-breaks
/// CamelCase and snake_case word names.
///
/// This is only safe to use for text nodes. If you need your output to be
/// safely contained in an attribute, use [`Escape`]. If you don't know the
/// difference, use [`Escape`].
pub(crate) struct EscapeBodyTextWithWbr<'a>(pub &'a str);

impl<'a> fmt::Display for EscapeBodyTextWithWbr<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let EscapeBodyTextWithWbr(text) = *self;
        if text.len() < 8 {
            return EscapeBodyText(text).fmt(fmt);
        }
        let mut last = 0;
        let mut it = text.grapheme_indices(true).peekable();
        let _ = it.next(); // don't insert wbr before first char
        while let Some((i, s)) = it.next() {
            let pk = it.peek();
            let is_uppercase = || s.chars().any(|c| c.is_uppercase());
            let next_is_uppercase =
                || pk.map_or(true, |(_, t)| t.chars().any(|c| c.is_uppercase()));
            let next_is_underscore = || pk.map_or(true, |(_, t)| t.contains('_'));
            if (i - last > 3 && is_uppercase() && !next_is_uppercase())
                || (s.contains('_') && !next_is_underscore())
            {
                EscapeBodyText(&text[last..i]).fmt(fmt)?;
                fmt.write_str("<wbr>")?;
                last = i;
            }
        }
        if last < text.len() {
            EscapeBodyText(&text[last..]).fmt(fmt)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
