pub(crate) const COMPACT_LOGO: &str = "⣰⣭⣆";
pub(crate) const PRODUCT_NAME: &str = "Agent9527";

#[cfg(test)]
mod tests {
    use super::*;
    use unicode_width::UnicodeWidthStr;

    #[test]
    fn compact_logo_has_stable_terminal_width() {
        assert_eq!(UnicodeWidthStr::width(COMPACT_LOGO), 3);
    }
}
