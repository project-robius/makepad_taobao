use makepad_widgets::*;

live_design! {
    APP_NAVIGATION_FONT = {
        font_size: 9,
        font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf")}
    }

    TITLE_TEXT = {
        font_size: 14,
        // TODO Use the Bold variant of the font, when performance is good enough
        font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf")}
    }

    REGULAR_TEXT = {
        font_size: 12,
        font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf")}
    }

    BACKGROUND_COLOR = #f5f5f5
    ORANGE_COLOR = #ee7630
    LIGHT_ORANGE_COLOR = #f2d1bd
    COLOR_DIVIDER = #x00000018
}
