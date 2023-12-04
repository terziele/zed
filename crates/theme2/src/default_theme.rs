use std::sync::Arc;

use crate::{
    one_themes::{one_dark, one_family},
    Theme, ThemeFamily, Appearance, ThemeStyles, SystemColors, ThemeColors, StatusColors, PlayerColors, SyntaxTheme, default_color_scales,
};

fn zed_pro_daylight() -> Theme {
    Theme {
        id: "zed_pro_daylight".to_string(),
        name: "Zed Pro Daylight".into(),
        appearance: Appearance::Light,
        styles: ThemeStyles {
            system: SystemColors::default(),
            colors: ThemeColors::light(),
            status: StatusColors::light(),
            player: PlayerColors::light(),
            syntax: Arc::new(SyntaxTheme::light()),
        },
    }
}

pub(crate) fn zed_pro_moonlight() -> Theme {
    Theme {
        id: "zed_pro_moonlight".to_string(),
        name: "Zed Pro Moonlight".into(),
        appearance: Appearance::Dark,
        styles: ThemeStyles {
            system: SystemColors::default(),
            colors: ThemeColors::dark(),
            status: StatusColors::dark(),
            player: PlayerColors::dark(),
            syntax: Arc::new(SyntaxTheme::dark()),
        },
    }
}

pub fn zed_pro_family() -> ThemeFamily {
    ThemeFamily {
        id: "zed_pro".to_string(),
        name: "Zed Pro".into(),
        author: "Zed Team".into(),
        themes: vec![zed_pro_daylight(), zed_pro_moonlight()],
        scales: default_color_scales(),
    }
}

impl Default for ThemeFamily {
    fn default() -> Self {
        one_family()
    }
}

impl Default for Theme {
    fn default() -> Self {
        one_dark()
    }
}
