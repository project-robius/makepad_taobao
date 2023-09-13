use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

const IMAGE_WIDTH: f64 = 166.0;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    CARROUSEL_1_IMG = dep("crate://self/resources/catalog/carrousel_1.png")
    CARROUSEL_2_IMG = dep("crate://self/resources/catalog/carrousel_2.png")
    CARROUSEL_3_IMG = dep("crate://self/resources/catalog/carrousel_3.png")

    IMAGE_WIDTH: 166.0

    Carrousel = {{Carrousel}} {
        width: (IMAGE_WIDTH)
        height: Fit
        flow: Overlay,
        align: {x: 0.0, y: 0.0}

        image1 = <View> {
            width: Fit, height: Fit
            <Image> {
                width: (IMAGE_WIDTH)
                height: 226
                source: (CARROUSEL_1_IMG)
            }
        }

        image2 = <View> {
            width: Fit, height: Fit
            <Image> {
                width: (IMAGE_WIDTH)
                height: 226
                source: (CARROUSEL_2_IMG)
            }
        }

        image3 = <View> {
            width: Fit, height: Fit
            <Image> {
                width: (IMAGE_WIDTH)
                height: 226
                source: (CARROUSEL_3_IMG)
            }
        }

        <View> {
            width: Fill
            height: Fit
            margin: {top: 205.}
            flow: Right,
            align: {x: 0.5, y: 0.0}
            spacing: 5.0

            indicator1 = <RoundedView> {
                width: 10.0
                height: 10.0
                draw_bg: {
                    color: #f60,
                    radius: 2.5
                }
            }

            indicator2 = <RoundedView> {
                width: 20.0
                height: 10.0
                draw_bg: {
                    color: #f60,
                    radius: 2.5
                }
            }

            indicator3 = <RoundedView> {
                width: 10.0
                height: 10.0
                draw_bg: {
                    color: #f60,
                    radius: 2.5
                }
            }
        }

        offset: 0

        animator: {
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
    view: View,

    #[live]
    offset: f64,

    #[animator]
    animator: Animator,

    #[rust]
    next_view: NextFrame,

    #[rust]
    last_abs: f64,

    #[rust]
    image_containers: Vec<ViewRef>,

    #[rust]
    indicators_widgets: Vec<ViewRef>,

    #[rust(0)]
    current_image_index: i32,

    #[rust(0)]
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
            self.view(id!(image1)),
            self.view(id!(image2)),
            self.view(id!(image3)),
        ];

        self.indicators_widgets = vec![
            self.view(id!(indicator1)),
            self.view(id!(indicator2)),
            self.view(id!(indicator3)),
        ];

        self.reset_images_visibility();

        self.next_view = cx.new_next_frame();
        self.animator_play(cx, id!(carrousel.keep));
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

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.view.walk(cx)
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.view.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.next_view = cx.new_next_frame();
        self.view.draw_walk_widget(cx, walk)
    }
}

impl Carrousel {
    fn control_animation(&mut self, cx: &mut Cx, event: &Event) {
        if let Some(_ne) = self.next_view.is_event(event) {
            if self.animator_handle_event(cx, event).is_animating() {
                self.update_image_positions(cx);
            } else {
                self.fire_next_animation(cx);
            }
            self.next_view = cx.new_next_frame();
        }

        // Fixes a bug where the carrousel would not animate returning from stack navigation
        if let Event::NextFrame(_) = event {
            self.next_view = cx.new_next_frame();
        }
    }

    fn get_active_images_containers(&mut self) -> (ViewRef, ViewRef) {
        let prev_index = self.previous_image_index as usize;
        let curr_index = self.current_image_index as usize;
        (
            self.image_containers[prev_index].clone(),
            self.image_containers[curr_index].clone(),
        )
    }

    fn update_image_positions(&mut self, cx: &mut Cx) {
        if self.animator.animator_in_state(cx, id!(carrousel.display)) {
            let direction = self.direction;
            let offset = self.offset;

            let (mut prev_image, mut current_image) = self.get_active_images_containers();

            match direction {
                CarrouselDirection::Forward => {
                    Self::set_horizontal_margin(&mut current_image, offset, cx);
                    Self::set_horizontal_margin(&mut prev_image, offset - IMAGE_WIDTH, cx);
                }
                CarrouselDirection::Backward => {
                    Self::set_horizontal_margin(&mut current_image, -offset, cx);
                    Self::set_horizontal_margin(&mut prev_image, IMAGE_WIDTH - offset, cx);
                }
            }
        }
    }

    fn fire_next_animation(&mut self, cx: &mut Cx) {
        if self.animator.animator_in_state(cx, id!(carrousel.restart)) {
            // Fires the animation of the carrousel again
            self.animator_play(cx, id!(carrousel.display));
        } else if self.animator.animator_in_state(cx, id!(carrousel.display)) {
            // Begins the period of time where the carrousel is stopped
            self.animator_play(cx, id!(carrousel.keep));
            let (previous_image, _) = self.get_active_images_containers();
            previous_image.set_visible(false);
        } else if self.animator.animator_in_state(cx, id!(carrousel.keep)) {
            // Ends the period of time where the carrousel is stopped,
            // prepares the next image
            self.setup_next_animation(CarrouselDirection::Forward);

            let (_, mut current_image) = self.get_active_images_containers();
            Self::set_horizontal_margin(&mut current_image, IMAGE_WIDTH, cx);
            current_image.set_visible(true);

            self.animator_play(cx, id!(carrousel.restart));
        }
    }

    fn set_horizontal_margin(image_ref: &mut ViewRef, offset: f64, cx: &mut Cx) {
        image_ref.apply_over(cx, live! {margin: {left: (offset) }});
    }

    fn adjust_indicators_width(&mut self, cx: &mut Cx) {
        for (i, indicator) in self.indicators_widgets.iter_mut().enumerate() {
            if i == self.current_image_index as usize {
                indicator.apply_over(cx, live! {width: 20.0});
            } else {
                indicator.apply_over(cx, live! {width: 10.0});
            }
        }
    }

    fn handle_mouse_event(&mut self, cx: &mut Cx, event: &Event) {
        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(fe) => {
                self.last_abs = fe.abs.x;
            }
            Hit::FingerUp(fe) => {
                if fe.is_over && (fe.abs.x - self.last_abs).abs() > 10.0 {
                    self.previous_image_index = self.current_image_index;

                    let initial_offset = if fe.abs.x > self.last_abs {
                        self.setup_next_animation(CarrouselDirection::Backward);
                        IMAGE_WIDTH
                    } else {
                        self.setup_next_animation(CarrouselDirection::Forward);
                        -IMAGE_WIDTH
                    };

                    let (_, mut current_image) = self.get_active_images_containers();
                    Self::set_horizontal_margin(&mut current_image, initial_offset, cx);
                    self.reset_images_visibility();

                    self.animator_play(cx, id!(carrousel.restart));
                }
            }
            _ => {}
        }
    }

    fn setup_next_animation(&mut self, direction: CarrouselDirection) {
        self.direction = direction;
        self.previous_image_index = self.current_image_index;
        match direction {
            CarrouselDirection::Forward => {
                self.current_image_index =
                    (self.current_image_index + 1).rem_euclid(self.image_containers.len() as i32);
            }
            CarrouselDirection::Backward => {
                self.current_image_index =
                    (self.current_image_index - 1).rem_euclid(self.image_containers.len() as i32);
            }
        }
    }

    fn reset_images_visibility(&mut self) {
        for (i, image) in self.image_containers.iter().enumerate() {
            if i == self.current_image_index as usize || i == self.previous_image_index as usize {
                image.set_visible(true);
            } else {
                image.set_visible(false);
            }
        }
    }
}
