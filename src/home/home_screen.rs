use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::text_input::TextInput;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;

    WORLDWIDE_ICON = dep("crate://self/resources/worldwide.png")
    MEMBERSHIP_CODE_ICON = dep("crate://self/resources/membership_code.png")
    SCAN_ICON = dep("crate://self/resources/scan.png")
    CAMERA_ICON = dep("crate://self/resources/camera.png")

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

    SearchBar = <Box> {
        walk: {width: Fill, height: Fit, margin: 10.0}
        layout: {flow: Right, align: {x: 0.0, y: 0.5}, padding: {left: 10., right: 10.} spacing: 6.0}
        draw_bg: {
            color: #fff,
            radius: 10.
        }

        <Image> {
            walk: {width: 24, height: 24}
            image: (SCAN_ICON)
        }

        <LineH> {
            walk: {width: 3.0, height: Fill}
        }

        input = <TextInput> {
            walk: {width: Fill, height: Fit}
            layout: {
                clip_x: true,
                clip_y: true,
                align: {y: 0.5},
                padding: {bottom: 13.}
            },
            text: "Ping An fruit box"
            label_walk: {
                margin: 0.0
            }
            draw_bg: {
                color: #fff
            }
            draw_label: {
                text_style:<REGULAR_TEXT>{font_size: 10.0},

                fn get_color(self) -> vec4 {
                    return #ccc
                }
            }

            // TODO find a way to override colors
            draw_cursor: {
                instance focus: 0.0
                uniform border_radius: 0.5
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        0.,
                        0.,
                        self.rect_size.x,
                        self.rect_size.y,
                        self.border_radius
                    )
                    sdf.fill(mix((ORANGE_COLOR), (ORANGE_COLOR), self.focus));
                    return sdf.result
                }
            }

            // TODO find a way to override colors
            draw_select: {
                instance hover: 0.0
                instance focus: 0.0
                uniform border_radius: 2.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        0.,
                        0.,
                        self.rect_size.x,
                        self.rect_size.y,
                        self.border_radius
                    )
                    sdf.fill(mix((ORANGE_COLOR), (ORANGE_COLOR), self.focus));
                    return sdf.result
                }
            }
        }

        <Image> {
            walk: {width: 24, height: 24}
            image: (CAMERA_ICON)
        }

        <Box> {
            walk: {width: Fit, height: Fit}
            layout: {
                flow: Right, align: {x: 0.0, y: 0.5}, spacing: 10.0,
                padding: {top: 6., left: 6., right: 6., bottom: 6.}
            }
            draw_bg: {
                color: (ORANGE_COLOR),
                radius: 6.
            }

            <Label> {
                walk: {width: Fit, height: Fit}
                label: "Search"
                draw_label: {
                    color: #fff
                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                }
            }
        }
    }

    HomeScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}
        show_bg: true,
        draw_bg: {
            color: (BACKGROUND_COLOR)
        }

        os_header_placeholder = <Box> {
            walk: {width: Fill, height: 50, margin: 0}
            layout: {flow: Right, spacing: 6.0, padding: 0}
        }

        top_bar = <Frame> {
            walk: {width: Fill, height: Fit}
            layout: {flow: Right, align: {x: 0.0, y: 0.5}, spacing: 10.0, padding: {left: 10., right: 10.}}

            <IconWithText> {
                image = { image: (WORLDWIDE_ICON) }
                caption = { label: "WW" }
            }

            <FillerX> {}

            <Label> {
                walk: {width: Fit, height: Fit}
                label: "Recommend"
                draw_label: {
                    color: #000
                    text_style: <REGULAR_TEXT> {},
                }
            }
            <Label> {
                walk: {width: Fit, height: Fit}
                label: "Special Offer"
                draw_label: {
                    color: #000
                    text_style: <REGULAR_TEXT> {},
                }
            }

            <FillerX> {}

            <IconWithText> {
                image = { image: (MEMBERSHIP_CODE_ICON) }
                caption = { label: "M Code" }
            }
        }

        <SearchBar> {}
    }


}
