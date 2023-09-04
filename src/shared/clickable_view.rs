use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    ClickableView = {{ClickableView}} {
        width: Fit
        height: Fit
        show_bg: true
        draw_bg: {
            color: #fff
        }
    }
}

#[derive(Live)]
pub struct ClickableView {
    #[deref]
    view: View,
}

impl LiveHook for ClickableView {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, ClickableView);
    }
}

#[derive(Clone, WidgetAction)]
pub enum ClickableViewAction {
    None,
    Click,
}

impl Widget for ClickableView {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let uid = self.widget_uid();
        self.handle_event_with(cx, event, &mut |cx, action| {
            dispatch_action(cx, WidgetActionItem::new(action.into(), uid));
        });
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

impl ClickableView {
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, ClickableViewAction),
    ) {
        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(fe) => {
                if fe.was_tap() {
                    dispatch_action(cx, ClickableViewAction::Click);
                }
            }
            _ => (),
        }
    }
}

#[derive(Debug, Clone, PartialEq, WidgetRef)]
pub struct ClickableViewRef(WidgetRef);

impl ClickableViewRef {
    pub fn clicked(&self, actions: &WidgetActions) -> bool {
        if let Some(item) = actions.find_single_action(self.widget_uid()) {
            if let ClickableViewAction::Click = item.action() {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, WidgetSet)]
pub struct ClickableViewSet(WidgetSet);

impl ClickableViewSet {
    pub fn clicked(&self, actions: &WidgetActions) -> bool {
        for clickable_view in self.iter() {
            if clickable_view.clicked(actions) {
                return true;
            }
        }
        false
    }
}
