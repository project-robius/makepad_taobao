use makepad_widgets::*;
use makepad_widgets::widget::WidgetCache;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::label::*;
    import makepad_widgets::text_input::TextInput;
    
    import crate::shared::styles::*;
    import crate::shared::helpers::*;

    SearchTerms = {{SearchTerms}} {
        walk: {width: Fill, height: 30.0}
        layout: {
            flow: Right,
            align: {x: 0.0, y: 0.0},
        }

        label = <Label> {
            walk: {width: Fill, height: Fit, margin: { top: 10.0 } }
            label: "通讯录"

            draw_label: {
                color: #333,
                text_style: <REGULAR_TEXT> {font_size: 10.0},
            }
        }

        state: {
            carrousel = {
                default: show,
                show = {
                    from: {all: Snap}
                    apply: {label = {walk: {margin: {top: 10.0 }}}}
                }

                keep = {
                    from: {all: Forward {duration: 3.0}}
                    apply: {label = {walk: {margin: {top: 10.0 }}}}
                }

                hide = {
                    from: {all: Forward {duration: 0.3}}
                    apply: {label = {walk: {margin: {top: -30.0 }}}}
                }
            }
        }
    }
}

#[derive(Live)]
pub struct SearchTerms {
    #[deref]
    frame: Frame,

    #[state]
    state: LiveState,

    #[rust]
    next_frame: NextFrame,

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

        self.next_frame = cx.new_next_frame();
        self.animate_state(cx, id!(carrousel.show));
    }
}

impl Widget for SearchTerms {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        if let Some(_ne) = self.next_frame.is_event(event) {
            // Control animations when they are done
            if !self.state_handle_event(cx, event).is_animating() {
                if self.state.is_in_state(cx, id!(carrousel.show)) {
                    self.animate_state(cx, id!(carrousel.keep));
                } else if self.state.is_in_state(cx, id!(carrousel.keep)) {
                    self.animate_state(cx, id!(carrousel.hide));
                } else if self.state.is_in_state(cx, id!(carrousel.hide)) {
                    // Show new term
                    self.current_term_index += 1;
                    if self.current_term_index >= self.terms.len() as i32 {
                        self.current_term_index = 0;
                    }
                    self.get_label(id!(label)).set_label(&self.terms[self.current_term_index as usize]);

                    self.animate_state(cx, id!(carrousel.show));
                }
            }
            self.get_label(id!(label)).redraw(cx);
            self.next_frame = cx.new_next_frame();
        }
    }

    fn get_walk(&self) -> Walk {
        self.frame.get_walk()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.frame.redraw(cx)
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.frame.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.frame.draw_walk_widget(cx, walk)
    }
}