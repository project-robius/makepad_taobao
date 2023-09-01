use makepad_widgets::*;

live_design! {
    import makepad_widgets::view::*;
    import makepad_widgets::label::*;
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
        width: Fill
        height: Fit
        flow: Right
        padding: {left: 10., right: 10.}
        align: {x: 0.0, y: 0.5}
        spacing: 10.0

        <IconWithText> {
            image = { source: (WORLDWIDE_ICON) }
            caption = { text: "全球" }
        }

        <FillerX> {}

        <Label> {
            width: Fit
            height: Fit

            text: "推荐"
            draw_text: {
                color: #000
                text_style: <REGULAR_TEXT> {},
            }
        }
        <Label> {
            width: Fit
            height: Fit

            text: "特价"
            draw_text: {
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

    SearchBar = <RoundedView> {
        width: Fill
        height: Fit
        margin: 10.0
        flow: Right
        align: {x: 0.0, y: 0.5}
        padding: {left: 10., right: 10.}
        spacing: 6.0

        draw_bg: {
            color: #fff,
            radius: 10.
        }

        <Image> { width: 24, height: 24, source: (SCAN_ICON) }

        <LineH> { width: 3.0, height: Fill }

        input = <SearchTerms> {}

        <Image> { width: 24, height: 24, source: (CAMERA_ICON) }

        <RoundedView> {
            width: Fit,
            height: Fit
            flow: Right,
            align: {x: 0.0, y: 0.5},
            spacing: 10.0
            padding: {top: 9., left: 6., right: 6., bottom: 9.}

            draw_bg: {
                color: (ORANGE_COLOR),
                radius: 6.
            }

            <Label> {
                width: Fit,
                height: Fit

                text: "搜索"
                draw_text: {
                    color: #fff
                    text_style: <REGULAR_TEXT> {font_size: 10.0},
                }
            }
        }
    }

    HomeScreen = <View> {
        width: Fill
        height: Fill
        flow: Down

        draw_bg: {
            color: (BACKGROUND_COLOR)
        }

        os_header_placeholder = <RoundedView> {
            width: Fill,
            height: 50,
            margin: 0
            padding: 0
            flow: Right,
            spacing: 6.0,
        }

        <TopBar> {}
        <SearchBar> {}
        <HomeContent> {}
    }
}
