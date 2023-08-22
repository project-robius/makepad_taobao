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
            visible: false,
            walk: {width: Fit, height: Fit}
            <Image> {
                walk: {width: 166, height: 226}
                source: (CARROUSEL_1_IMG)
            }
        }

        image2 = <Frame> {
            visible: false,
            walk: {width: Fit, height: Fit}
            <Image> {
                walk: {width: 166, height: 226}
                source: (CARROUSEL_2_IMG)
            }
        }

        image3 = <Frame> {
            visible: false,
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

#[derive(Clone, Copy)]
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
    image_containers: Vec<FrameRef>,

    #[rust]
    indicators_widgets: Vec<FrameRef>,

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
        self.image_containers = vec![
            self.get_frame(id!(image1)),
            self.get_frame(id!(image2)),
            self.get_frame(id!(image3)),
        ];

        self.indicators_widgets = vec![
            self.get_frame(id!(indicator1)),
            self.get_frame(id!(indicator2)),
            self.get_frame(id!(indicator3)),
        ];

        let (_, current_image) = self.get_active_images_containers();
        current_image.set_visible(true);

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
        self.control_animation(cx, event);
        self.adjust_indicators_width(cx);
        self.handle_mouse_event(cx, event);
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
    fn control_animation(&mut self, cx: &mut Cx, event: &Event) {
        if let Some(_ne) = self.next_frame.is_event(event) {
            if self.state_handle_event(cx, event).is_animating() {
                // Update images position as main animation progresses
                if self.state.is_in_state(cx, id!(carrousel.display)) {
                    let direction = self.direction;
                    let offset = self.offset;

                    let (mut prev_image, mut current_image) = self.get_active_images_containers();

                    match direction {
                        CarrouselDirection::Forward => {
                            Self::set_horizontal_margin(&mut current_image, offset, cx);
                            Self::set_horizontal_margin(&mut prev_image, offset - 166.0, cx);
                        },
                        CarrouselDirection::Backward => {
                            Self::set_horizontal_margin(&mut current_image, -offset, cx);
                            Self::set_horizontal_margin(&mut prev_image, 166.0 - offset, cx);
                        }   
                    }
                }
            } else {
                if self.state.is_in_state(cx, id!(carrousel.restart)) {
                    // Fires the animation of the carrousel again
                    self.animate_state(cx, id!(carrousel.display));
                } else if self.state.is_in_state(cx, id!(carrousel.display)) {
                    // Begins the period of time where the carrousel is stopped
                    self.animate_state(cx, id!(carrousel.keep));
                    let (previous_image, _) = self.get_active_images_containers();
                    previous_image.set_visible(false);
                } else if self.state.is_in_state(cx, id!(carrousel.keep)) {
                    // Ends the period of time where the carrousel is stopped,
                    // prepares the next image
                    self.setup_next_animation(CarrouselDirection::Forward);

                    let (_, mut current_image) = self.get_active_images_containers();
                    Self::set_horizontal_margin(&mut current_image, 166.0, cx);
                    current_image.set_visible(true);
                    
                    self.animate_state(cx, id!(carrousel.restart));
                }
            }
            self.next_frame = cx.new_next_frame();
        }
    }

    fn get_active_images_containers(&mut self) -> (FrameRef, FrameRef) {
        let prev_index = self.previous_image_index as usize;
        let curr_index = self.current_image_index as usize;
        (
            self.image_containers[prev_index].clone(),
            self.image_containers[curr_index].clone()
        )
    }

    fn set_horizontal_margin(image_ref: &mut FrameRef, offset: f64, cx: &mut Cx) {
        image_ref.apply_over(cx, live!{walk: {margin: {left: (offset) }}});
    }

    fn adjust_indicators_width(&mut self, cx: &mut Cx) {
        for (i, indicator) in self.indicators_widgets.iter_mut().enumerate() {
            if i == self.current_image_index as usize {
                indicator.apply_over(cx, live!{walk: {width: 20.0}});
            } else {
                indicator.apply_over(cx, live!{walk: {width: 10.0}});
            }
        }
    }

    fn handle_mouse_event(&mut self, cx: &mut Cx, event: &Event) {
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
                            self.setup_next_animation(CarrouselDirection::Backward);
                            initial_offset = 166.0;
                        } else {
                            self.setup_next_animation(CarrouselDirection::Forward);
                            initial_offset = -166.0;
                        }

                        let (_, mut current_image) = self.get_active_images_containers();
                        Self::set_horizontal_margin(&mut current_image, initial_offset, cx);
                        current_image.set_visible(true);

                        self.animate_state(cx, id!(carrousel.restart));
                    }
                }
            },
            _ => {}
        }
    }

    fn setup_next_animation(&mut self, direction: CarrouselDirection) {
        self.direction = direction;
        self.previous_image_index = self.current_image_index;
        match direction {
            CarrouselDirection::Forward => {
                self.current_image_index = (self.current_image_index + 1).rem_euclid(self.image_containers.len() as i32);
            },
            CarrouselDirection::Backward => {
                self.current_image_index = (self.current_image_index - 1).rem_euclid(self.image_containers.len() as i32);
            }
        }
    }
}