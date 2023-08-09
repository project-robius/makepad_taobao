use makepad_widgets::*;

live_design! {
    import makepad_widgets::desktop_window::DesktopWindow
    import makepad_widgets::frame::*
    import makepad_widgets::radio_button::RadioButton

    import crate::shared::styles::*
    import crate::home::home_screen::HomeScreen

    HOME_ICON = dep("crate://self/resources/home.svg")
    DISCOVER_ICON = dep("crate://self/resources/discover.svg")
    INFO_ICON = dep("crate://self/resources/info.svg")
    CART_ICON = dep("crate://self/resources/cart.svg")
    MY_TAOBAO_ICON = dep("crate://self/resources/my_taobao.svg")


    H3_TEXT_REGULAR = {
        font_size: 9.0,
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    AppTab = <RadioButton> {
        walk: {height: Fill, width: Fit}
        layout: {align: {x: 0.0, y: 0.0}}
        draw_radio: {
            radio_type: Tab,
            color_active: (BACKGROUND_COLOR),
            color_inactive: (BACKGROUND_COLOR),
        }
        draw_label: {
            color_selected: #0b0,
            color_unselected: #000,
            color_unselected_hover: #111,
            text_style: <H3_TEXT_REGULAR> {}
        }
    }

    App = {{App}} {
        ui: <DesktopWindow> {
            window: {position: vec2(0, 0), inner_size: vec2(400, 800)},
            pass: {clear_color: (BACKGROUND_COLOR)}
            block_signal_event: true;

            <Frame> {
                walk: {width: Fill, height: Fill}
                layout: {padding: 0, align: {x: 0.0, y: 0.0}, spacing: 0., flow: Down}

                application_pages = <Frame> {
                    walk: {margin: 0.0}
                    layout: {padding: 0.0}

                    tab1_frame = <HomeScreen> {visible: true}
                    tab2_frame = <HomeScreen> {visible: false}
                    tab3_frame = <HomeScreen> {visible: false}
                    tab4_frame = <HomeScreen> {visible: false}
                    tab5_frame = <HomeScreen> {visible: false}
                }

                mobile_menu = <Box> {
                    walk: {width: Fill, height: 80}
                    layout: {flow: Right, spacing: 6.0, padding: 10}
                    draw_bg: {
                        instance radius: 0.0,
                        instance border_width: 1.0,
                        instance border_color: #aaa,
                        color: (BACKGROUND_COLOR)
                    }

                    mobile_modes = <Frame> {
                        tab1 = <AppTab> {
                            label: "Home"
                            draw_icon: {
                                svg_file: (HOME_ICON),
                                fn get_color(self) -> vec4 {
                                    return mix(
                                        #000,
                                        #0b0,
                                        self.selected
                                    )
                                }
                            }
                            walk: {width: Fill}
                            icon_walk: {width: 20, height: 20}
                            layout: {flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}}
                        }
                        tab2 = <AppTab> {
                            state: {selected = {default: on}}
                            label: "Discover",
                            draw_icon: {
                                svg_file: (DISCOVER_ICON),
                                fn get_color(self) -> vec4 {
                                    return mix(
                                        #000,
                                        #0b0,
                                        self.selected
                                    )
                                }
                            }
                            walk: {width: Fill}
                            icon_walk: {width: 20, height: 20}
                            layout: {flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}}
                        }
                        tab3 = <AppTab> {
                            label: "Info",
                            draw_icon: {
                                svg_file: (INFO_ICON),
                                fn get_color(self) -> vec4 {
                                    return mix(
                                        #000,
                                        #0b0,
                                        self.selected
                                    )
                                }
                            }
                            walk: {width: Fill}
                            icon_walk: {width: 20, height: 20}
                            layout: {flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}}
                        }
                        tab4 = <AppTab> {
                            label: "Cart",
                            draw_icon: {
                                svg_file: (CART_ICON),
                                fn get_color(self) -> vec4 {
                                    return mix(
                                        #000,
                                        #0b0,
                                        self.selected
                                    )
                                }
                            }
                            walk: {width: Fill}
                            icon_walk: {width: 20, height: 20}
                            layout: {flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}}
                        }
                        tab5 = <AppTab> {
                            label: "My Taobao",
                            draw_icon: {
                                svg_file: (MY_TAOBAO_ICON),
                                fn get_color(self) -> vec4 {
                                    return mix(
                                        #000,
                                        #0b0,
                                        self.selected
                                    )
                                }
                            }
                            walk: {width: Fill}
                            icon_walk: {width: 20, height: 20}
                            layout: {flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}}
                        }
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl App {}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        makepad_widgets::live_design(cx);

        // shared
        crate::shared::styles::live_design(cx);
        crate::shared::helpers::live_design(cx);

        // home
        crate::home::home_screen::live_design(cx);
        crate::home::search_terms::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }

        let ui = self.ui.clone();
        let actions = ui.handle_widget_event(cx, event);

        ui.get_radio_button_set(ids!(
            mobile_modes.tab1,
            mobile_modes.tab2,
            mobile_modes.tab3,
            mobile_modes.tab4,
            mobile_modes.tab5,
        ))
        .selected_to_visible(
            cx,
            &ui,
            &actions,
            ids!(
                application_pages.tab1_frame,
                application_pages.tab2_frame,
                application_pages.tab3_frame,
                application_pages.tab4_frame,
                application_pages.tab5_frame,
            ),
        );
    }
}
