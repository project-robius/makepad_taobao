use std::collections::VecDeque;

use crate::home::catalog_data::*;
use crate::home::home_content::icon_atlas::HashMap;
use crate::shared::clickable_view::ClickableViewAction;
use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::clickable_view::*;
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
    CATALOG_PROTEIN_VIDEO = dep("crate://self/resources/catalog/protein.mp4")
    SHOES_VIDEO = dep("crate://self/resources/catalog/shoes.mp4")
    CATALOG_CUPS_VIDEO = dep("crate://self/resources/catalog/cups.mp4")

    FEATURED_1_IMG = dep("crate://self/resources/featured/featured_1.png")
    FEATURED_2_IMG = dep("crate://self/resources/featured/featured_2.png")
    FEATURED_3_IMG = dep("crate://self/resources/featured/featured_3.png")
    FEATURED_4_IMG = dep("crate://self/resources/featured/featured_4.png")
    FEATURED_5_IMG = dep("crate://self/resources/featured/featured_5.png")
    FEATURED_6_IMG = dep("crate://self/resources/featured/featured_6.png")
    FEATURED_7_IMG = dep("crate://self/resources/featured/featured_7.png")
    FEATURED_8_IMG = dep("crate://self/resources/featured/featured_8.png")

    Options = <View> {
        width: Fill
        height: Fit
        padding: {left: 40., right: 40.}
        flow: Right
        align: {x: 0.0, y: 0.5}
        spacing: 10.0

        <IconWithText> {
            image = { source: (TAOBAO_COINS_IMG) }
            caption = { text: "淘宝币" }
        }

        <FillerX> {}

        <IconWithText> {
            image = { source: (HELP_CENTER_IMG) }
            caption = { text: "帮助中心" }
        }

        <FillerX> {}

        <IconWithText> {
            image = { source: (LOGISTICS_IMG) }
            caption = { text: "物流中心" }
        }

        <FillerX> {}

        <IconWithText> {
            image = { source: (PAYMENTS_IMG) }
            caption = { text: "支付中心" }
        }

        <FillerX> {}

        <IconWithText> {
            image = { source: (GLOBAL_DEALS_IMG) }
            caption = { text: "全球优惠" }
        }
  }

  Payment = <RoundedView> {
      width: Fill,
      height: Fit,
      margin: 10.0
      padding: {left: 10., right: 10.}
      flow: Right
      align: {x: 0.0, y: 0.5}
      spacing: 6.0

      draw_bg: {
          color: #fff,
          radius: 4.
      }

      <Label> {
          width: Fit
          height: Fit
          margin: {top: 20., bottom: 20.}

          text: "支付支持"
          draw_text: {
              color: #000
              text_style: <REGULAR_TEXT> {font_size: 10.0},
          }
      }

      <Image> {
          width: 118, height: 18, source: (CREDIT_CARDS_IMG)
      }

      <FillerX> {}

      <LineH> { width: 2.0, height: Fill }

      <IconWithText> {
          image = { width: 32, height: 32, source: (SHIPPING_ESTIMATE_IMG) }
          caption = { text: "货运价格估计" }
      }
  }

    CatalogItem = <ClickableView> {
        width: Fill
        height: Fit

        show_bg: false

        container = <RoundedView> {
            width: Fill
            height: Fit
            flow: Down
            align: {x: 0.5, y: 0.0}
            spacing: 2.0

            draw_bg: {
                color: #fff,
                radius: 8.
            }

            image = <Image> {
                width: Fill, height: 166
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
            info = <View> {
                width: Fill
                height: Fit
                flow: Down
                padding: 5.
                spacing: 3.

                title = <Label> {
                    width: Fill
                    height: Fit

                    text: "男士人字拖 2023"
                    draw_text: {
                        color: #000
                        text_style: <REGULAR_TEXT> {font_size: 12.0},
                    }
                }

                subtitle = <Label> {
                    width: Fill
                    height: Fit

                    text: "500+人付费"
                    draw_text: {
                        color: #888
                        text_style: <REGULAR_TEXT> {font_size: 8.0},
                    }
                }

                <View> {
                    width: 160
                    height: 40
                    flow: Overlay

                    <Image> {
                        width: Fill
                        height: Fill
                        source: (BUY_IT_BANNER_IMG)
                    }

                    <View> {
                        width: Fill
                        height: Fill
                        padding: 5.
                        flow: Down
                        spacing: 2.

                        <View> {
                            width: Fit
                            height: Fit
                            flow: Right
                            spacing: 1.
                            align: {x: 0.0, y: 1.0}

                            <Label> {
                                width: Fit
                                height: Fit
                                margin: {bottom: 2.}

                                text: "¥"
                                draw_text: {
                                    color: #fff
                                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                                }
                            }
                            <Label> {
                                width: Fit
                                height: Fit

                                text: "58"
                                draw_text: {
                                    color: #fff
                                    text_style: <REGULAR_TEXT> {font_size: 14.0},
                                }
                            }
                        }
                        <Label> {
                            width: Fit
                            height: Fit

                            text: "USD 8.11"
                            draw_text: {
                                color: #fff
                                text_style: <REGULAR_TEXT> {font_size: 8.0},
                            }
                        }
                    }
                }
            }
        }
    }

    CatalogItemWithOffer = <CatalogItem> {
        container = {
            info = {
                <RoundedView> {
                    width: Fit
                    height: Fit
                    padding: 3.0
                    draw_bg: {
                        color: (LIGHT_ORANGE_COLOR),
                        radius: 2.
                    }
                    <Label> {
                        width: Fit
                        height: Fit
                        text: "每300减40"
                        draw_text: {
                            color: (ORANGE_COLOR)
                            text_style: <REGULAR_TEXT> {font_size: 8.0},
                        }
                    }
                }
            }
        }
    }

    ItemVideo = <Video> {
        autoplay: false,
        mute: true,
        is_looping: true
        width: 180, height: 180

        draw_bg: {
            instance radius: 8.
            image_pan: vec2(0.3, 0.3)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    1,
                    1,
                    self.rect_size.x - 2.0,
                    self.rect_size.y + self.radius * 2.0,
                    max(1.0, self.radius)
                )
                sdf.fill_keep(self.get_color())
                sdf.stroke(#fff, 1);
                return sdf.result
            }
        }
    }

    CatalogPairBase = <View> {
        width: Fill
        height: Fit
        padding: {left: 10., right: 10., bottom: 5.}
        flow: Right
        align: {x: 0.5, y: 0.0}
    }

    CatalogPairShoes = <CatalogPairBase> {
        left = <CatalogItem> {
            container = { image = <ItemVideo> {
                source: Dependency { path: (SHOES_VIDEO)}
                width: 180, height: 180
            } }
        }
        <FillerX> {
            width: 10.
        }
        right = <CatalogItemWithOffer> {
            container = { image = { source: (CATALOG_COSMETICS_IMG) } }
        }
    }

    CatalogPairCups = <CatalogPairBase> {
        align: {x: 0.5, y: 0.5}
        left = <CatalogItemWithOffer> {
            width: Fill
            height: Fit
            container = {
                width: Fill
                height: Fit
                image = <ItemVideo> {
                    source: Dependency { path: (CATALOG_CUPS_VIDEO)}
                    height: 203.4, width: 383.4
                }
            }
        }
    }

    CatalogPairProtein = <CatalogPairBase> {
        left = <CatalogItem> {
            container = { image = { source: (CATALOG_MEAL_IMG) } }
        }
        <FillerX> {
            width: 10.
        }
        right = <CatalogItem> {
            container = { image = <ItemVideo> {
                source: Dependency { path: (CATALOG_PROTEIN_VIDEO)}
                width: 180, height: 180
            } }
        }
    }

    CatalogPairRing = <CatalogPairBase> {
        left = <CatalogItemWithOffer> {
            container = { image = { source: (CATALOG_RING_IMG) } }
        }
        <FillerX> {
            width: 10.
        }
        right = <CatalogItem> {
            container = { image = { source: (CATALOG_ROUTER_IMG) } }
        }
    }

    CatalogPairCouch = <CatalogPairBase> {
        left = <CatalogItemWithOffer> {
            container = { image = { source: (CATALOG_LIVING_FURNITURE_IMG) } }
        }
        <FillerX> {
            width: 10.
        }
        right = <CatalogItemWithOffer> {
            container = { image = { source: (CATALOG_SEUL_COLLECTION_IMG) } }
        }
    }

    Featured1 = <View> {
        width: Fill,
        height: Fit,
        margin: 10.
        flow: Right,
        align: {x: 0.0, y: 0.0},
        spacing: 10.0

        <Carrousel> {}
        <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 0

            <FeaturedBoxWithHighlightBox> {
                header = {
                    label = {
                        text: "淘宝直播"
                    }
                    highlight = {
                        label = {
                            text: "直播中"
                        }
                    }
                }
                content = {
                    image_container_1 = {
                        image = {
                            width: 60, height: 56, source: (FEATURED_1_IMG)
                        }
                        label = {
                            text: "爆款直降"
                        }
                    }

                    image_container_2 = {
                        image = {
                            width: 46, height: 60, source: (FEATURED_2_IMG)
                        }
                        label = {
                            text: "优质好货"
                        }
                    }
                }
            }
            <FeaturedBox> {
                header = {
                    label = {
                        text: "聚划算"
                    }
                }
                content = {
                    image_container_1 = {
                        image = {
                            width: 60, height: 62, source: (FEATURED_3_IMG)
                        }
                        label = {
                            text: "爆款直降"
                        }
                    }

                    image_container_2 = {
                        image = {
                            width: 60, height: 49, source: (FEATURED_4_IMG)
                        }
                        label = {
                            text: "抢大红包"
                        }
                    }
                }
            }
        }
    }

    Featured2 = <View> {
        width: Fill
        height: Fit
        margin: 10.
        flow: Right
        align: {x: 0.0, y: 0.0}
        spacing: 10.0

        <FeaturedBoxWithHighlightLabel> {
            header = {
                label = {
                    text: "百亿补贴"
                }
                highlight = {
                    text: "品牌正品"
                }
            }
            content = {
                image_container_1 = {
                    image = {
                        width: 54, height: 60, source: (FEATURED_5_IMG)
                    }
                    label = {
                        text: "爆款直降"
                    }
                }

                image_container_2 = {
                    image = {
                        width: 60, height: 36, source: (FEATURED_6_IMG)
                    }
                    label = {
                        text: "优质好货"
                    }
                }
            }
        }
        <FeaturedBoxWithHighlightLabel> {
            header = {
                label = {
                    text: "有好货"
                }
                highlight = {
                    text: "用过才好说"
                    draw_text: {
                        color: #x93edea
                    }
                }
            }
            content = {
                image_container_1 = {
                    image = {
                        width: 54, height: 62, source: (FEATURED_7_IMG)
                    }
                    label = {
                        text: "爆款直降"
                    }
                }

                image_container_2 = {
                    image = {
                        width: 60, height: 28, source: (FEATURED_8_IMG)
                    }
                    label = {
                        text: "优质好货"
                    }
                }
            }
        }
    }

    HomeContent = {{HomeContent}} {
        list = <PortalList> {
            width: Fill
            height: Fill
            flow: Down
            spacing: 0.0
            // FIXME(julian): keep_invisible introduces UB with Video widget because the widget instances
            // get implicitly destroyed before stopping decoding on the platform.
            // We need to introduce lazy deleting of the widget instances.
            keep_invisible: true

        options = <Options> {}
            payments = <Payment> {}
            featured_1 = <Featured1> {}
            featured_2 = <Featured2> {}

            catalog_pair_shoes = <CatalogPairShoes> {}
            catalog_pair_cups = <CatalogPairCups> {}
            catalog_pair_protein = <CatalogPairProtein> {}
            catalog_pair_ring = <CatalogPairRing> {}
            catalog_pair_couch = <CatalogPairCouch> {}
        }
    }
}

#[derive(Live, Widget)]
pub struct HomeContent {
    #[deref]
    view: View,
    #[rust]
    data: Vec<CatalogDataItem>,
    #[rust]
    catalog_item_view_map: HashMap<u64, u64>,
    #[rust]
    currently_playing_videos: VecDeque<(u64, VideoRef)>,
}

impl LiveHook for HomeContent {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.data = CatalogData::new().items;
    }
}

impl Widget for HomeContent {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let widget_uid = self.widget_uid();
        for list_action in cx.capture_actions(|cx| self.view.handle_event(cx, event, scope)) {
            match list_action.as_widget_action().cast() {
                ClickableViewAction::Click => {
                    let widget_action = list_action.as_widget_action().unwrap();

                    for (_id, videoref) in self.currently_playing_videos.iter_mut() {
                        // FIXME(julian): we might want to stop and reset videos instead
                        videoref.pause_playback(cx);
                    }

                    if let Some(catalog_item_id) =
                        self.catalog_item_view_map.get(&widget_action.widget_uid.0)
                    {
                        cx.widget_action(
                            widget_uid,
                            &scope.path,
                            CatalogItemListAction::Click(*catalog_item_id),
                        );
                    }
                }
                ClickableViewAction::None => (),
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let pairs_count: u64 = (self.data.len() / 2_usize) as u64;

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, 1_000);
                while let Some(item_id) = list.next_visible_item(cx) {
                    let template = match item_id {
                        0 => id!(options),
                        1 => id!(payments),
                        2 => id!(featured_1),
                        3 => id!(featured_2),
                        x if (x - 2) % 5 == 0 => id!(catalog_pair_cups),
                        x if (x - 2) % 5 == 1 => id!(catalog_pair_ring),
                        x if (x - 2) % 5 == 2 => id!(catalog_pair_protein),
                        x if (x - 2) % 5 == 3 => id!(catalog_pair_couch),
                        x if (x - 2) % 5 == 4 => id!(catalog_pair_shoes),
                        _ => id!(catalog_pair_protein),
                    };

                    let item = list.item(cx, item_id, template[0]).unwrap();

                    if item_id > 3 && item_id < pairs_count + 5 {
                        let data_left = &self.data[((item_id - 4) * 2) as usize];
                        let data_right = &self.data[((item_id - 4) * 2 + 1) as usize];

                        self.catalog_item_view_map
                            .insert(item.widget(id!(left)).widget_uid().0, data_left.id);
                        self.catalog_item_view_map
                            .insert(item.widget(id!(right)).widget_uid().0, data_right.id);

                        if let Some(mut catalog_pair) = item.borrow_mut::<View>() {
                            catalog_pair
                                .label(id!(left.info.title))
                                .set_text(&data_left.title);
                            catalog_pair
                                .label(id!(left.info.subtitle))
                                .set_text(&data_left.subtitle);

                            catalog_pair
                                .label(id!(right.info.title))
                                .set_text(&data_right.title);
                            catalog_pair
                                .label(id!(right.info.subtitle))
                                .set_text(&data_right.subtitle);

                                let mut started_video = false;

                                //left
                                let mut video = catalog_pair.video(id!(left.image));
                                if video.is_unprepared() && !video.is_preparing() {
                                    video.begin_playback(cx);
    
                                    self.currently_playing_videos.push_back((item_id, video));
                                    started_video = true;
                                } else if video.is_paused() {
                                    // video.resume_playback(cx);
                                }
    
                                //right
                                let mut video = catalog_pair.video(id!(right.image));
                                if video.is_unprepared() && !video.is_preparing() {
                                    video.begin_playback(cx);
                                    self.currently_playing_videos.push_back((item_id, video));
                                    started_video = true;
                                } else if video.is_paused() {
                                    // video.resume_playback(cx);
                                }
    
                                if started_video {
                                    // Pause the earliest video that is still playing
                                    if self.currently_playing_videos.len() > 2 {
                                        if let Some((earliest_id, videoref)) =
                                            self.currently_playing_videos.pop_front()
                                        {
                                            if earliest_id != item_id {
                                                videoref.pause_playback(cx);
                                                // FIXME(julian): we might want to stop and reset videos instead, can't keep more than ~ 10 paused videos
                                                // videoref.stop_and_cleanup_resources(cx);
                                                // however that introduces performance hits, need to investigate further
                                            }
                                        }
                                    }
                                }
                        }
                    }
                    item.draw_all(cx, scope);
                }
            }
        }
        DrawStep::done()
    }
}

pub type CatalogItemId = u64;

#[derive(Clone, Debug, DefaultNone)]
pub enum CatalogItemListAction {
    Click(CatalogItemId),
    None,
}
