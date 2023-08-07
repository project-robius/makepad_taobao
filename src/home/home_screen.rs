use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;

    import crate::shared::header::SimpleHeader;

    HomeScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}
        show_bg: true,
        draw_bg: {
            color: #fff
        }
        <SimpleHeader> {
        }
    }
}
