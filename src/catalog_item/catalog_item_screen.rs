use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

// TODO: Use the item ID to get the data dynamically

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::list_view::ListView;
    import makepad_widgets::text_input::TextInput;
    import makepad_widgets::image::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;

    CATALOG_RING_IMG = dep("crate://self/resources/catalog/ring.png")
    AVATAR_IMG = dep("crate://self/resources/default_avatar.png")

    Section = <Box> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Down, spacing: 10., padding: 10.}
        show_bg: true
        draw_bg: {
            color: #fff
            radius: 5.
        }
    }
    CatalogItem = {{CatalogItem}} {
        frame: <Frame> {
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

            sections = <Frame> {
                walk: {width: Fill, height: Fit}
                layout: {flow: Down, spacing: 10., padding: 10.}

                <Section> {
                    price_container = <Frame> {
                        walk: {width: Fit, height: Fit}
                        layout: {align: {y: 0.5}, flow: Right, spacing: 5.}
                        <Label> {
                            walk: {width: Fit, height: Fit}
                            label: "¥50"
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
                                label: "每每每每 ¥3000"
                                draw_label: {
                                    color: #fff
                                    text_style: <TITLE_TEXT> {font_size: 14.0},
                                }
                            }
                        }
                    }
                    <Frame> {
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
                                label: "每300减40"
                                draw_label: {
                                    color: (ORANGE_COLOR)
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <FillerX> {}

                        <Label> {
                            walk: {width: Fit, height: Fit}
                            label: "每减 >"
                            draw_label: {
                                color: (ORANGE_COLOR)
                                text_style: <REGULAR_TEXT> {font_size: 10.0},
                            }
                        }
                    }

                    <Frame> {
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
                                label: "每每"
                                draw_label: {
                                    color: #fff
                                    text_style: <TITLE_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <FillerX> {}

                        <Label> {
                            walk: {width: Fit, height: Fit}
                            label: "每减 100+"
                            draw_label: {
                                color: #9d
                                text_style: <REGULAR_TEXT> {font_size: 10.0},
                            }
                        }
                    }

                    <Label> {
                        walk: {width: Fill, height: Fit}
                        label: "萨帕托斯萨帕托斯萨帕托斯萨帕托斯萨帕托斯萨帕托斯萨帕托斯斯萨帕托斯"
                        draw_label: {
                            color: #0
                            text_style: <TITLE_TEXT> {font_size: 10.0},
                        }
                    }
                }

                <Section> {
                    layout: {spacing: 20}
                    <Frame> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Right, spacing: 20.}

                        <Frame> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, align: {x: 0, y: 0}}
                            <Label> {
                                label: "每减"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <Frame> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Down, spacing: 3.}
                            <Label> {
                                label: "每减每减每减"
                                draw_label: {
                                    color: #0
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                            <Frame> {
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

                        <Frame> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, align: {x: 1, y: 1}}
                            <Label> {
                                label: ">"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                    }

                    <Frame> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Right, spacing: 20.}

                        <Frame> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, align: {x: 0, y: 0}}
                            <Label> {
                                label: "每减"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <Frame> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Down, spacing: 5.}
                            <Label> {
                                label: "每减每减每减"
                                draw_label: {
                                    color: #0
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                            <Label> {
                                label: "每减: 每减"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <Frame> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, align: {x: 1, y: 1}}
                            <Label> {
                                label: ">"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                    }

                    <Frame> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Right, spacing: 20.}

                        <Frame> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, align: {x: 0, y: 0}}
                            <Label> {
                                label: "每减"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <Frame> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Down, spacing: 5.}

                            <Label> {
                                label: "每减每减每减"
                                draw_label: {
                                    color: #0
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }

                        <Frame> {
                            walk: {width: Fit, height: Fit}
                            layout: {flow: Right, align: {x: 1, y: 1}}
                            <Label> {
                                label: ">"
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
                    <Frame> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Down, spacing: 10.}
                        <Frame> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Right, spacing: 10., align: {y: 0.5}}

                            <Label> {
                                label: "每减每 (400+)"
                                draw_label: {
                                    color: #0
                                    text_style: <TITLE_TEXT> {font_size: 11},
                                }
                            }

                            <FillerX> {}

                            <Label> {
                                label: "每减每 >"
                                draw_label: {
                                    color: (ORANGE_COLOR)
                                    text_style: <TITLE_TEXT> {font_size: 10},
                                }
                            }
                        }

                        <Frame> {
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
                                    label: "每减 (40)"
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
                                    label: "每减 (70)"
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
                                    label: "每减 (90)"
                                    draw_label: {
                                        color: #0,
                                        text_style: <REGULAR_TEXT> {font_size: 10.0},
                                    }
                                }
                            }
                        }

                        <Frame> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Down, spacing: 10.}

                            <Frame> {
                                walk: {width: Fill, height: Fit}
                                layout: {flow: Right, spacing: 5, align: {y: 0.5}}

                                <Image> {
                                    walk: {width: 25, height: 25}
                                    source: (AVATAR_IMG)
                                }
                                <Label> {
                                    walk: {width: Fit, height: Fit}
                                    label: "每减减减"
                                    draw_label: {
                                        color: #0,
                                        text_style: <REGULAR_TEXT> {font_size: 8.0},
                                    }
                                }
                            }
                            <Label> {
                                walk: {width: Fit, height: Fit}
                                label: "每减每减"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                        <Frame> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Down, spacing: 10.}

                            <Frame> {
                                walk: {width: Fill, height: Fit}
                                layout: {flow: Right, spacing: 5, align: {y: 0.5}}

                                <Image> {
                                    walk: {width: 25, height: 25}
                                    source: (AVATAR_IMG)
                                }
                                <Label> {
                                    walk: {width: Fit, height: Fit}
                                    label: "每减减减"
                                    draw_label: {
                                        color: #0,
                                        text_style: <REGULAR_TEXT> {font_size: 8.0},
                                    }
                                }
                            }
                            <Label> {
                                walk: {width: Fit, height: Fit}
                                label: "每每减每减减"
                                draw_label: {
                                    color: #9d
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                        }
                    }

                    <Frame> {
                        walk: {width: Fill, height: Fit}
                        layout: {flow: Down, spacing: 20.}

                        <Frame> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Right, spacing: 10., align: {y: 0.5}}

                            <Label> {
                                label: "每减每 (5)"
                                draw_label: {
                                    color: #0
                                    text_style: <TITLE_TEXT> {font_size: 11},
                                }
                            }

                            <FillerX> {}

                            <Label> {
                                label: ">",
                                draw_label: {
                                    color: #9d
                                    text_style: <TITLE_TEXT> {font_size: 10},
                                }
                            }
                        }

                        <Frame> {
                            walk: {width: Fill, height: Fit}
                            layout: {flow: Down, spacing: 10., padding: {left: 20.}}

                            <Frame> {
                                walk: {width: Fill, height: Fit}
                                layout: {flow: Right, spacing: 10.}

                                <Label> {
                                    label: "问",
                                    draw_label: {
                                        color: (ORANGE_COLOR)
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }

                                <Label> {
                                    label: "每减减减 减减",
                                    draw_label: {
                                        color: #0
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }

                                <FillerX> {}


                                <Label> {
                                    label: "6减减",
                                    draw_label: {
                                        color: #9d
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }
                            }
                            <Frame> {
                                walk: {width: Fill, height: Fit}
                                layout: {flow: Right, spacing: 10.}

                                <Label> {
                                    label: "问",
                                    draw_label: {
                                        color: (ORANGE_COLOR)
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }

                                <Label> {
                                    label: "每减减减 减减每减减减 减减",
                                    draw_label: {
                                        color: #0
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }

                                <FillerX> {}


                                <Label> {
                                    label: "6减减",
                                    draw_label: {
                                        color: #9d
                                        text_style: <REGULAR_TEXT> {font_size: 10},
                                    }
                                }
                            }

                        }
                    }
                }
            }
        }
    }

    CatalogItemScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}
        show_bg: true,

        catalog_item = <CatalogItem> {}
    }
}

#[derive(Live)]
pub struct CatalogItem {
    #[live]
    walk: Walk,
    #[live]
    layout: Layout,

    #[live]
    frame: Frame,
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
        let _actions = self.frame.handle_widget_event(cx, event);

        for action in _actions {
            dispatch_action(cx, action);
        }
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.frame.redraw(cx);
    }

    fn get_walk(&self) -> Walk {
        self.frame.get_walk()
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.frame.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.frame.draw_walk_widget(cx, walk);
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
