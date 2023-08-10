use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::text_input::TextInput;
    import makepad_widgets::list_view::ListView;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;
    import crate::home::search_terms::SearchTerms;

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

    CATALOG_FLIP_FLOPS_IMG = dep("crate://self/resources/catalog/flip_flops.png")
    CATALOG_COSMETICS_IMG = dep("crate://self/resources/catalog/cosmetics.png")
    CATALOG_LIVING_FURNITURE_IMG = dep("crate://self/resources/catalog/living_furniture.png")
    CATALOG_SEUL_COLLECTION_IMG = dep("crate://self/resources/catalog/seul_collection.png")
    CATALOG_MEAL_IMG = dep("crate://self/resources/catalog/meal.png")
    CATALOG_PROTEIN_IMG = dep("crate://self/resources/catalog/protein.png")
    CATALOG_RING_IMG = dep("crate://self/resources/catalog/ring.png")
    CATALOG_ROUTER_IMG = dep("crate://self/resources/catalog/router.png")

    BUY_IT_BANNER_IMG = dep("crate://self/resources/buy_it_banner.png")

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

    TopBar = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Right, align: {x: 0.0, y: 0.5}, spacing: 10.0, padding: {left: 10., right: 10.}}

        <IconWithText> {
            image = { image: (WORLDWIDE_ICON) }
            caption = { label: "全球" }
        }

        <FillerX> {}

        <Label> {
            walk: {width: Fit, height: Fit}
            label: "推荐"
            draw_label: {
                color: #000
                text_style: <REGULAR_TEXT> {},
            }
        }
        <Label> {
            walk: {width: Fit, height: Fit}
            label: "特价"
            draw_label: {
                color: #000
                text_style: <REGULAR_TEXT> {},
            }
        }

        <FillerX> {}

        <IconWithText> {
            image = { image: (MEMBERSHIP_CODE_ICON) }
            caption = { label: "会员码" }
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

        input = <SearchTerms> {}

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
                label: "搜索"
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

        <IconWithText> {
            image = { image: (TAOBAO_COINS_IMG) }
            caption = { label: "淘宝币" }
        }

        <FillerX> {}

        <IconWithText> {
            image = { image: (HELP_CENTER_IMG) }
            caption = { label: "帮助中心" }
        }

        <FillerX> {}

        <IconWithText> {
            image = { image: (LOGISTICS_IMG) }
            caption = { label: "物流中心" }
        }

        <FillerX> {}

        <IconWithText> {
            image = { image: (PAYMENTS_IMG) }
            caption = { label: "支付中心" }
        }

        <FillerX> {}

        <IconWithText> {
            image = { image: (GLOBAL_DEALS_IMG) }
            caption = { label: "全球优惠" }
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
            label: "支付支持"
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
            caption = { label: "货运价格估计" }
        }
    }

    CatalogItem = <Box> {
        walk: {width: 166, height: Fit}
        layout: {flow: Down, align: {x: 0.5, y: 0.0}, spacing: 2.0}

        draw_bg: {
            color: #fff,
            radius: 8.
        }

        image = <Image> {
            walk: {width: 166, height: 166}
            image: (CATALOG_FLIP_FLOPS_IMG)

            // Override to have the upper corners rounded
            draw_bg: {
                instance radius: 8.
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
                    sdf.stroke(#fff, 1);
                    return sdf.result
                }
            }
        }
        info = <Frame> {
            walk: {width: Fill, height: Fit}
            layout: {flow: Down, padding: 5., spacing: 3.}
            title = <Label> {
                walk: {width: Fill, height: Fit}
                label: "男士人字拖 2023"
                draw_label: {
                    color: #000
                    text_style: <REGULAR_TEXT> {font_size: 12.0},
                }
            }

            subtitle = <Label> {
                walk: {width: Fill, height: Fit}
                label: "500+人付费"
                draw_label: {
                    color: #888
                    text_style: <REGULAR_TEXT> {font_size: 8.0},
                }
            }

            image = <Image> {
                walk: {width: 166, height: 36}
                layout: {flow: Down, spacing: 2., padding: 5.}
                image: (BUY_IT_BANNER_IMG)
                <Frame> {
                    walk: {width: Fit, height: Fit}
                    layout: {flow: Right, spacing: 1., align: {x: 0.0, y: 1.0}}
                    <Label> {
                        walk: {width: Fit, height: Fit, margin: {bottom: 2.}}
                        label: "¥"
                        draw_label: {
                            color: #fff
                            text_style: <REGULAR_TEXT> {font_size: 10.0},
                        }
                    }
                    <Label> {
                        walk: {width: Fit, height: Fit}
                        label: "58"
                        draw_label: {
                            color: #fff
                            text_style: <REGULAR_TEXT> {font_size: 14.0},
                        }
                    }
                }
                <Label> {
                    walk: {width: Fit, height: Fit}
                    label: "USD 8.11"
                    draw_label: {
                        color: #fff
                        text_style: <REGULAR_TEXT> {font_size: 8.0},
                    }
                }
            }
        }
    }

    CatalogItemWithOffer = <CatalogItem> {
        info = {
            <Box> {
                walk: {width: Fit, height: Fit}
                layout: {padding: 3.0}
                draw_bg: {
                    color: (LIGHT_ORANGE_COLOR),
                    radius: 2.
                }
                <Label> {
                    walk: {width: Fit, height: Fit}
                    label: "每300减40"
                    draw_label: {
                        color: (ORANGE_COLOR)
                        text_style: <REGULAR_TEXT> {font_size: 8.0},
                    }
                }
            }
        }
    }

    CatalogPair1 = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Right, align: {x: 0.5, y: 0.0}, padding: {left: 10., right: 10., bottom: 5.}}
        left = <CatalogItem> {}
        <FillerX> {}
        right = <CatalogItemWithOffer> {
            image = { image: (CATALOG_COSMETICS_IMG) }
        }
    }

    CatalogPair2 = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Right, align: {x: 0.5, y: 0.0}, padding: {left: 10., right: 10., bottom: 5.}}

        left = <CatalogItemWithOffer> {
            image = { image: (CATALOG_LIVING_FURNITURE_IMG) }
        }
        <FillerX> {}
        right = <CatalogItemWithOffer> {
            image = { image: (CATALOG_SEUL_COLLECTION_IMG) }
        }
    }

    CatalogPair3 = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Right, align: {x: 0.5, y: 0.0}, padding: {left: 10., right: 10., bottom: 5.}}

        left = <CatalogItem> {
            image = { image: (CATALOG_MEAL_IMG) }
        }
        <FillerX> {}
        right = <CatalogItem> {
            image = { image: (CATALOG_PROTEIN_IMG) }
        }
    }

    CatalogPair4 = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Right, align: {x: 0.5, y: 0.0}, padding: {left: 10., right: 10., bottom: 5.}}

        left = <CatalogItemWithOffer> {
            image = { image: (CATALOG_RING_IMG) }
        }
        <FillerX> {}
        right = <CatalogItem> {
            image = { image: (CATALOG_ROUTER_IMG) }
        }
    }

    HomeContent = {{HomeContent}} {
        layout: {flow: Down}
        list_view: <ListView> {
            walk: {width: Fill, height: Fill}
            layout: {flow: Down, spacing: 0.0}

            options = <Options> {}
            payments = <Payment> {}

            catalog_pair_1 = <CatalogPair1> {}
            catalog_pair_2 = <CatalogPair2> {}
            catalog_pair_3 = <CatalogPair3> {}
            catalog_pair_4 = <CatalogPair4> {}
        }
    }

    HomeScreen = <Frame> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}
        draw_bg: {
            color: (BACKGROUND_COLOR)
        }

        os_header_placeholder = <Box> {
            walk: {width: Fill, height: 50, margin: 0}
            layout: {flow: Right, spacing: 6.0, padding: 0}
        }
        <TopBar> {}
        <SearchBar> {}
        <HomeContent> {}
    }
}

#[derive(Debug, Clone)]
pub struct CatalogItemInfo {
    pub title: String,
    pub subtitle: String,
    pub price: String,
}


#[derive(Live)]
pub struct HomeContent {
    #[live]
    walk: Walk,
    #[live]
    layout: Layout,
    #[live]
    list_view: ListView,
    #[rust]
    data: Vec<CatalogItemInfo>,
}

impl LiveHook for HomeContent {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, HomeContent);
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.data = vec![
            CatalogItemInfo {
                title: "男士人字拖 2023".to_string(),
                subtitle: "500+人付费".to_string(),
                price: "58".to_string(),
            },
            CatalogItemInfo {
                title: "巧克力大地色调".to_string(),
                subtitle: "10000+人付费".to_string(),
                price: "8.9".to_string(),
            },
            CatalogItemInfo {
                title: "冰丝防沙发垫夏季垫t".to_string(),
                subtitle: "50+人付费".to_string(),
                price: "20.9".to_string(),
            },
            CatalogItemInfo {
                title: "胡萝卜奶锅婴儿不粘锅".to_string(),
                subtitle: "100+人付费".to_string(),
                price: "89".to_string(),
            },
            CatalogItemInfo {
                title: "脆皮芝士香蕉味网红".to_string(),
                subtitle: "200+人付费".to_string(),
                price: "18.5".to_string(),
            },
            CatalogItemInfo {
                title: "德国浓缩乳清蛋白y".to_string(),
                subtitle: "200+人付费".to_string(),
                price: "20.3".to_string(),
            },
            CatalogItemInfo {
                title: "不可思议的戒指".to_string(),
                subtitle: "500+人付费".to_string(),
                price: "100".to_string(),
            },
            CatalogItemInfo {
                title: "路由器AX6000".to_string(),
                subtitle: "13人付费".to_string(),
                price: "266".to_string(),
            },
            CatalogItemInfo {
                title: "男士人字拖 2023".to_string(),
                subtitle: "500+人付费".to_string(),
                price: "58".to_string(),
            },
            CatalogItemInfo {
                title: "巧克力大地色调".to_string(),
                subtitle: "10000+人付费".to_string(),
                price: "8.9".to_string(),
            },
            CatalogItemInfo {
                title: "冰丝防沙发垫夏季垫t".to_string(),
                subtitle: "50+人付费".to_string(),
                price: "20.9".to_string(),
            },
            CatalogItemInfo {
                title: "胡萝卜奶锅婴儿不粘锅".to_string(),
                subtitle: "100+人付费".to_string(),
                price: "89".to_string(),
            },
            CatalogItemInfo {
                title: "脆皮芝士香蕉味网红".to_string(),
                subtitle: "200+人付费".to_string(),
                price: "18.5".to_string(),
            },
            CatalogItemInfo {
                title: "德国浓缩乳清蛋白".to_string(),
                subtitle: "200+人付费".to_string(),
                price: "20.3".to_string(),
            },
        ];
    }
}

impl Widget for HomeContent {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let _actions = self.list_view.handle_widget_event(cx, event);

        for action in _actions {
            dispatch_action(cx, action);
        }
    }

    fn get_walk(&self) -> Walk {
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

impl HomeContent {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        let pairs_count: u64 = (self.data.len() / 2_usize) as u64;

        cx.begin_turtle(walk, self.layout);

        self.list_view.set_item_range(0, pairs_count + 1, 1);

        while self.list_view.draw_widget(cx).hook_widget().is_some() {
            while let Some(item_id) = self.list_view.next_visible_item(cx) {
                let template = match item_id {
                    0 => id!(options),
                    1 => id!(payments),
                    x if (x - 2) % 4 == 0 => id!(catalog_pair_1),
                    x if (x - 2) % 4 == 1 => id!(catalog_pair_2),
                    x if (x - 2) % 4 == 2 => id!(catalog_pair_3),
                    _ => id!(catalog_pair_4),
                };
                let item = self.list_view.get_item(cx, item_id, template[0]).unwrap();

                if item_id > 1 && item_id < pairs_count + 2 {
                    let data_left = &self.data[((item_id - 2) * 2) as usize];
                    let data_right = &self.data[((item_id - 2) * 2 + 1) as usize];

                    if let Some(mut catalog_pair) = item.borrow_mut::<Frame>() {
                        catalog_pair.get_label(id!(left.info.title)).set_label(&data_left.title);
                        catalog_pair.get_label(id!(left.info.subtitle)).set_label(&data_left.subtitle);

                        catalog_pair.get_label(id!(right.info.title)).set_label(&data_right.title);
                        catalog_pair.get_label(id!(right.info.subtitle)).set_label(&data_right.subtitle);
                    }
                }
                item.draw_widget_all(cx);
            }
        }

        cx.end_turtle();
    }
}
