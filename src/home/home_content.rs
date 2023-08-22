use crate::home::catalog_data::*;
use crate::home::home_content::icon_atlas::HashMap;
use crate::shared::clickable_frame::ClickableFrameAction;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::image::*;
    import makepad_widgets::list_view::ListView;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::clickable_frame::*;
    import crate::shared::helpers::*;
    import crate::home::helpers::*;
    import crate::home::carrousel::*;

    TAOBAO_COINS_IMG = dep("crate://self/resources/taobao_coins.png")
    HELP_CENTER_IMG = dep("crate://self/resources/help_center.png")
    LOGISTICS_IMG = dep("crate://self/resources/logistics_center.png")
    PAYMENTS_IMG = dep("crate://self/resources/payments_center.png")
    GLOBAL_DEALS_IMG = dep("crate://self/resources/global_deals.png")
    CREDIT_CARDS_IMG = dep("crate://self/resources/credit_cards.png")
    SHIPPING_ESTIMATE_IMG = dep("crate://self/resources/shipping_estimate.png")
    BUY_IT_BANNER_IMG = dep("crate://self/resources/buy_it_banner.png")

    CATALOG_FLIP_FLOPS_IMG = dep("crate://self/resources/catalog/flip_flops.png")
    CATALOG_COSMETICS_IMG = dep("crate://self/resources/catalog/cosmetics.png")
    CATALOG_LIVING_FURNITURE_IMG = dep("crate://self/resources/catalog/living_furniture.png")
    CATALOG_SEUL_COLLECTION_IMG = dep("crate://self/resources/catalog/seul_collection.png")
    CATALOG_MEAL_IMG = dep("crate://self/resources/catalog/meal.png")
    CATALOG_PROTEIN_IMG = dep("crate://self/resources/catalog/protein.png")
    CATALOG_RING_IMG = dep("crate://self/resources/catalog/ring.png")
    CATALOG_ROUTER_IMG = dep("crate://self/resources/catalog/router.png")

    FEATURED_1_IMG = dep("crate://self/resources/featured/featured_1.png")
    FEATURED_2_IMG = dep("crate://self/resources/featured/featured_2.png")
    FEATURED_3_IMG = dep("crate://self/resources/featured/featured_3.png")
    FEATURED_4_IMG = dep("crate://self/resources/featured/featured_4.png")
    FEATURED_5_IMG = dep("crate://self/resources/featured/featured_5.png")
    FEATURED_6_IMG = dep("crate://self/resources/featured/featured_6.png")
    FEATURED_7_IMG = dep("crate://self/resources/featured/featured_7.png")
    FEATURED_8_IMG = dep("crate://self/resources/featured/featured_8.png")

    Options = <Frame> {
      walk: {width: Fill, height: Fit}
      layout: {flow: Right, align: {x: 0.0, y: 0.5}, spacing: 10.0, padding: {left: 40., right: 40.}}

      <IconWithText> {
          image = { source: (TAOBAO_COINS_IMG) }
          caption = { label: "淘宝币" }
      }

      <FillerX> {}

      <IconWithText> {
          image = { source: (HELP_CENTER_IMG) }
          caption = { label: "帮助中心" }
      }

      <FillerX> {}

      <IconWithText> {
          image = { source: (LOGISTICS_IMG) }
          caption = { label: "物流中心" }
      }

      <FillerX> {}

      <IconWithText> {
          image = { source: (PAYMENTS_IMG) }
          caption = { label: "支付中心" }
      }

      <FillerX> {}

      <IconWithText> {
          image = { source: (GLOBAL_DEALS_IMG) }
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
          source: (CREDIT_CARDS_IMG)
      }

      <FillerX> {}

      <LineH> {
          walk: {width: 2.0, height: Fill}
      }

      <IconWithText> {
          image = { source: (SHIPPING_ESTIMATE_IMG), walk: {width: 32, height: 32} }
          caption = { label: "货运价格估计" }
      }
  }

    CatalogItem = <ClickableFrame> {
        walk: {width: 166, height: Fit}
        layout: {flow: Down, align: {x: 0.5, y: 0.0}, spacing: 2.0}

        draw_bg: {
            color: #fff,
            radius: 8.
        }

        image = <Image> {
            walk: {width: 166, height: 166}
            source: (CATALOG_FLIP_FLOPS_IMG)

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

            image = <ImageFrame> {
                walk: {width: 160, height: 40}
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

    CatalogPairBase = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Right, align: {x: 0.5, y: 0.0}, padding: {left: 10., right: 10., bottom: 5.}}
    }

    CatalogPair1 = <CatalogPairBase> {
        left = <CatalogItem> {}
        <FillerX> {}
        right = <CatalogItemWithOffer> {
            image = { source: (CATALOG_COSMETICS_IMG) }
        }
    }

    CatalogPair2 = <CatalogPairBase> {
        left = <CatalogItemWithOffer> {
            image = { source: (CATALOG_LIVING_FURNITURE_IMG) }
        }
        <FillerX> {}
        right = <CatalogItemWithOffer> {
            image = { source: (CATALOG_SEUL_COLLECTION_IMG) }
        }
    }

    CatalogPair3 = <CatalogPairBase> {
        left = <CatalogItem> {
            image = { source: (CATALOG_MEAL_IMG) }
        }
        <FillerX> {}
        right = <CatalogItem> {
            image = { source: (CATALOG_PROTEIN_IMG) }
        }
    }

    CatalogPair4 = <CatalogPairBase> {
        left = <CatalogItemWithOffer> {
            image = { source: (CATALOG_RING_IMG) }
        }
        <FillerX> {}
        right = <CatalogItem> {
            image = { source: (CATALOG_ROUTER_IMG) }
        }
    }

    Featured1 = <Frame> {
        walk: {width: Fill, height: Fit, margin: 10.}
        layout: {flow: Right, align: {x: 0.0, y: 0.0}, spacing: 10.0}

        <Carrousel> {}
        <Frame> {
            walk: {width: Fill, height: Fit}
            layout: {flow: Down, spacing: 0}
            <FeaturedBoxWithHighlightBox> {
                header = {
                    label = {
                        label: "淘宝直播"
                    }
                    highlight = {
                        label = {
                            label: "直播中"
                        }
                    }
                }
                content = {
                    image_container_1 = {
                        image = {
                            source: (FEATURED_1_IMG)
                            walk: {width: 60, height: 56}
                        }
                        label = {
                            label: "爆款直降"
                        }
                    }

                    image_container_2 = {
                        image = {
                            source: (FEATURED_2_IMG)
                            walk: {width: 46, height: 60}
                        }
                        label = {
                            label: "优质好货"
                        }
                    }
                }
            }
            <FeaturedBox> {
                header = {
                    label = {
                        label: "聚划算"
                    }
                }
                content = {
                    image_container_1 = {
                        image = {
                            source: (FEATURED_3_IMG)
                            walk: {width: 60, height: 62}
                        }
                        label = {
                            label: "爆款直降"
                        }
                    }

                    image_container_2 = {
                        image = {
                            source: (FEATURED_4_IMG)
                            walk: {width: 60, height: 49}
                        }
                        label = {
                            label: "抢大红包"
                        }
                    }
                }
            }
        }
    }

    Featured2 = <Frame> {
        walk: {width: Fill, height: Fit, margin: 10.}
        layout: {flow: Right, align: {x: 0.0, y: 0.0}, spacing: 10.0}

        <FeaturedBoxWithHighlightLabel> {
            header = {
                label = {
                    label: "百亿补贴"
                }
                highlight = {
                    label: "品牌正品"
                }
            }
            content = {
                image_container_1 = {
                    image = {
                        source: (FEATURED_5_IMG)
                        walk: {width: 54, height: 60}
                    }
                    label = {
                        label: "爆款直降"
                    }
                }

                image_container_2 = {
                    image = {
                        source: (FEATURED_6_IMG)
                        walk: {width: 60, height: 36}
                    }
                    label = {
                        label: "优质好货"
                    }
                }
            }
        }
        <FeaturedBoxWithHighlightLabel> {
            header = {
                label = {
                    label: "有好货"
                }
                highlight = {
                    label: "用过才好说"
                    draw_label: {
                        color: #x93edea
                    }
                }
            }
            content = {
                image_container_1 = {
                    image = {
                        source: (FEATURED_7_IMG)
                        walk: {width: 54, height: 62}
                    }
                    label = {
                        label: "爆款直降"
                    }
                }

                image_container_2 = {
                    image = {
                        source: (FEATURED_8_IMG)
                        walk: {width: 60, height: 28}
                    }
                    label = {
                        label: "优质好货"
                    }
                }
            }
        }
    }

    HomeContent = {{HomeContent}} {
        layout: {flow: Down}
        list_view: <ListView> {
            walk: {width: Fill, height: Fill}
            layout: {flow: Down, spacing: 0.0}

            options = <Options> {}
            payments = <Payment> {}
            featured_1 = <Featured1> {}
            featured_2 = <Featured2> {}

            catalog_pair_1 = <CatalogPair1> {}
            catalog_pair_2 = <CatalogPair2> {}
            catalog_pair_3 = <CatalogPair3> {}
            catalog_pair_4 = <CatalogPair4> {}
        }
    }
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
    data: Vec<CatalogDataItem>,
    #[rust]
    catalog_item_view_map: HashMap<u64, u64>,
}

impl LiveHook for HomeContent {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, HomeContent);
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.data = CatalogData::new().items;
    }
}

impl Widget for HomeContent {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let widget_uid = self.widget_uid();
        self.handle_event_with(cx, event, &mut |cx, action| {
            dispatch_action(cx, WidgetActionItem::new(action.into(), widget_uid));
        });
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
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, CatalogItemListAction),
    ) {
        let mut actions = Vec::new();
        self.list_view
            .handle_widget_event_with(cx, event, &mut |_, action| {
                if let Some(catalog_item_id) = self.catalog_item_view_map.get(&action.widget_uid.0)
                {
                    actions.push((catalog_item_id, action));
                }
            });

        for (catalog_item_id, action) in actions {
            match action.action() {
                ClickableFrameAction::Click => {
                    dispatch_action(cx, CatalogItemListAction::Click(*catalog_item_id))
                }
                _ => (),
            }
        }
    }

    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        let pairs_count: u64 = (self.data.len() / 2_usize) as u64;

        cx.begin_turtle(walk, self.layout);

        self.list_view.set_item_range(0, pairs_count + 3, 1);

        while self.list_view.draw_widget(cx).hook_widget().is_some() {
            while let Some(item_id) = self.list_view.next_visible_item(cx) {
                let template = match item_id {
                    0 => id!(options),
                    1 => id!(payments),
                    2 => id!(featured_1),
                    3 => id!(featured_2),
                    x if (x - 2) % 4 == 0 => id!(catalog_pair_1),
                    x if (x - 2) % 4 == 1 => id!(catalog_pair_2),
                    x if (x - 2) % 4 == 2 => id!(catalog_pair_3),
                    _ => id!(catalog_pair_4),
                };
                let item = self.list_view.get_item(cx, item_id, template[0]).unwrap();

                if item_id > 2 && item_id < pairs_count + 4 {
                    let data_left = &self.data[((item_id - 3) * 3) as usize];
                    let data_right = &self.data[((item_id - 3) * 3 + 1) as usize];

                    self.catalog_item_view_map
                        .insert(item.get_widget(id!(left)).widget_uid().0, data_left.id);
                    self.catalog_item_view_map
                        .insert(item.get_widget(id!(right)).widget_uid().0, data_right.id);

                    if let Some(mut catalog_pair) = item.borrow_mut::<Frame>() {
                        catalog_pair
                            .get_label(id!(left.info.title))
                            .set_label(&data_left.title);
                        catalog_pair
                            .get_label(id!(left.info.subtitle))
                            .set_label(&data_left.subtitle);

                        catalog_pair
                            .get_label(id!(right.info.title))
                            .set_label(&data_right.title);
                        catalog_pair
                            .get_label(id!(right.info.subtitle))
                            .set_label(&data_right.subtitle);
                    }
                }
                item.draw_widget_all(cx);
            }
        }

        cx.end_turtle();
    }
}

pub type CatalogItemId = u64;

#[derive(Debug, Clone, WidgetAction)]
pub enum CatalogItemListAction {
    Click(CatalogItemId),
    None,
}
