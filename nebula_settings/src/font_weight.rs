//! Terminal font weight choices.

use crate::RawSettings;

/// CSS-style weights offered by the settings page; `400` keeps the previous fixed behaviour.
pub const FONT_WEIGHT_VALUES: &[&str] =
    &["100", "200", "300", "400", "500", "600", "700", "800", "900"];
pub const DEFAULT_FONT_WEIGHT: u16 = 400;

pub(crate) fn font_weight(raw: &RawSettings) -> u16 {
    raw.value("font_weight")
        .filter(|value| FONT_WEIGHT_VALUES.contains(value))
        .and_then(|value| value.parse().ok())
        .unwrap_or(DEFAULT_FONT_WEIGHT)
}

/// Bold text stays visibly heavier than regular text; at the default weight it is exactly 700.
pub fn bold_font_weight(regular: u16) -> u16 {
    regular.saturating_add(300).clamp(700, 900)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RawSettings, RuntimeSettings, apply_updates};

    #[test]
    fn missing_and_invalid_weights_use_the_default() {
        for value in ["", "font_weight=", "font_weight=abc", "font_weight=450", "font_weight=1000"] {
            let settings = RuntimeSettings::from_raw(&RawSettings::from_text(value));
            assert_eq!(settings.font_weight, DEFAULT_FONT_WEIGHT, "{value}");
        }
    }

    #[test]
    fn every_offered_weight_round_trips() {
        for value in FONT_WEIGHT_VALUES {
            let text = apply_updates("", &[("font_weight", (*value).into())]);
            let settings = RuntimeSettings::from_raw(&RawSettings::from_text(&text));
            assert_eq!(settings.font_weight, value.parse::<u16>().unwrap());
        }
    }

    #[test]
    fn bold_is_never_lighter_than_700() {
        assert_eq!(bold_font_weight(100), 700);
        assert_eq!(bold_font_weight(400), 700);
        assert_eq!(bold_font_weight(500), 800);
        assert_eq!(bold_font_weight(900), 900);
    }
}
