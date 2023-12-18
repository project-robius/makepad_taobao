use crate::catalog_item::catalog_item_screen::CatalogItemWidgetRefExt;
use crate::home::home_content::CatalogItemListAction;
use crate::shared::stack_navigation::StackNavigationWidgetRefExt;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*
    import crate::shared::stack_navigation::*
    import crate::home::home_screen::HomeScreen
    import crate::catalog_item::catalog_item_screen::*
    import crate::settings::settings_screen::SettingsScreen

    HOME_ICON = dep("crate://self/resources/home.svg")
    SETTINGS_ICON = dep("crate://self/resources/settings.svg")
    INFO_ICON = dep("crate://self/resources/info.svg")
    CART_ICON = dep("crate://self/resources/cart.svg")
    MY_TAOBAO_ICON = dep("crate://self/resources/my_taobao.svg")

    AppTab = <RadioButton> {
        height: Fill
        width: Fit
        align: {x: 0.0, y: 0.0}

        draw_radio: {
            radio_type: Tab,
            color_active: (BACKGROUND_COLOR),
            color_inactive: (BACKGROUND_COLOR),
        }
        draw_text: {
            color_selected: (ORANGE_COLOR),
            color_unselected: #000,
            color_unselected_hover: #111,
            text_style: <APP_NAVIGATION_FONT> {}
        }
    }

    App = {{App}} {
        ui: <Window> {
            window: {position: vec2(0, 0), inner_size: vec2(360, 800)},
            pass: {clear_color: (BACKGROUND_COLOR)}

            body = {
                navigation = <StackNavigation> {
                    root_view = {
                        width: Fill
                        height: Fill
                        padding: 0
                        flow: Down
                        align: {x: 0.0, y: 0.0}
                        spacing: 0

                        application_pages = <View> {
                            margin: 0
                            padding: 0

                            home_screen = <HomeScreen> {visible: true}
                            settings_screen = <SettingsScreen> {visible: false}
                            tab3_frame = <View> {visible: false}
                            tab4_frame = <View> {visible: false}
                            tab5_frame = <View> {visible: false}
                        }

                        mobile_menu = <RoundedView> {
                            width: Fill
                            height: 80
                            padding: 10
                            flow: Right
                            spacing: 6.0
                            draw_bg: {
                                instance radius: 0.0,
                                instance border_width: 1.0,
                                instance border_color: #aaa,
                                color: (BACKGROUND_COLOR)
                            }

                            mobile_modes = <View> {
                                tab1 = <AppTab> {
                                    animator: {selected = {default: on}}
                                    label: "首页"
                                    draw_icon: {
                                        svg_file: (HOME_ICON),
                                        fn get_color(self) -> vec4 {
                                            return mix(
                                                #000,
                                                ORANGE_COLOR,
                                                self.selected
                                            )
                                        }
                                    }
                                    width: Fill
                                    flow: Down
                                    spacing: 5.0
                                    align: {x: 0.5, y: 0.5}

                                    icon_walk: {width: 20, height: 20}
                                }
                                tab2 = <AppTab> {
                                    label: "逛逛",
                                    draw_icon: {
                                        svg_file: (SETTINGS_ICON),
                                        fn get_color(self) -> vec4 {
                                            return mix(
                                                #000,
                                                ORANGE_COLOR,
                                                self.selected
                                            )
                                        }
                                    }
                                    width: Fill
                                    flow: Down
                                    spacing: 5.0
                                    align: {x: 0.5, y: 0.5}

                                    icon_walk: {width: 20, height: 20}
                                }
                                tab3 = <AppTab> {
                                    label: "消息",
                                    draw_icon: {
                                        svg_file: (INFO_ICON),
                                        fn get_color(self) -> vec4 {
                                            return mix(
                                                #000,
                                                ORANGE_COLOR,
                                                self.selected
                                            )
                                        }
                                    }
                                    width: Fill
                                    flow: Down
                                    spacing: 5.0
                                    align: {x: 0.5, y: 0.5}

                                    icon_walk: {width: 20, height: 20}
                                }
                                tab4 = <AppTab> {
                                    label: "购物车",
                                    draw_icon: {
                                        svg_file: (CART_ICON),
                                        fn get_color(self) -> vec4 {
                                            return mix(
                                                #000,
                                                ORANGE_COLOR,
                                                self.selected
                                            )
                                        }
                                    }
                                    width: Fill
                                    flow: Down
                                    spacing: 5.0
                                    align: {x: 0.5, y: 0.5}

                                    icon_walk: {width: 20, height: 20}
                                }
                                tab5 = <AppTab> {
                                    label: "我的淘宝",
                                    draw_icon: {
                                        svg_file: (MY_TAOBAO_ICON),
                                        fn get_color(self) -> vec4 {
                                            return mix(
                                                #000,
                                                ORANGE_COLOR,
                                                self.selected
                                            )
                                        }
                                    }
                                    width: Fill
                                    flow: Down
                                    spacing: 5.0
                                    align: {x: 0.5, y: 0.5}

                                    icon_walk: {width: 20, height: 20}
                                }
                            }
                        }
                    }

                    catalog_item_stack_view = <StackNavigationView> {
                        catalog_item_screen = <CatalogItemScreen> {}
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl App {}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);

        // shared
        crate::shared::styles::live_design(cx);
        crate::shared::helpers::live_design(cx);
        crate::shared::clickable_view::live_design(cx);
        crate::shared::stack_navigation::live_design(cx);

        // home
        crate::home::helpers::live_design(cx);
        crate::home::home_screen::live_design(cx);
        crate::home::home_content::live_design(cx);
        crate::home::search_terms::live_design(cx);
        crate::home::carrousel::live_design(cx);

        // others
        crate::settings::settings_screen::live_design(cx);
        crate::catalog_item::catalog_item_screen::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let ui = self.ui.clone();

        ui.radio_button_set(ids!(
            mobile_modes.tab1,
            mobile_modes.tab2,
            mobile_modes.tab3,
            mobile_modes.tab4,
            mobile_modes.tab5,
        ))
        .selected_to_visible(
            cx,
            &ui,
            actions,
            ids!(
                application_pages.home_screen,
                application_pages.settings_screen,
                application_pages.tab3_frame,
                application_pages.tab4_frame,
                application_pages.tab5_frame,
            ),
        );

        for action in actions {
            if let CatalogItemListAction::Click(id) = action.as_widget_action().cast() {
                let mut stack_navigation = ui.stack_navigation(id!(navigation));

                let catalog_item_ref = stack_navigation
                    .view(id!(catalog_item_stack_view.catalog_item_screen))
                    .catalog_item(id!(catalog_item));
                catalog_item_ref.set_catalog_item_id(id);

                stack_navigation
                    .show_stack_view_by_id(LiveId::from_str("catalog_item_stack_view"), cx);
            }
        }
    }
}
