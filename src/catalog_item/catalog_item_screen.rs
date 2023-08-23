use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::list_view::ListView;
    import makepad_widgets::text_input::TextInput;
    import makepad_widgets::image::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;

    CURVED_ARROW_IMG = dep("crate://self/resources/curved_arrow.png")

    CatalogItem = {{CatalogItem}} {
        walk: {width: Fill, height: Fill}
        layout: {flow: Right, spacing: 10., padding: 0.}

        frame: <Frame> {
            walk: {width: Fill, height: Fill}
            layout: {flow: Down, spacing: 0.}
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
