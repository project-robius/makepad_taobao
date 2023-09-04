use makepad_widgets::*;
use makepad_widgets::widget::WidgetCache;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    
    import crate::shared::styles::*;
    import crate::shared::helpers::*;

    SearchTerms = {{SearchTerms}} {
        width: Fill
        height: 30.0
        flow: Right,
        align: {x: 0.0, y: 0.0}

        label = <Label> {
            width: Fill
            height: Fit
            margin: { top: 10.0 }

            text: "通讯录"

            draw_text: {
                color: #333,
                text_style: <REGULAR_TEXT> {font_size: 10.0},
            }
        }

        animator: {
            carrousel = {
                default: show,
                show = {
                    from: {all: Snap}
                    apply: {label = {margin: {top: 10.0 }}}
                }

                keep = {
                    from: {all: Forward {duration: 3.0}}
                    apply: {label = {margin: {top: 10.0 }}}
                }

                hide = {
                    from: {all: Forward {duration: 0.3}}
                    apply: {label = {margin: {top: -30.0 }}}
                }
            }
        }
    }
}

#[derive(Live)]
pub struct SearchTerms {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,

    #[rust]
    next_view: NextFrame,

    #[rust]
    terms: Vec<String>,

    #[rust]
    current_term_index: i32,
}

impl LiveHook for SearchTerms {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, SearchTerms);
    }

    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.terms = vec![
            "平安果包装盒".to_string(),
            "火锅底料".to_string(),
            "園适合的帽子冬季".to_string(),
        ];
        self.current_term_index = 0;

        self.next_view = cx.new_next_frame();
        self.animator_play(cx, id!(carrousel.show));
    }
}

impl Widget for SearchTerms {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        if let Some(_ne) = self.next_view.is_event(event) {
            // Control animations when they are done
            if !self.animator_handle_event(cx, event).is_animating() {
                if self.animator.animator_in_state(cx, id!(carrousel.show)) {
                    self.animator_play(cx, id!(carrousel.keep));
                } else if self.animator.animator_in_state(cx, id!(carrousel.keep)) {
                    self.animator_play(cx, id!(carrousel.hide));
                } else if self.animator.animator_in_state(cx, id!(carrousel.hide)) {
                    // Show new term
                    self.current_term_index += 1;
                    if self.current_term_index >= self.terms.len() as i32 {
                        self.current_term_index = 0;
                    }
                    self.label(id!(label)).set_text(&self.terms[self.current_term_index as usize]);

                    self.animator_play(cx, id!(carrousel.show));
                }
            }
            self.label(id!(label)).redraw(cx);
            self.next_view = cx.new_next_frame();
        }

        // Fixes a bug where the carrousel would not animate returning from stack navigation
        if let Event::NextFrame(_) = event {
            self.next_view = cx.new_next_frame();
        }
    }

    fn walk(&self) -> Walk {
        self.view.walk()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx)
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.view.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.view.draw_walk_widget(cx, walk)
    }
}