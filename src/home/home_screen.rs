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

    TAOBAO_COINS_IMG = dep("crate://self/resources/taobao_coins.png")
    HELP_CENTER_IMG = dep("crate://self/resources/help_center.png")
    LOGISTICS_IMG = dep("crate://self/resources/logistics_center.png")
    PAYMENTS_IMG = dep("crate://self/resources/payments_center.png")
    GLOBAL_DEALS_IMG = dep("crate://self/resources/global_deals.png")

    CREDIT_CARDS_IMG = dep("crate://self/resources/credit_cards.png")
    SHIPPING_ESTIMATE_IMG = dep("crate://self/resources/shipping_estimate.png")

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

    IconWithTwoLineText = <Frame> {
        walk: {width: Fit, height: Fit}
        layout: {flow: Down, align: {x: 0.5, y: 0.5}, spacing: 5.0}

        image = <Image> {
            walk: {width: 40, height: 40}
        }
        caption = <Label> {
            walk: {width: Fit, height: Fit}
            draw_label: {
                color: #000
                text_style: <REGULAR_TEXT> {font_size: 8.0},
            }
        }
        caption2 = <Label> {
            walk: {width: Fit, height: Fit}
            draw_label: {
                color: #000
                text_style: <REGULAR_TEXT> {font_size: 8.0},
            }
        }
    }

    TopBar = <Frame> {
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
                padding: {top: 9., left: 6., right: 6., bottom: 9.}
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

    Options = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Right, align: {x: 0.0, y: 0.5}, spacing: 10.0, padding: {left: 40., right: 40.}}

        <IconWithTwoLineText> {
            image = { image: (TAOBAO_COINS_IMG) }
            caption = { label: "Taobao" }
            caption2 = { label: "Coins" }
        }

        <FillerX> {}

        <IconWithTwoLineText> {
            image = { image: (HELP_CENTER_IMG) }
            caption = { label: "Help" }
            caption2 = { label: "Center" }
        }

        <FillerX> {}

        <IconWithTwoLineText> {
            image = { image: (LOGISTICS_IMG) }
            caption = { label: "Logistics" }
            caption2 = { label: "Center" }
        }

        <FillerX> {}

        <IconWithTwoLineText> {
            image = { image: (PAYMENTS_IMG) }
            caption = { label: "Payments" }
            caption2 = { label: "Center" }
        }

        <FillerX> {}

        <IconWithTwoLineText> {
            image = { image: (GLOBAL_DEALS_IMG) }
            caption = { label: "Global" }
            caption2 = { label: "Deals" }
        }
    }

    Payment = <Box> {
        walk: {width: Fill, height: Fit, margin: 10.0}
        layout: {flow: Right, align: {x: 0.0, y: 0.5}, padding: {left: 10., right: 10.} spacing: 6.0}
        draw_bg: {
            color: #fff,
            radius: 4.
        }

        <Label> {
            walk: {width: Fit, height: Fit, margin: {top: 20., bottom: 20.}}
            label: "Payment Support"
            draw_label: {
                color: #000
                text_style: <REGULAR_TEXT> {font_size: 10.0},
            }
        }

        <Image> {
            walk: {width: 118, height: 18}
            image: (CREDIT_CARDS_IMG)
        }

        <FillerX> {}

        <LineH> {
            walk: {width: 2.0, height: Fill}
        }

        <IconWithText> {
            image = { image: (SHIPPING_ESTIMATE_IMG), walk: {width: 32, height: 32} }
            caption = { label: "Ship Est" }
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

        <TopBar> {}
        <SearchBar> {}
        <Options> {}
        <Payment> {}
    }


}
