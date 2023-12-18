use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import crate::shared::styles::*;
    import crate::shared::helpers::*;

    IconWithText = <View> {
        width: Fit
        height: Fit
        flow: Down
        align: {x: 0.5, y: 0.5}
        spacing: 5.0

        image = <Image> {
            width: 24, height: 24
        }
        caption = <Label> {
            width: Fit, height: Fit
            draw_text: {
                color: #000
                text_style: <REGULAR_TEXT> {font_size: 8.0},
            }
        }
    }

    FeaturedBox = <RoundedView> {
        width: Fill, height: 113
        flow: Down
        padding: 6.0
        align: {x: 0.0, y: 0.0}
        spacing: 2.0

        draw_bg: {
            color: #fff,
            radius: 4.
        }

        header = <View> {
            width: Fill, height: Fit
            label = <Label> {
                width: Fit, height: Fit, margin: {top: 1.0}
                draw_text: {
                    color: #000
                    text_style: <TITLE_TEXT> {font_size: 9.0},
                }
            }
        }

        content = <View> {
            width: Fill, height: Fill
            padding: {right: 18.0, left: 18.0}
            flow: Right,
            align: {x: 0.5, y: 0},
            spacing: 1.0,

            image_container_1 = <View> {
                width: 60
                height: Fill
                flow: Down
                align: {x: 0.5, y: 0.0}
                spacing: 5.0

                <FillerY> {}
                image = <Image> {}
                <FillerY> {}
                label = <Label> {
                    width: Fit, height: Fit
                    draw_text: {
                        color: #bf1f5f
                        text_style: <TITLE_TEXT> {font_size: 8.0},
                    }
                }
            }

            image_container_2 = <View> {
                width: 60
                height: Fill
                flow: Down
                align: {x: 0.5, y: 0.0}
                spacing: 5.0
                <FillerY> {}
                image = <Image> {}
                <FillerY> {}
                label = <Label> {
                    width: Fit, height: Fit
                    draw_text: {
                        color: #bf1f5f
                        text_style: <TITLE_TEXT> {font_size: 8.0},
                    }
                }
            }
        }
    }

    FeaturedBoxWithHighlightBox = <FeaturedBox> {
        header = {
            highlight = <RoundedView> {
                width: Fit
                height: Fit
                margin: {left: 6.}
                padding: {left: 2.0, right: 2.0, top: 1.0, bottom: 1.0}
                flow: Right,
                align: {x: 0.0, y: 0.0},

                draw_bg: {
                    color: #ed3bd5,
                    radius: 1.
                }
                label = <Label> {
                    width: Fit, height: Fit
                    draw_text: {
                        color: #fff
                        text_style: <REGULAR_TEXT> {font_size: 9.0},
                    }
                }
            }
        }
    }

    FeaturedBoxWithHighlightLabel = <FeaturedBox> {
        header = {
            highlight = <Label> {
                width: Fit, height: Fit, margin: {left: 6., top: 3.0}
                draw_text: {
                    color: #ed9fe3
                    text_style: <REGULAR_TEXT> {font_size: 7.0},
                }
            }
        }
    }
}
