use makepad_widgets::*;
use makepad_widgets::widget::WidgetCache;

live_design! {
    import makepad_widgets::frame::*;
    import makepad_widgets::image::*;
    import makepad_draw::shader::std::*;

    CARROUSEL_1_IMG = dep("crate://self/resources/catalog/carrousel_1.png")
    CARROUSEL_2_IMG = dep("crate://self/resources/catalog/carrousel_2.png")
    CARROUSEL_3_IMG = dep("crate://self/resources/catalog/carrousel_3.png")
    
    Carrousel = {{Carrousel}} {
        walk: {width: 166, height: Fit, margin: 10.}
        layout: {flow: Right, align: {x: 0.0, y: 0.5}}

        layout: {
            flow: Overlay,
            align: {x: 0.0, y: 0.0},
        }

        image1 = <Frame> {
            walk: {width: Fit, height: Fit}
            <Image> {
                walk: {width: 166, height: 226}
                source: (CARROUSEL_1_IMG)
            }
        }

        image2 = <Frame> {
            walk: {width: Fit, height: Fit}
            <Image> {
                walk: {width: 166, height: 226}
                source: (CARROUSEL_2_IMG)
            }
        }

        image3 = <Frame> {
            walk: {width: Fit, height: Fit}
            <Image> {
                walk: {width: 166, height: 226}
                source: (CARROUSEL_3_IMG)
            }
        }

        offset: 0

        state: {
            carrousel = {
                default: display,
                display = {
                    from: {all: Forward {duration: 1.0}}
                    // Bug: Constants are not working as part of an live state value
                    apply: {offset: 0.0}
                }

                keep = {
                    from: {all: Forward {duration: 3.0}}
                    apply: {offset: 0.0}
                }

                restart = {
                    from: {all: Snap}
                    apply: {offset: 166.0}
                }
            }
        }
    }
}

#[derive(Live)]
pub struct Carrousel {
    #[deref]
    frame: Frame,

    #[live]
    offset: f64,

    #[state]
    state: LiveState,

    #[rust]
    next_frame: NextFrame,

    #[rust]
    images: Vec<LiveId>,

    #[rust(0)]
    current_image_index: i32,
}

impl LiveHook for Carrousel {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Carrousel);
    }

    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.images = vec![
            id!(image1)[0], id!(image2)[0], id!(image3)[0]
        ];

        self.get_frame(id!(image1)).set_visible(true);
        self.get_frame(id!(image2)).set_visible(false);
        self.get_frame(id!(image3)).set_visible(false);

        self.next_frame = cx.new_next_frame();
        self.animate_state(cx, id!(carrousel.keep));
    }
}

impl Widget for Carrousel {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        if let Some(_ne) = self.next_frame.is_event(event) {
            // Control animations when they are done
            if self.state_handle_event(cx, event).is_animating() {
                if self.state.is_in_state(cx, id!(carrousel.display)) {
                    let image_id = self.images[self.current_image_index as usize]; 
                    let mut image = self.get_frame(&[image_id]);
                    image.apply_over(cx, live!{walk: {margin: {left: (self.offset) }}});

                    let mut prev_index = self.current_image_index - 1;
                    if prev_index < 0 {
                        prev_index = self.images.len() as i32 - 1;
                    }
                    let prev_image_id = self.images[prev_index as usize];
                    image = self.get_frame(&[prev_image_id]);
                    image.apply_over(cx, live!{walk: {margin: {left: (self.offset - 166.0) }}})
                }
            } else {
                if self.state.is_in_state(cx, id!(carrousel.restart)) {
                    self.animate_state(cx, id!(carrousel.display));
                } else if self.state.is_in_state(cx, id!(carrousel.display)) {
                    self.animate_state(cx, id!(carrousel.keep));

                    let mut prev_index = self.current_image_index - 1;
                    if prev_index < 0 {
                        prev_index = self.images.len() as i32 - 1;
                    }

                    let image_id = self.images[prev_index as usize];
                    let image = self.get_frame(&[image_id]);
                    image.set_visible(false);
                } else if self.state.is_in_state(cx, id!(carrousel.keep)) {
                    self.current_image_index += 1;
                    if self.current_image_index >= self.images.len() as i32 {
                        self.current_image_index = 0;
                    }

                    let image_id = self.images[self.current_image_index as usize]; 
                    let mut image = self.get_frame(&[image_id]);

                    image.apply_over(cx, live!{walk: {margin: {left: 166.0 }}});
                    self.animate_state(cx, id!(carrousel.restart));
                    
                    image.set_visible(true);
                }
            } 
            self.redraw(cx);
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