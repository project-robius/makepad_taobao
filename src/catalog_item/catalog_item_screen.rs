use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

// TODO: Use the item ID to get the data dynamically

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::view::*;
    import makepad_widgets::text::*;
    import makepad_widgets::list_view::ListView;
    import makepad_widgets::text_input::TextInput;
    import makepad_widgets::image::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;

    CATALOG_RING_IMG = dep("crate://self/resources/catalog/ring.png")
    AVATAR_IMG = dep("crate://self/resources/default_avatar.png")
    STAR_IMG = dep("crate://self/resources/star.png")

    Section = <Box> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Down, spacing: 10., padding: 10.}
        show_bg: true
        draw_bg: {
            color: #fff
            radius: 5.
        }
    }

    RecomendedItem = <View> {
        walk: {width: Fit, height: Fit}
        layout: {flow: Down, spacing: 10.}
        <Image> {
            walk: {width: 100, height: 100}
            source: (CATALOG_RING_IMG)
        }
        <Label> {
            walk: {width: Fill, height: Fit}
            text: "每每减每2023每减每减每"
            draw_label: {
                color: #0
                text_style: <REGULAR_TEXT> {font_size: 10},
                wrap: Word
            }
        }
        <Label> {
            walk: {width: Fit, height: Fit}
            text: "¥50"
            draw_label: {
                color: #f15603
                text_style: <TITLE_TEXT> {font_size: 14.0},
            }
        }
    }


    CatalogItem = {{CatalogItem}} {
        frame: <View> {
            walk: {width: Fill, height: Fit}
            layout: {flow: Down, spacing: 0., padding: 0.}
            show_bg: true
            draw_bg: {
                color: (BACKGROUND_COLOR)
            }

            main_image = <Image> {
                walk: {width: Fill, height: 300}
                source: (CATALOG_RING_IMG)
            }

            sections = <View> {
                walk: {width: Fill, height: Fit}
                layout: {flow: Down, spacing: 10., padding: 10.}

                <Section> {
                    price_container = <View> {
                        walk: {width: Fit, height: Fit}
                        layout: {align: {y: 0.5}, flow: Right, spacing: 5.}
                        <Label> {
                            walk: {width: Fit, height: Fit}
                            text: "¥50"
                            draw_label: {
                                color: #f15603
                                text_style: <TITLE_TEXT> {font_size: 14.0},
                            }
                        }
                        <Box> {
                            walk: {width: Fit, height: Fit}
                            layout: {padding: {right:10.0, left: 10., top: 5, bottom: 5}}
                            draw_bg: {
                                color: #f15603,
                                radius: 7.
                            }
                            <Label> {
                                walk: {width: Fit, height: Fit}
                                text: "每每每每 ¥3000"
                                draw_label: {
                                    color: #fff
                                    text_style: <TITLE_TEXT> {font_size: 14.0},
                                }
                            }
                        }
                    }
                    <View> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Right}

                        <Box> {
                            walk: {width: Fit, height: Fit}
                            layout: {padding: 3.0}
                            draw_bg: {
                                color: #ffe9e5,
                                radius: 2.
                            }
                            <Label> {
                                walk: {width: Fit, height: Fit}
                                text: "每300减40"
                                draw_label: {
                                    color: (ORANGE_COLOR)
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <FillerX> {}

                        <Label> {
                            walk: {width: Fit, height: Fit}
                            text: "每减 >"
                            draw_label: {
                                color: (ORANGE_COLOR)
                                text_style: <REGULAR_TEXT> {font_size: 10.0},
                            }
                        }
                    }

                    <View> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Right}

                        <Box> {
                            walk: {width: Fit, height: Fit}
                            layout: {padding: {right: 6., left: 6., top: 3, bottom: 3}}
                            draw_bg: {
                                color: #fa0322,
                                radius: 4.
                            }
                            <Label> {
                                walk: {width: Fit, height: Fit}
                                text: "每每"
                                draw_label: {
                                    color: #fff
                                    text_style: <TITLE_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <FillerX> {}

                        <Label> {
                            walk: {width: Fit, height: Fit}
                            text: "每减 100+"
                            draw_label: {
                                color: #9d
                                text_style: <REGULAR_TEXT> {font_size: 10.0},
                            }
                        }
                    }

                    <Label> {
                        walk: {width: Fill, height: Fit}
                        text: "萨帕托斯萨帕托斯萨帕托斯萨帕托斯萨帕托斯萨帕托斯萨帕托斯斯萨帕托斯"
                        draw_label: {
                            color: #0
                            text_style: <TITLE_TEXT> {font_size: 10.0},
                        }
                    }
                }

                <Section> {
                    layout: {spacing: 20}
                    <View> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Right, spacing: 20.}

                        <View> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, align: {x: 0, y: 0}}
                            <Label> {
                                text: "每减"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <View> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Down, spacing: 3.}
                            <Label> {
                                text: "每减每减每减"
                                draw_label: {
                                    color: #0
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                            <View> {
                                walk: {width: Fit, height: Fit}
                                layout: {flow: Right, spacing: 3.}
                                <Image> {
                                    walk: {width: 30, height: 30}
                                    source: (CATALOG_RING_IMG)
                                }
                                <Image> {
                                    walk: {width: 30, height: 30}
                                    source: (CATALOG_RING_IMG)
                                }
                                <Image> {
                                    walk: {width: 30, height: 30}
                                    source: (CATALOG_RING_IMG)
                                }
                            }
                        }

                        <View> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, align: {x: 1, y: 1}}
                            <Label> {
                                text: ">"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                    }

                    <View> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Right, spacing: 20.}

                        <View> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, align: {x: 0, y: 0}}
                            <Label> {
                                text: "每减"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <View> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Down, spacing: 5.}
                            <Label> {
                                text: "每减每减每减"
                                draw_label: {
                                    color: #0
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                            <Label> {
                                text: "每减: 每减"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <View> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, align: {x: 1, y: 1}}
                            <Label> {
                                text: ">"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                    }

                    <View> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Right, spacing: 20.}

                        <View> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, align: {x: 0, y: 0}}
                            <Label> {
                                text: "每减"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <View> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Down, spacing: 5.}

                            <Label> {
                                text: "每减每减每减"
                                draw_label: {
                                    color: #0
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <View> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, align: {x: 1, y: 1}}
                            <Label> {
                                text: ">"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                    }
                }

                <Section> {
                    layout: {spacing: 30.}
                    <View> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Down, spacing: 10.}
                        <View> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Right, spacing: 10., align: {y: 0.5}}

                            <Label> {
                                text: "每减每 (400+)"
                                draw_label: {
                                    color: #0
                                    text_style: <TITLE_TEXT> {font_size: 11},
                                }
                            }

                            <FillerX> {}

                            <Label> {
                                text: "每减每 >"
                                draw_label: {
                                    color: (ORANGE_COLOR)
                                    text_style: <TITLE_TEXT> {font_size: 10},
                                }
                            }
                        }

                        <View> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Right, spacing: 5}
                            <Box> {
                                walk: {width: Fit, height: Fit}
                                layout: {padding: 6.0}
                                draw_bg: {
                                    color: #ffe9e5,
                                    radius: 3.
                                }
                                <Label> {
                                    walk: {width: Fit, height: Fit}
                                    text: "每减 (40)"
                                    draw_label: {
                                        color: #0,
                                        text_style: <REGULAR_TEXT> {font_size: 10.0},
                                    }
                                }
                            }
                            <Box> {
                                walk: {width: Fit, height: Fit}
                                layout: {padding: 6.0}
                                draw_bg: {
                                    color: #ffe9e5,
                                    radius: 3.
                                }
                                <Label> {
                                    walk: {width: Fit, height: Fit}
                                    text: "每减 (70)"
                                    draw_label: {
                                        color: #0,
                                        text_style: <REGULAR_TEXT> {font_size: 10.0},
                                    }
                                }
                            }
                            <Box> {
                                walk: {width: Fit, height: Fit}
                                layout: {padding: 6.0}
                                draw_bg: {
                                    color: #ffe9e5,
                                    radius: 3.
                                }
                                <Label> {
                                    walk: {width: Fit, height: Fit}
                                    text: "每减 (90)"
                                    draw_label: {
                                        color: #0,
                                        text_style: <REGULAR_TEXT> {font_size: 10.0},
                                    }
                                }
                            }
                        }

                        <View> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Down, spacing: 10.}

                            <View> {
                                walk: {width: Fill, height: Fit}
                                layout: {flow: Right, spacing: 5, align: {y: 0.5}}

                                <Image> {
                                    walk: {width: 25, height: 25}
                                    source: (AVATAR_IMG)
                                }
                                <Label> {
                                    walk: {width: Fit, height: Fit}
                                    text: "每减减减"
                                    draw_label: {
                                        color: #0,
                                        text_style: <REGULAR_TEXT> {font_size: 8.0},
                                    }
                                }
                            }
                            <Label> {
                                walk: {width: Fit, height: Fit}
                                text: "每减每减"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                        <View> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Down, spacing: 10.}

                            <View> {
                                walk: {width: Fill, height: Fit}
                                layout: {flow: Right, spacing: 5, align: {y: 0.5}}

                                <Image> {
                                    walk: {width: 25, height: 25}
                                    source: (AVATAR_IMG)
                                }
                                <Label> {
                                    walk: {width: Fit, height: Fit}
                                    text: "每减减减"
                                    draw_label: {
                                        color: #0,
                                        text_style: <REGULAR_TEXT> {font_size: 8.0},
                                    }
                                }
                            }
                            <Label> {
                                walk: {width: Fit, height: Fit}
                                text: "每每减每减减"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                    }

                    <View> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Down, spacing: 20.}

                        <View> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Right, spacing: 10., align: {y: 0.5}}

                            <Label> {
                                text: "每减每 (5)"
                                draw_label: {
                                    color: #0
                                    text_style: <TITLE_TEXT> {font_size: 11},
                                }
                            }

                            <FillerX> {}

                            <Label> {
                                text: ">",
                                draw_label: {
                                    color: #9d
                                    text_style: <TITLE_TEXT> {font_size: 10},
                                }
                            }
                        }

                        <View> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Down, spacing: 10., padding: {left: 20.}}

                            <View> {
                                walk: {width: Fill, height: Fit}
                                layout: {flow: Right, spacing: 10.}

                                <Label> {
                                    text: "问",
                                    draw_label: {
                                        color: (ORANGE_COLOR)
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }

                                <Label> {
                                    text: "每减减减 减减",
                                    draw_label: {
                                        color: #0
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }

                                <FillerX> {}


                                <Label> {
                                    text: "6减减",
                                    draw_label: {
                                        color: #9d
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }
                            }
                            <View> {
                                walk: {width: Fill, height: Fit}
                                layout: {flow: Right, spacing: 10.}

                                <Label> {
                                    text: "问",
                                    draw_label: {
                                        color: (ORANGE_COLOR)
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }

                                <Label> {
                                    text: "每减减减 减减每减减减 减减",
                                    draw_label: {
                                        color: #0
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }

                                <FillerX> {}


                                <Label> {
                                    text: "6减减",
                                    draw_label: {
                                        color: #9d
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }
                            }
                        }

                    }
                }

                <Section> {
                    layout: {padding: 0}
                    <View> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Down, spacing: 20., padding: {top: 20, left: 15, right: 15, bottom: 25}}
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
                                sdf.stroke(#a6213e, 1);
                                return sdf.result
                            }
                        }

                        <View> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Right, spacing: 5.}
                            // TODO: Make rounded
                            <Image> {
                                walk: {width: 50, height: 50}
                                source: (CATALOG_RING_IMG)
                            }

                            <View> {
                                walk: {width: Fill, height: Fit}
                                layout: {flow: Down, spacing: 10.}

                                <Label> {
                                    text: "减减减减",
                                    draw_label: {
                                        color: #f
                                        text_style: <TITLE_TEXT> {font_size: 10},
                                    }
                                }

                                <View> {
                                    walk: {width: Fill, height: Fit}
                                    layout: {flow: Right, spacing: 5., align: {y: 0.5}}

                                    <Box> {
                                        walk: {width: Fit, height: Fit}
                                        layout: {padding: {right: 6., left: 6., top: 3, bottom: 3}}
                                        draw_bg: {
                                            color: #fa0322,
                                            radius: 4.
                                        }
                                        <Label> {
                                            walk: {width: Fit, height: Fit}
                                            text: "每每"
                                            draw_label: {
                                                color: #fff
                                                text_style: <TITLE_TEXT> {font_size: 10.0},
                                            }
                                        }
                                    }

                                    <Box> {
                                        walk: {width: Fit, height: Fit}
                                        layout: {spacing: 5., align: {y: 0.5}, padding: {right: 6., left: 6., top: 3, bottom: 3}}
                                        draw_bg: {
                                            color: #43,
                                            radius: 4.
                                        }
                                        <Label> {
                                            walk: {width: Fit, height: Fit}
                                            text: "每每"
                                            draw_label: {
                                                color: #f
                                                text_style: <REGULAR_TEXT> {font_size: 10.0},
                                            }
                                        }
                                        <View> {
                                            walk: {width: Fit, height: Fit}
                                            layout: {align: {y: 0.5}}

                                            <Image> {
                                                walk: {width: 10, height: 10}
                                                source: (STAR_IMG)
                                            }
                                            <Image> {
                                                walk: {width: 10, height: 10}
                                                source: (STAR_IMG)
                                            }
                                            <Image> {
                                                walk: {width: 10, height: 10}
                                                source: (STAR_IMG)
                                            }
                                            <Image> {
                                                walk: {width: 10, height: 10}
                                                source: (STAR_IMG)
                                            }
                                            <Image> {
                                                walk: {width: 10, height: 10}
                                                source: (STAR_IMG)
                                            }
                                        }
                                    }
                                    <Label> {
                                        walk: {width: Fit, height: Fit}
                                        text: "3.75每每"
                                        draw_label: {
                                            color: #f
                                            text_style: <REGULAR_TEXT> {font_size: 8.0},
                                        }
                                    }
                                }

                                <View> {
                                    walk: {width: Fill, height: Fit}
                                    layout: {flow: Right, spacing: 5., align: {y: 0.5}}

                                    <Label> {
                                        walk: {width: Fit, height: Fit}
                                        text: "每每3.75"
                                        draw_label: {
                                            color: #f
                                            text_style: <REGULAR_TEXT> {font_size: 8.0},
                                        }
                                    }
                                    <Label> {
                                        walk: {width: Fit, height: Fit}
                                        text: "每每3.75"
                                        draw_label: {
                                            color: #f
                                            text_style: <REGULAR_TEXT> {font_size: 8.0},
                                        }
                                    }
                                    <Label> {
                                        walk: {width: Fit, height: Fit}
                                        text: "每每3.75"
                                        draw_label: {
                                            color: #f
                                            text_style: <REGULAR_TEXT> {font_size: 8.0},
                                        }
                                    }
                                }
                            }
                        }
                        <View> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Right, spacing: 5., align: {x: 0.5}}
                            <Box> {
                                walk: {width: Fit, height: Fit}
                                layout: {padding: {right: 12., left: 12., top: 5, bottom: 5}}
                                draw_bg: {
                                    color: #f,
                                    radius: 4.
                                }
                                <Label> {
                                    walk: {width: Fit, height: Fit}
                                    text: "每每每每"
                                    draw_label: {
                                        color: (ORANGE_COLOR)
                                        text_style: <TITLE_TEXT> {font_size: 14.0},
                                    }
                                }
                            }
                            <Box> {
                                walk: {width: Fit, height: Fit}
                                layout: {padding: {right: 12., left: 12., top: 5, bottom: 5}}
                                draw_bg: {
                                    color: #f,
                                    radius: 4.
                                }
                                <Label> {
                                    walk: {width: Fit, height: Fit}
                                    text: "每每每每"
                                    draw_label: {
                                        color: (ORANGE_COLOR)
                                        text_style: <TITLE_TEXT> {font_size: 14.0},
                                    }
                                }
                            }
                        }
                    }

                    <View> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Down, spacing: 10., padding: 10.}
                        <View> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Right, spacing: 10., align: {y: 0.5}}

                            <Label> {
                                text: "每减每 (5)"
                                draw_label: {
                                    color: #0
                                    text_style: <TITLE_TEXT> {font_size: 11},
                                }
                            }

                            <FillerX> {}

                            <Label> {
                                walk: {width: Fit, height: Fit}
                                text: "每减 >"
                                draw_label: {
                                    color: (ORANGE_COLOR)
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                    }

                    <View> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Down, spacing: 5., align: {x: 0.5}}

                        <View> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, spacing: 5.}
                            <RecomendedItem> {}
                            <RecomendedItem> {}
                            <RecomendedItem> {}
                        }
                        <View> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, spacing: 5.}
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
            walk: {width: Fill, height: Fill}
            layout: {flow: Down, spacing: 0.0}
            catalog_item = <CatalogItem> {}
        }
    }

    CatalogItemScreen = <View> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}
        show_bg: true,

        catalog_item_scrollable = <CatalogItemScrollable> {}
    }
}

#[derive(Live)]
pub struct CatalogItem {
    #[live]
    walk: Walk,
    #[live]
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
    #[live]
    walk: Walk,
    #[live]
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

        self.list_view.set_item_range(0, 2, 0);

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
