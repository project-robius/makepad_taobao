use makepad_widgets::*;

live_design! {
    import makepad_widgets::view::*;
    import makepad_widgets::text::*;
    import makepad_widgets::image::*;
    import makepad_widgets::text_input::TextInput;
    import makepad_widgets::list_view::ListView;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;
    import crate::home::helpers::*;
    import crate::home::search_terms::SearchTerms;
    import crate::home::home_content::HomeContent;

    WORLDWIDE_ICON = dep("crate://self/resources/worldwide.png")
    MEMBERSHIP_CODE_ICON = dep("crate://self/resources/membership_code.png")
    SCAN_ICON = dep("crate://self/resources/scan.png")
    CAMERA_ICON = dep("crate://self/resources/camera.png")

    TopBar = <View> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Right, align: {x: 0.0, y: 0.5}, spacing: 10.0, padding: {left: 10., right: 10.}}

        <IconWithText> {
            image = { source: (WORLDWIDE_ICON) }
            caption = { text: "全球" }
        }

        <FillerX> {}

        <Label> {
            walk: {width: Fit, height: Fit}
            text: "推荐"
            draw_label: {
                color: #000
                text_style: <REGULAR_TEXT> {},
            }
        }
        <Label> {
            walk: {width: Fit, height: Fit}
            text: "特价"
            draw_label: {
                color: #000
                text_style: <REGULAR_TEXT> {},
            }
        }

        <FillerX> {}

        <IconWithText> {
            image = { source: (MEMBERSHIP_CODE_ICON) }
            caption = { text: "会员码" }
        }
    }

    SearchBar = <Box> {
        walk: {width: Fill, height: Fit, margin: 10.0}
        layout: {flow: Right, align: {x: 0.0, y: 0.5}, padding: {left: 10., right: 10.} spacing: 6.0}
        draw_bg: {
            color: #fff,
            radius: 10.
        }

        <Image> {
            walk: {width: 24, height: 24}
            source: (SCAN_ICON)
        }

        <LineH> {
            walk: {width: 3.0, height: Fill}
        }

        input = <SearchTerms> {}

        <Image> {
            walk: {width: 24, height: 24}
            source: (CAMERA_ICON)
        }

        <Box> {
            walk: {width: Fit, height: Fit}
            layout: {
                flow: Right, align: {x: 0.0, y: 0.5}, spacing: 10.0,
                padding: {top: 9., left: 6., right: 6., bottom: 9.}
            }
            draw_bg: {
                color: (ORANGE_COLOR),
                radius: 6.
            }

            <Label> {
                walk: {width: Fit, height: Fit}
                text: "搜索"
                draw_label: {
                    color: #fff
                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                }
            }
        }
    }

    HomeScreen = <View> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}
        draw_bg: {
            color: (BACKGROUND_COLOR)
        }

        os_header_placeholder = <Box> {
            walk: {width: Fill, height: 50, margin: 0}
            layout: {flow: Right, spacing: 6.0, padding: 0}
        }
        <TopBar> {}
        <SearchBar> {}
        <HomeContent> {}
    }
}
