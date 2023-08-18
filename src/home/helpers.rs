use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::image::*;
    import crate::shared::styles::*;
    import crate::shared::helpers::*;
    
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

    FeaturedBox = <Box> {
        walk: {width: Fill, height: 113}
        layout: {flow: Down, align: {x: 0.0, y: 0.0}, spacing: 2.0, padding: 6.0}

        draw_bg: {
            color: #fff,
            radius: 4.
        }

        header = <Frame> {
            walk: {width: Fill, height: Fit}
            label = <Label> {
                walk: {width: Fit, height: Fit, margin: {top: 1.0}}
                draw_label: {
                    color: #000
                    text_style: <TITLE_TEXT> {font_size: 9.0},
                }
            }
        }

        content = <Frame> {
            walk: {width: Fill, height: Fill }
            layout: {
                flow: Right,
                align: {x: 0.5, y: 0},
                spacing: 1.0,
                padding: {right: 18.0, left: 18.0}
            }

            image_container_1 = <Frame> {
                walk: {width: 60, height: Fill}
                layout: {flow: Down, align: {x: 0.5, y: 0.0}, spacing: 5.0}
                <FillerY> {}
                image = <Image> {
                }
                <FillerY> {}
                label = <Label> {
                    walk: {width: Fit, height: Fit}
                    draw_label: {
                        color: #bf1f5f
                        text_style: <TITLE_TEXT> {font_size: 8.0},
                    }
                }
            }

            image_container_2 = <Frame> {
                walk: {width: 60, height: Fill}
                layout: {flow: Down, align: {x: 0.5, y: 0.0}, spacing: 5.0}
                <FillerY> {}
                image = <Image> {
                }
                <FillerY> {}
                label = <Label> {
                    walk: {width: Fit, height: Fit}
                    draw_label: {
                        color: #bf1f5f
                        text_style: <TITLE_TEXT> {font_size: 8.0},
                    }
                }
            }
        }
    }

    FeaturedBoxWithHighlightBox = <FeaturedBox> {
        header = {
            highlight = <Box> {
                walk: {width: Fit, height: Fit, margin: {left: 6.}}
                layout: {
                    flow: Right,
                    align: {x: 0.0, y: 0.0},
                    padding: {left: 2.0, right: 2.0, top: 1.0, bottom: 1.0}
                }
                draw_bg: {
                    color: #ed3bd5,
                    radius: 1.
                }
                label = <Label> {
                    walk: {width: Fit, height: Fit}
                    draw_label: {
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
                walk: {width: Fit, height: Fit, margin: {left: 6., top: 3.0}}
                draw_label: {
                    color: #ed9fe3
                    text_style: <REGULAR_TEXT> {font_size: 7.0},
                }
            }
        }
    }
}