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
        walk: {width: 166, height: Fit}
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

        <Frame> {
            walk: {width: Fill, height: Fit, margin: {top: 205.}}
            layout: {
                flow: Right,
                align: {x: 0.5, y: 0.0},
                spacing: 5.0,
            }

            indicator1 = <Box> {
                walk: {width: 10.0, height: 10.0}
                draw_bg: {
                    color: #f60,
                    radius: 2.5
                }
            }

            indicator2 = <Box> {
                walk: {width: 20.0, height: 10.0}
                draw_bg: {
                    color: #f60,
                    radius: 2.5
                }
            }

            indicator3 = <Box> {
                walk: {width: 10.0, height: 10.0}
                draw_bg: {
                    color: #f60,
                    radius: 2.5
                }
            }
        }

        offset: 0

        state: {
            carrousel = {
                default: display,
                display = {
                    from: {all: Forward {duration: 0.4}}
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

enum CarrouselDirection {
    Forward,
    Backward,
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
    last_abs: f64,

    #[rust]
    images: Vec<LiveId>,

    #[rust(0)]
    current_image_index: i32,

    #[rust(1)]
    previous_image_index: i32,

    #[rust(CarrouselDirection::Forward)]
    direction: CarrouselDirection,
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
            if self.state_handle_event(cx, event).is_animating() {
                // Update images position as main animation progresses
                if self.state.is_in_state(cx, id!(carrousel.display)) {
                    let current_image_id = self.images[self.current_image_index as usize]; 
                    let mut current_image = self.get_frame(&[current_image_id]);

                    let prev_image_id = self.images[self.previous_image_index as usize];
                    let mut prev_image = self.get_frame(&[prev_image_id]);
                    
                    match self.direction {
                        CarrouselDirection::Forward => {
                            current_image.apply_over(cx, live!{walk: {margin: {left: (self.offset) }}});
                            prev_image.apply_over(cx, live!{walk: {margin: {left: (self.offset - 166.0) }}})
                        },
                        CarrouselDirection::Backward => {
                            current_image.apply_over(cx, live!{walk: {margin: {left: (-self.offset) }}});
                            prev_image.apply_over(cx, live!{walk: {margin: {left: (166.0 - self.offset) }}})
                        }   
                    }
                }
            } else {
                if self.state.is_in_state(cx, id!(carrousel.restart)) {
                    // Fires the animation of the carrousel again
                    self.animate_state(cx, id!(carrousel.display));
                } else if self.state.is_in_state(cx, id!(carrousel.display)) {
                    // Beings the period of time where the carrousel is stopped
                    self.animate_state(cx, id!(carrousel.keep));

                    let image_id = self.images[self.previous_image_index as usize];
                    let image = self.get_frame(&[image_id]);
                    image.set_visible(false);
                } else if self.state.is_in_state(cx, id!(carrousel.keep)) {
                    // Ends the period of time where the carrousel is stopped,
                    // prepares the next image
                    self.direction = CarrouselDirection::Forward;
                    self.previous_image_index = self.current_image_index;
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
            // TODO check if this is necessary
            self.redraw(cx);
            self.next_frame = cx.new_next_frame();
        }

        self.adjust_indicators_width(cx);

        match event.hits(cx, self.frame.area()) {
            Hit::FingerDown(fe) => {
                self.last_abs = fe.abs.x;
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    if (fe.abs.x - self.last_abs).abs() > 10.0 {
                        self.previous_image_index = self.current_image_index;
                        let initial_offset;

                        if fe.abs.x > self.last_abs {
                            self.direction = CarrouselDirection::Backward;
                            self.current_image_index -= 1;
                            if self.current_image_index < 0 {
                                self.current_image_index = self.images.len() as i32 - 1;
                            }
                            initial_offset = 166.0;
                        } else {
                            self.direction = CarrouselDirection::Forward;
                            self.current_image_index += 1;
                            if self.current_image_index >= self.images.len() as i32 {
                                self.current_image_index = 0;
                            }
                            initial_offset = -166.0;
                        }

                        let image_id = self.images[self.current_image_index as usize]; 
                        let mut image = self.get_frame(&[image_id]);
                        image.apply_over(cx, live!{walk: {margin: {left: (initial_offset) }}});
                        self.animate_state(cx, id!(carrousel.restart));
                        image.set_visible(true);

                        // TODO check if this is necessary
                        self.redraw(cx);
                    }
                }
            },
            _ => {}
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

impl Carrousel {
    fn adjust_indicators_width(&mut self, cx: &mut Cx) {
        let mut indicators_list = vec![
            self.get_frame(id!(indicator1)),
            self.get_frame(id!(indicator2)),
            self.get_frame(id!(indicator3)),
        ];

        for (i, indicator) in indicators_list.iter_mut().enumerate() {
            if i == self.current_image_index as usize {
                indicator.apply_over(cx, live!{walk: {width: 20.0}});
            } else {
                indicator.apply_over(cx, live!{walk: {width: 10.0}});
            }
        }
    }
}