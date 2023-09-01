use makepad_widgets::*;

live_design! {
    import makepad_widgets::view::*;
    import crate::shared::styles::*;

    Divider = <View> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Down}
        <Box> {
            walk: {width: Fill, height: 1.}
            draw_bg: {color: (#ddd)}
        }
    }

    LineH = <Box> {
        walk: {width: Fill, height: 2, margin: 0.0}
        layout: {padding: 0.0, spacing: 0.0}
        draw_bg: {color: (COLOR_DIVIDER)}
    }

    FillerX = <View> {walk: {width: Fill, height: Fit}}
    FillerY = <View> {walk: {width: Fit, height: Fill}}
}
