use std::collections::VecDeque;

use crate::home::catalog_data::*;
use crate::home::home_content::icon_atlas::HashMap;
use crate::shared::clickable_view::ClickableViewAction;
use makepad_widgets::*;

const MAX_INITIALIZED_VIDEOS: usize = 4;

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

            // Heads-up: `keep_invisible: false` introduces UB with Video widget because the widget instances
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

    // Catalog item information to be displayed.
    #[rust]
    data: Vec<CatalogDataItem>,

    // Used for keeping tracking of selected item.
    #[rust]
    catalog_item_view_map: HashMap<u64, u64>,

    // Range of items displayed in the latest draw call.
    #[rust]
    current_visible_items_range: (u64, u64),

    // Direction in which the items are currently being scrolled.
    #[rust]
    scroll_direction: ScrollDirection,

    // Video widget items that are currently being scrolled.
    // The `back` of the queue represent the newest (on screen) video when scrolling down.
    // When scrolling up, the `front` of the queue wil be the newest (on screen) video (earliest video we've kept track of).
    #[rust]
    current_draw_videos: VecDeque<(u64, VideoRef)>,

    // Video widget items that have been scrolled off screen.
    #[rust]
    invisible_initialized_videos: VecDeque<(u64, VideoRef)>,

    // Video widget items that must be paused after they load their first frame.
    #[rust]
    videos_to_pause_on_first_frame: HashMap<u64, (u64, VideoRef)>,

    // First time the portal list is rendered, no scroll has happened yet.
    #[rust(true)]
    first_render: bool
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

            // Check for videos that need to be paused after they load their first frame.
            match list_action.as_widget_action().cast() {
                VideoAction::TextureUpdated => {
                    let tex_update_action = list_action.as_widget_action().unwrap();
                    if let Some((_item_id, video)) = self.videos_to_pause_on_first_frame.get(&tex_update_action.widget_uid.0) {
                        log!("Pausing queued video: {}", _item_id);
                        video.pause_playback(cx);
                        self.videos_to_pause_on_first_frame.remove(&tex_update_action.widget_uid.0);
                    }
                },
                _ => {}
            }

            // Check for clicks on catalog items.
            match list_action.as_widget_action().cast() {
                ClickableViewAction::Click => {
                    let widget_action = list_action.as_widget_action().unwrap();

                    for (_id, videoref) in self.invisible_initialized_videos.iter_mut() {
                        videoref.stop_and_cleanup_resources(cx);
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
        self.current_draw_videos.clear();

        let pairs_count: u64 = (self.data.len() / 2_usize) as u64;

        // When using Portal List's next_visible_item(cx), the item ids will be in increasing order
        // however, ocasionally the last item will be the earliest item in the list. 
        // For example we will receive items in the following order: 6, 7, 8, 9, 10, 5. 
        // Since the order in which we process the items is very important for our logic,
        // we keep track of this behaviour to ignore adding the last video to our current video queue.
        let mut is_item_first_in_list = false;

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, 50);

                let mut earliest_visible_item_id = None;
                let mut latest_visible_item_id = self.current_visible_items_range.1;

                while let Some(item_id) = list.next_visible_item(cx) {
                    if earliest_visible_item_id.is_none() {   
                        earliest_visible_item_id = Some(item_id);
                    }
                    if !self.first_render && item_id < latest_visible_item_id {
                        is_item_first_in_list = true;
                    }
                    latest_visible_item_id = item_id;

                    let (template, video_path) = match item_id {
                        0 => (id!(options), None),
                        1 => (id!(payments), None),
                        2 => (id!(featured_1), None),
                        3 => (id!(featured_2), None),
                        x if (x - 2) % 5 == 0 => (id!(catalog_pair_cups), Some(id!(left.image))),
                        x if (x - 2) % 5 == 1 => (id!(catalog_pair_ring), None),
                        x if (x - 2) % 5 == 2 => (id!(catalog_pair_protein), Some(id!(right.image))),
                        x if (x - 2) % 5 == 3 => (id!(catalog_pair_couch), None),
                        x if (x - 2) % 5 == 4 => (id!(catalog_pair_shoes), Some(id!(left.image))),
                        _ => (id!(catalog_pair_protein), Some(id!(right.image))),
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

                            if !is_item_first_in_list {
                                if let Some(video_path) = video_path {
                                    // TODO: To prevent seeing black frames on videos, begin playback here with pausing at the first frame.
                                    let video = catalog_pair.video(video_path);
                                    self.current_draw_videos.push_back((item_id, video));
                                }
                            }
                        }        
                    }
                    item.draw_all(cx, scope);
                }

                let new_visible_items_range = (earliest_visible_item_id.unwrap(), latest_visible_item_id);
                if new_visible_items_range.0 < self.current_visible_items_range.0 {
                    self.scroll_direction = ScrollDirection::Up;
                } else if new_visible_items_range.1 > self.current_visible_items_range.1 {
                    self.scroll_direction = ScrollDirection::Down;
                }                
                self.current_visible_items_range = new_visible_items_range;
            }
        }

        self.update_video_items_playback(cx);

        if self.first_render {
            self.first_render = false;
        }

        self.cleanup_video_items(cx);

        DrawStep::done()
    }
}


impl HomeContent {

    // Updates video playback so that the most recent (on screen) video is playing.
    // Traverses the current_draw_videos buffer, if scrolling down then it plays/resumes the latest, 
    // if scrolling up then plays/resume the earliest.
    fn update_video_items_playback(&mut self, cx: &mut Cx) {
        // TODO: Sometimes scrolling back up doesn't correctly resume video playback.
        let mut started_video = false;
        if self.scroll_direction == ScrollDirection::Down || self.first_render {
            log!("current draw videos item ids: {:?}", self.current_draw_videos.iter().map(|(id, _)| *id).collect::<Vec<u64>>());

            // Begin playback and immediately pause the upcoming video at the edge of the screen.
            if self.current_draw_videos.len() > 2 {
                if let Some((item_id, mut video)) = self.current_draw_videos.pop_back() {
                    if video.is_unprepared() &&!video.is_preparing() {
                        video.should_dispatch_texture_updates(true);
                        video.begin_playback(cx);
                        log!("Playing video and queueing for pause: {:?}", item_id);
                    }
                    // queue to pause when ready
                    self.videos_to_pause_on_first_frame.insert(video.widget_uid().0, (item_id, video));
                }
            }

            // Now, since we removed the upcoming video, calling pop_back will return the current, 
            // most visible (closer to center of the screen) video. We make sure to play it.
            if let Some((item_id, mut video)) = self.current_draw_videos.pop_back() {
                if video.is_paused() {
                    video.resume_playback(cx);
                    log!("Resuming video: {:?}", item_id);
                    started_video = true; 
                } else if video.is_unprepared() &&!video.is_preparing() {
                    video.begin_playback(cx);
                    log!("Playing video: {:?}", item_id);
                    started_video = true;
                }
            }
        } else if self.scroll_direction == ScrollDirection::Up {

            // Begin playback and immediately pause the upcoming video at the bottom edge of the screen.
            if self.current_draw_videos.len() > 2 {
                if let Some((item_id, video)) = self.current_draw_videos.pop_front() {
                    log!("Pausing video: {:?}", item_id);                        
                    video.pause_playback(cx);
                }
            }

            // Now since we removed the upcoming video, calling pop_front will return the current (most visible) video
            if let Some((item_id, mut video)) = self.current_draw_videos.pop_front() {
                if video.is_paused() {
                    video.resume_playback(cx);
                    log!("Resuming video: {:?}", item_id);
                    started_video = true; 
                } else if video.is_unprepared() &&!video.is_preparing() {
                    video.begin_playback(cx);
                    log!("Playing video: {:?}", item_id);
                    started_video = true;
                }
            }
        }
        
        // If a new video playback began in this draw, pause all other videos.
        if started_video {
            while let Some((item_id, video)) = self.current_draw_videos.pop_front() {
                log!("Pausing video: {:?}", item_id);
                video.pause_playback(cx);

                // push into list of invisible but initialized videos for later cleanup
                self.invisible_initialized_videos.push_back((item_id, video));
            }
        }
    }

    // Cleanup videos that are no longer visible and less likely to be played next.
    fn cleanup_video_items(&mut self, cx: &mut Cx) {
        if self.invisible_initialized_videos.len() > MAX_INITIALIZED_VIDEOS {
            if self.scroll_direction == ScrollDirection::Down {
                if let Some((_item_id, mut video)) = self.invisible_initialized_videos.pop_front() {
                    video.stop_and_cleanup_resources(cx);
                    log!("Released video: {:?}", _item_id);
                }
            } else {
                if let Some((_item_id, mut video)) = self.invisible_initialized_videos.pop_back() {
                    video.stop_and_cleanup_resources(cx);
                    log!("Released video: {:?}", _item_id);
                }
            }
        }
    }
}

pub type CatalogItemId = u64;

#[derive(Clone, Debug, DefaultNone)]
pub enum CatalogItemListAction {
    Click(CatalogItemId),
    None,
}

#[derive(Default, Debug, PartialEq)]
enum ScrollDirection {
    #[default]
    Down,
    Up
}
