use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::image::*;
    import crate::shared::styles::*;
    
    IconWithText = <Frame> {
        walk: {width: Fit, height: Fit}
        layout: {flow: Down, align: {x: 0.5, y: 0.5}, spacing: 5.0}
        
        image = <Image> {
            walk: {width: 24, height: 24}
        }
        caption = <Label> {
            walk: {width: Fit, height: Fit}
            draw_label: {
                color: #000
                text_style: <REGULAR_TEXT> {font_size: 8.0},
            }
        }
    }
}