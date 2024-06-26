use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_draw::shader::draw_icon::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::helpers::*;

    CURVED_ARROW_IMG = dep("crate://self/resources/curved_arrow.png")
    MEATBALLS_MENU_IMG = dep("crate://self/resources/meatballs_menu.png")
    BACK_ICON = dep("crate://self/resources/back.svg")
    SEARCH_ICON = dep("crate://self/resources/search.svg")
    CART_ICON = dep("crate://self/resources/cart.svg")

    CatalogSearchBar = <RoundedView> {
        width: Fill
        height: Fit
        margin: 10.0
        padding: {left: 5., right: 5.}
        flow: Right
        align: {x: 0.0, y: 0.5}

        draw_bg: {
            color: #fff,
            radius: 10.
        }

        <Button> {
            width: Fit,
            height: Fit
            icon_walk: {width: 16, height: 16}
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    return sdf.result
                }
            }
            draw_icon: {
                svg_file: (SEARCH_ICON)
                fn get_color(self) -> vec4 {
                    return #8b
                }
            }
        }

        <Label> {
            width: Fit, height: Fit
            text: "搜索"
            draw_text: {
                color: #8b
                text_style:  {font_size: 10.0},
            }
        }

        <FillerX> {}

    }

    CatalogHeader = <View> {
        width: Fill
        height: 110
        margin: 0
        flow: Overlay
        align: {x: 0.5, y: 0.0}
        spacing: 0.0
        padding: { top: 16, left: 0 }

        show_bg: true
        draw_bg: {
            color: #f2
        }

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 8.

            search_container = <View> {
                width: Fill
                height: Fit
                flow: Right
                align: { y: 0.5}

                <CatalogSearchBar> {}

                curved_arrow = <Image> {
                    width: 32, height: 32, source: (CURVED_ARROW_IMG)
                }

                <Button> {
                    width: Fit, height: Fit
                    icon_walk: {width: 16, height: 16}
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            return sdf.result
                        }
                    }
                    draw_icon: {
                        svg_file: (CART_ICON)
                        fn get_color(self) -> vec4 {
                            return #0
                        }
                    }
                }

                meatballs_menu = <Image> {
                    width: 32, height: 32, source: (MEATBALLS_MENU_IMG)
                }
            }

            navigation_container = <View> {
                width: Fill
                height: Fit
                flow: Right

                // Compensate the shift made for the parent view to uncover the Makepad StackNavigationView back button
                margin: { left: -30 }

                <FillerX> {}
                <Label> {
                    width: Fit, height: Fit
                    text: "搜索"
                    draw_text: {
                        color: #0
                        text_style:  {font_size: 14.0},
                    }
                }
                <FillerX> {}

                <Label> {
                    width: Fit, height: Fit
                    text: "搜索"
                    draw_text: {
                        color: #0
                        text_style:  {font_size: 14.0},
                    }
                }
                <FillerX> {}

                <Label> {
                    width: Fit, height: Fit
                    text: "搜索"
                    draw_text: {
                        color: #0
                        text_style:  {font_size: 14.0},
                    }
                }
                <FillerX> {}

                <Label> {
                    width: Fit, height: Fit
                    text: "搜索"
                    draw_text: {
                        color: #0
                        text_style:  {font_size: 14.0},
                    }
                }
                <FillerX> {}
            }
        }
    }
}