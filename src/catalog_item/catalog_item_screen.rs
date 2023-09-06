use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

// TODO: Use the item ID to get the data dynamically

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;

    CATALOG_RING_IMG = dep("crate://self/resources/catalog/ring.png")
    AVATAR_IMG = dep("crate://self/resources/default_avatar.png")
    STAR_IMG = dep("crate://self/resources/star.png")
    ROUNDED_STAR_IMG = dep("crate://self/resources/rounded_star.png")
    CHAT_BUBBLE_IMG = dep("crate://self/resources/chat_bubble.png")
    STORE_IMG = dep("crate://self/resources/store.png")

    BottomBar = <View> {
        width: Fill
        height: Fit
        flow: Right
        spacing: 15.
        padding: 5.
        align: {y:0.5}
        draw_bg: {
            color: #fff
        }

        <View> {
            width: Fit
            height: Fit
            flow: Right
            spacing: 25.
            <View> {
                width: Fit
                height: Fit
                flow: Down
                spacing: 5.
                <Image> {
                    width: 25, height: 25
                    source: (STORE_IMG)
                }
                <Label> {
                    width: Fit, height: Fit
                    text: "每每"
                    draw_text: {
                        color: #9d
                        text_style: <REGULAR_TEXT> {font_size: 10.0},
                    }
                }
            }
            <View> {
                width: Fit
                height: Fit
                flow: Down
                spacing: 5.
                <Image> {
                    width: 25, height: 25
                    source: (CHAT_BUBBLE_IMG)
                }
                <Label> {
                    width: Fit, height: Fit
                    text: "每每"
                    draw_text: {
                        color: #9d
                        text_style: <REGULAR_TEXT> {font_size: 10.0},
                    }
                }
            }
            <View> {
                width: Fit
                height: Fit
                flow: Down
                spacing: 5.
                <Image> {
                    width: 25, height: 25
                    source: (ROUNDED_STAR_IMG)
                }
                <Label> {
                    width: Fit, height: Fit
                    text: "每每"
                    draw_text: {
                        color: #9d
                        text_style: <REGULAR_TEXT> {font_size: 10.0},
                    }
                }
            }
        }

        <View> {
            width: Fit
            height: Fit
            flow: Right
            padding: {top: 10., right: 25, bottom: 10, left: 25.}
            spacing: 25.
            show_bg: true
            draw_bg: {
                color: #ffad01
                instance radius: 8.
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                    sdf.box(
                        1,
                        1,
                        self.rect_size.x - 2.0,
                        self.rect_size.y - 2.0,
                        max(1.0, self.radius)
                    );

                    if self.pos.x < 0.5 {
                        sdf.fill_keep(#ffad01);
                    } else {
                        sdf.fill_keep(#ff6103);
                    }

                    return sdf.result;
                }

            }
            <Label> {
                width: Fit, height: Fit
                text: "每每每"
                draw_text: {
                    color: #fff
                    text_style: <TITLE_TEXT> {font_size: 12.0},
                }
            }
            <FillerX> {}
            <Label> {
                width: Fit, height: Fit
                text: "每每每"
                draw_text: {
                    color: #fff
                    text_style: <TITLE_TEXT> {font_size: 12.0},
                }
            }
        }
    }

    Section = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        spacing: 10.
        padding: 10.
        show_bg: true
        draw_bg: {
            color: #fff
            radius: 5.
        }
    }

    StoreProfileImage = <Image> {
        width: 50, height: 50
        source: (CATALOG_RING_IMG)
        draw_bg: {
            instance radius: 2.
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    1,
                    1,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    max(1.0, self.radius)
                )
                sdf.fill_keep(self.get_color())
                return sdf.result
            }
        }
    }
    VariantImage = <StoreProfileImage> {
        width: 30, height: 30
    }

    RecomendedItem = <View> {
        width: Fit
        height: Fit
        flow: Down
        spacing: 10.

        <Image> {
            width: 100, height: 100, source: (CATALOG_RING_IMG)
            draw_bg: {
                instance radius: 5.
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        1,
                        1,
                        self.rect_size.x - 2.0,
                        self.rect_size.y - 2.0,
                        max(1.0, self.radius)
                    )
                    sdf.fill_keep(self.get_color())
                    return sdf.result
                }
            }
        }
        <Label> {
            width: Fill, height: Fit
            text: "每每减每2023每减每减每"
            draw_text: {
                color: #0
                text_style: <REGULAR_TEXT> {font_size: 10},
                wrap: Word
            }
        }
        <Label> {
            width: Fit, height: Fit
            text: "¥50"
            draw_text: {
                color: #f15603
                text_style: <TITLE_TEXT> {font_size: 14.0},
            }
        }
    }


    CatalogItem = {{CatalogItem}} {
        view: <View> {
            width: Fill
            height: Fit
            padding: 0.
            flow: Down
            spacing: 0.
            show_bg: true
            draw_bg: {
                color: (BACKGROUND_COLOR)
            }

            main_image = <Image> {
                width: Fill, height: 300, source: (CATALOG_RING_IMG)
            }

            sections = <View> {
                width: Fill
                height: Fit
                padding: 10.
                flow: Down
                spacing: 10.

                <Section> {
                    price_container = <View> {
                        width: Fit
                        height: Fit
                        flow: Right
                        align: {y: 0.5}
                        spacing: 5.

                        <Label> {
                            width: Fit, height: Fit
                            text: "¥50"
                            draw_text: {
                                color: #f15603
                                text_style: <TITLE_TEXT> {font_size: 14.0},
                            }
                        }
                        <RoundedView> {
                            width: Fit, height: Fit
                            padding: {right:10.0, left: 10., top: 5, bottom: 5}
                            draw_bg: {
                                color: #f15603,
                                radius: 7.
                            }
                            <Label> {
                                width: Fit, height: Fit
                                text: "每每每每 ¥3000"
                                draw_text: {
                                    color: #fff
                                    text_style: <TITLE_TEXT> {font_size: 14.0},
                                }
                            }
                        }
                    }
                    <View> {
                        width: Fill, height: Fit, flow: Right
                        <RoundedView> {
                            width: Fit
                            height: Fit
                            padding: 3.0
                            draw_bg: {
                                color: #ffe9e5,
                                radius: 2.
                            }
                            <Label> {
                                width: Fit, height: Fit
                                text: "每300减40"
                                draw_text: {
                                    color: (ORANGE_COLOR)
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <FillerX> {}

                        <Label> {
                            width: Fit, height: Fit
                            text: "每减 >"
                            draw_text: {
                                color: (ORANGE_COLOR)
                                text_style: <REGULAR_TEXT> {font_size: 10.0},
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit, flow: Right

                        <RoundedView> {
                            width: Fit
                            height: Fit
                            padding: {right: 6., left: 6., top: 3, bottom: 3}
                            draw_bg: {
                                color: #fa0322,
                                radius: 4.
                            }
                            <Label> {
                                width: Fit, height: Fit
                                text: "每每"
                                draw_text: {
                                    color: #fff
                                    text_style: <TITLE_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <FillerX> {}

                        <Label> {
                            width: Fit, height: Fit
                            text: "每减 100+"
                            draw_text: {
                                color: #9d
                                text_style: <REGULAR_TEXT> {font_size: 10.0},
                            }
                        }
                    }

                    <Label> {
                        width: Fill, height: Fit
                        text: "萨帕托斯萨帕托斯萨帕托斯萨帕托斯萨帕托斯萨帕托斯萨帕托斯斯萨帕托斯"
                        draw_text: {
                            color: #0
                            text_style: <TITLE_TEXT> {font_size: 10.0},
                        }
                    }
                }

                <Section> {
                    spacing: 20
                    <View> {
                        width: Fill, height: Fit, flow: Right, spacing: 20.

                        <View> {
                            width: Fit, height: Fit
                            flow: Right, align: {x: 0, y: 0}
                            <Label> {
                                text: "每减"
                                draw_text: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <View> {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 3.
                            <Label> {
                                text: "每减每减每减"
                                draw_text: {
                                    color: #0
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                            <View> {
                                width: Fit
                                height: Fit
                                flow: Right
                                spacing: 3.
                                <VariantImage> {}
                                <VariantImage> {}
                                <VariantImage> {}
                            }
                        }

                        <View> {
                            width: Fit
                            height: Fit
                            flow: Right
                            align: {x: 1, y: 1}
                            <Label> {
                                text: ">"
                                draw_text: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                    }

                    <View> {
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 20.

                        <View> {
                            width: Fit
                            height: Fit
                            flow: Right
                            align: {x: 0, y: 0}
                            <Label> {
                                text: "每减"
                                draw_text: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <View> {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 5.
                            <Label> {
                                text: "每减每减每减"
                                draw_text: {
                                    color: #0
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                            <Label> {
                                text: "每减: 每减"
                                draw_text: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <View> {
                            width: Fit
                            height: Fit
                            flow: Right
                            align: {x: 1, y: 1}
                            <Label> {
                                text: ">"
                                draw_text: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                    }

                    <View> {
                        width: Fill
                        height: Fit
                        flow: Right, spacing: 20.

                        <View> {
                            width: Fit
                            height: Fit
                            flow: Right
                            align: {x: 0, y: 0}
                            <Label> {
                                text: "每减"
                                draw_text: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <View> {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 5.

                            <Label> {
                                text: "每减每减每减"
                                draw_text: {
                                    color: #0
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <View> {
                            width: Fit
                            height: Fit
                            flow: Right
                            align: {x: 1, y: 1}
                            <Label> {
                                text: ">"
                                draw_text: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                    }
                }

                <Section> {
                    spacing: 30.
                    <View> {
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 10.
                        <View> {
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 10.
                            align: {y: 0.5}

                            <Label> {
                                text: "每减每 (400+)"
                                draw_text: {
                                    color: #0
                                    text_style: <TITLE_TEXT> {font_size: 11},
                                }
                            }

                            <FillerX> {}

                            <Label> {
                                text: "每减每 >"
                                draw_text: {
                                    color: (ORANGE_COLOR)
                                    text_style: <TITLE_TEXT> {font_size: 10},
                                }
                            }
                        }

                        <View> {
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 5.
                            <RoundedView> {
                                width: Fit, height: Fit, padding: 6.0
                                draw_bg: {
                                    color: #ffe9e5,
                                    radius: 3.
                                }
                                <Label> {
                                    width: Fit, height: Fit
                                    text: "每减 (40)"
                                    draw_text: {
                                        color: #0,
                                        text_style: <REGULAR_TEXT> {font_size: 10.0},
                                    }
                                }
                            }
                            <RoundedView> {
                                width: Fit, height: Fit, padding: 6.0
                                draw_bg: {
                                    color: #ffe9e5,
                                    radius: 3.
                                }
                                <Label> {
                                    width: Fit, height: Fit
                                    text: "每减 (70)"
                                    draw_text: {
                                        color: #0,
                                        text_style: <REGULAR_TEXT> {font_size: 10.0},
                                    }
                                }
                            }
                            <RoundedView> {
                                width: Fit, height: Fit, padding: 6.0
                                draw_bg: {
                                    color: #ffe9e5,
                                    radius: 3.
                                }
                                <Label> {
                                    width: Fit, height: Fit
                                    text: "每减 (90)"
                                    draw_text: {
                                        color: #0,
                                        text_style: <REGULAR_TEXT> {font_size: 10.0},
                                    }
                                }
                            }
                        }

                        <View> {
                            width: Fill, height: Fit
                            flow: Down, spacing: 10.

                            <View> {
                                width: Fill, height: Fit
                                flow: Right, spacing: 5, align: {y: 0.5}

                                <Image> {
                                    width: 25, height: 25
                                    source: (AVATAR_IMG)
                                }
                                <Label> {
                                    width: Fit, height: Fit
                                    text: "每减减减"
                                    draw_text: {
                                        color: #0,
                                        text_style: <REGULAR_TEXT> {font_size: 8.0},
                                    }
                                }
                            }
                            <Label> {
                                width: Fit, height: Fit
                                text: "每减每减"
                                draw_text: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                        <View> {
                            width: Fill, height: Fit
                            flow: Down, spacing: 10.

                            <View> {
                                width: Fill, height: Fit
                                flow: Right, spacing: 5, align: {y: 0.5}

                                <Image> {
                                    width: 25, height: 25
                                    source: (AVATAR_IMG)
                                }
                                <Label> {
                                    width: Fit, height: Fit
                                    text: "每减减减"
                                    draw_text: {
                                        color: #0,
                                        text_style: <REGULAR_TEXT> {font_size: 8.0},
                                    }
                                }
                            }
                            <Label> {
                                width: Fit, height: Fit
                                text: "每每减每减减"
                                draw_text: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: Fit
                        flow: Down, spacing: 20.

                        <View> {
                            width: Fill, height: Fit
                            flow: Right, spacing: 10., align: {y: 0.5}

                            <Label> {
                                text: "每减每 (5)"
                                draw_text: {
                                    color: #0
                                    text_style: <TITLE_TEXT> {font_size: 11},
                                }
                            }

                            <FillerX> {}

                            <Label> {
                                text: ">",
                                draw_text: {
                                    color: #9d
                                    text_style: <TITLE_TEXT> {font_size: 10},
                                }
                            }
                        }

                        <View> {
                            width: Fill, height: Fit
                            flow: Down, spacing: 10., padding: {left: 20.}

                            <View> {
                                width: Fill, height: Fit
                                flow: Right, spacing: 10.

                                <Label> {
                                    text: "问",
                                    draw_text: {
                                        color: (ORANGE_COLOR)
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }

                                <Label> {
                                    text: "每减减减 减减",
                                    draw_text: {
                                        color: #0
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }

                                <FillerX> {}


                                <Label> {
                                    text: "6减减",
                                    draw_text: {
                                        color: #9d
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }
                            }
                            <View> {
                                width: Fill, height: Fit
                                flow: Right, spacing: 10.

                                <Label> {
                                    text: "问",
                                    draw_text: {
                                        color: (ORANGE_COLOR)
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }

                                <Label> {
                                    text: "每减减减 减减每减减减 减减",
                                    draw_text: {
                                        color: #0
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }

                                <FillerX> {}


                                <Label> {
                                    text: "6减减",
                                    draw_text: {
                                        color: #9d
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }
                            }
                        }

                    }
                }

                <Section> {
                    padding: 0
                    <View> {
                        width: Fill
                        height: Fit
                        padding: {top: 20, left: 15, right: 15, bottom: 25}
                        flow: Down
                        spacing: 20.
                        show_bg: true
                        draw_bg: {
                            instance radius: 5.

                            fn get_color(self) -> vec4 {
                                return #a6213e
                            }
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(
                                    1,
                                    1,
                                    self.rect_size.x - 2.0,
                                    // This calculation is to make sure the bottom part is not rounded
                                    self.rect_size.y + self.radius * 2.0,
                                    max(1.0, self.radius)
                                )
                                sdf.fill_keep(self.get_color())
                                return sdf.result
                            }
                        }

                        <View> {
                            width: Fill, height: Fit
                            flow: Right, spacing: 5.

                            <StoreProfileImage> {}

                            <View> {
                                width: Fill, height: Fit
                                flow: Down, spacing: 10.

                                <Label> {
                                    text: "减减减减",
                                    draw_text: {
                                        color: #f
                                        text_style: <TITLE_TEXT> {font_size: 10},
                                    }
                                }

                                <View> {
                                    width: Fill, height: Fit
                                    flow: Right, spacing: 5., align: {y: 0.5}

                                    <RoundedView> {
                                        width: Fit, height: Fit
                                        padding: {right: 6., left: 6., top: 3, bottom: 3}
                                        draw_bg: {
                                            color: #fa0322,
                                            radius: 4.
                                        }
                                        <Label> {
                                            width: Fit, height: Fit
                                            text: "每每"
                                            draw_text: {
                                                color: #fff
                                                text_style: <TITLE_TEXT> {font_size: 10.0},
                                            }
                                        }
                                    }

                                    <RoundedView> {
                                        width: Fit, height: Fit
                                        padding: {right: 6., left: 6., top: 3, bottom: 3}
                                        spacing: 5.
                                        align: {y: 0.5}

                                        draw_bg: {
                                            color: #43,
                                            radius: 4.
                                        }
                                        <Label> {
                                            width: Fit, height: Fit
                                            text: "每每"
                                            draw_text: {
                                                color: #f
                                                text_style: <REGULAR_TEXT> {font_size: 10.0},
                                            }
                                        }
                                        <View> {
                                            width: Fit
                                            height: Fit
                                            align: {y: 0.5}

                                            <Image> {
                                                width: 10, height: 10
                                                source: (STAR_IMG)
                                            }
                                            <Image> {
                                                width: 10, height: 10
                                                source: (STAR_IMG)
                                            }
                                            <Image> {
                                                width: 10, height: 10
                                                source: (STAR_IMG)
                                            }
                                            <Image> {
                                                width: 10, height: 10
                                                source: (STAR_IMG)
                                            }
                                            <Image> {
                                                width: 10, height: 10
                                                source: (STAR_IMG)
                                            }
                                        }
                                    }
                                    <Label> {
                                        width: Fit, height: Fit
                                        text: "3.75每每"
                                        draw_text: {
                                            color: #f
                                            text_style: <REGULAR_TEXT> {font_size: 8.0},
                                        }
                                    }
                                }

                                <View> {
                                    width: Fill, height: Fit
                                    flow: Right, spacing: 5., align: {y: 0.5}

                                    <Label> {
                                        width: Fit, height: Fit
                                        text: "每每3.75"
                                        draw_text: {
                                            color: #f
                                            text_style: <REGULAR_TEXT> {font_size: 8.0},
                                        }
                                    }
                                    <Label> {
                                        width: Fit, height: Fit
                                        text: "每每3.75"
                                        draw_text: {
                                            color: #f
                                            text_style: <REGULAR_TEXT> {font_size: 8.0},
                                        }
                                    }
                                    <Label> {
                                        width: Fit, height: Fit
                                        text: "每每3.75"
                                        draw_text: {
                                            color: #f
                                            text_style: <REGULAR_TEXT> {font_size: 8.0},
                                        }
                                    }
                                }
                            }
                        }
                        <View> {
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 5.
                            align: {x: 0.5}

                            <RoundedView> {
                                width: Fit
                                height: Fit
                                padding: {right: 12., left: 12., top: 5, bottom: 5}

                                draw_bg: {
                                    color: #f,
                                    radius: 4.
                                }
                                <Label> {
                                    width: Fit, height: Fit
                                    text: "每每每每"
                                    draw_text: {
                                        color: (ORANGE_COLOR)
                                        text_style: <TITLE_TEXT> {font_size: 14.0},
                                    }
                                }
                            }
                            <RoundedView> {
                                width: Fit
                                height: Fit
                                padding: {right: 12., left: 12., top: 5, bottom: 5}

                                draw_bg: {
                                    color: #f,
                                    radius: 4.
                                }
                                <Label> {
                                    width: Fit, height: Fit
                                    text: "每每每每"
                                    draw_text: {
                                        color: (ORANGE_COLOR)
                                        text_style: <TITLE_TEXT> {font_size: 14.0},
                                    }
                                }
                            }
                        }
                    }

                    <View> {
                        width: Fill
                        height: Fit
                        padding: 10.
                        flow: Down
                        spacing: 10.

                        <View> {
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 10.
                            align: {y: 0.5}

                            <Label> {
                                text: "每减每 (5)"
                                draw_text: {
                                    color: #0
                                    text_style: <TITLE_TEXT> {font_size: 11},
                                }
                            }

                            <FillerX> {}

                            <Label> {
                                width: Fit, height: Fit
                                text: "每减 >"
                                draw_text: {
                                    color: (ORANGE_COLOR)
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                    }

                    <View> {
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 5.
                        align: {x: 0.5}

                        <View> {
                            width: Fit, height: Fit
                            flow: Right, spacing: 5.
                            <RecomendedItem> {}
                            <RecomendedItem> {}
                            <RecomendedItem> {}
                        }
                        <View> {
                            width: Fit, height: Fit
                            flow: Right, spacing: 5.
                            <RecomendedItem> {}
                            <RecomendedItem> {}
                            <RecomendedItem> {}
                        }
                    }
                }
            }
        }
    }

    CatalogItemScrollable = {{CatalogItemScrollable}} {
        list_view: <ListView> {
            width: Fill
            height: Fill
            flow: Down
            spacing: 0.0
            catalog_item = <CatalogItem> {}
        }
    }

    CatalogItemScreen = <View> {
        width: Fill
        height: Fill
        flow: Down
        show_bg: true,

        catalog_item_scrollable = <CatalogItemScrollable> {}
        bottom_bar = <BottomBar> {}
    }
}

#[derive(Live)]
pub struct CatalogItem {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[live]
    view: View,
    #[rust]
    catalog_item_id: u64,
}

impl LiveHook for CatalogItem {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, CatalogItem);
    }
}

impl Widget for CatalogItem {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let actions = self.view.handle_widget_event(cx, event);

        for action in actions {
            dispatch_action(cx, action);
        }
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn walk(&self) -> Walk {
        self.view.walk()
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.view.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.view.draw_walk_widget(cx, walk);
        WidgetDraw::done()
    }
}

#[derive(Debug, Clone, PartialEq, WidgetRef)]
pub struct CatalogItemRef(WidgetRef);

impl CatalogItemRef {
    pub fn set_catalog_item_id(&self, catalog_item_id: u64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.catalog_item_id = catalog_item_id;
        }
    }
}

#[derive(Live)]
pub struct CatalogItemScrollable {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[live]
    list_view: ListView,
}

impl LiveHook for CatalogItemScrollable {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, CatalogItemScrollable);
    }
}

impl Widget for CatalogItemScrollable {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let actions = self.list_view.handle_widget_event(cx, event);

        for action in actions {
            dispatch_action(cx, action);
        }
    }

    fn walk(&self) -> Walk {
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.list_view.redraw(cx)
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl CatalogItemScrollable {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        cx.begin_turtle(walk, self.layout);

        self.list_view.set_item_range(cx, 0, 2);

        while self.list_view.draw_widget(cx).hook_widget().is_some() {
            while let Some(item_id) = self.list_view.next_visible_item(cx) {
                if item_id > 0 {
                    continue;
                }
                let template = id!(catalog_item);
                let item = self.list_view.item(cx, item_id, template[0]).unwrap();

                item.draw_widget_all(cx);
            }
        }

        cx.end_turtle();
    }
}
