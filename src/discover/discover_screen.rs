use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;

    Toggle = <View> {
        draw_bg: {color: #fff}
        width: Fit,
        height: Fit
        flow: Down,

        checkbox = <CheckBox> {
            padding: {top: 0, right: 1, bottom: 0, left: 34}
            text: " "
            animator: {
                selected = {
                    default: off
                    off = {
                        from: {all: Forward {duration: 0.1}}
                        apply: {draw_check: {selected: 0.0}}
                    }
                    on = {
                        cursor: Arrow,
                        from: {all: Forward {duration: 0.1}}
                        apply: {draw_check: {selected: 1.0}}
                    }
                }
            }
            draw_check: {
                size: 14.;
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                    let sz = self.size;
                    let left = sz + 0.5;
                    let c = vec2(left + sz, self.rect_size.y * 0.5);
                    sdf.box(left, c.y - sz, sz * 3.0, sz * 2.0, 0.5 * sz);

                    sdf.fill(#c7)
                    sdf.blend(self.selected)
                    sdf.fill(#3dd041)

                    let isz = sz * 0.8;
                    sdf.circle(left + sz + self.selected * sz, c.y - 0.5, isz);
                    sdf.circle(left + sz + self.selected * sz, c.y - 0.5, isz);
                    sdf.blend(self.selected)
                    sdf.fill(#fff);
                    return sdf.result
                }
            }
            draw_text: {
                text_style: <TITLE_TEXT> {},
                color: #000
            }
        }
    }

    OptionToggle = <View> {
        width: Fill
        height: Fit
        flow: Right
        align: {y: 0.5}
        <Label> {
            width: Fit, height: Fit
            text: "国浓缩缩乳乳"
            draw_text: {
                color: #000
                text_style: <REGULAR_TEXT> {font_size: 10.},
            }
        }
        <FillerX> {}
        <Toggle> {}
    }

    DiscoverScreen = <View> {
        width: Fill
        height: Fill
        flow: Down
        padding: {top: 50}
        align: {x: 0.5}
        spacing: 20

        show_bg: true
        draw_bg: {
            color: (BACKGROUND_COLOR)
        }

        <Label> {
            width: Fit, height: Fit
            text: "国浓缩乳"
            draw_text: {
                color: #000
                text_style: <TITLE_TEXT> {font_size: 12.},
            }
        }

        <Label> {
            width: Fit, height: Fit
            text: "国浓缩乳: 2.0.0.1"
            draw_text: {
                color: #000
                text_style: <REGULAR_TEXT> {font_size: 10.},
            }
        }

        <View> {
            width: Fill
            height: Fill
            flow: Down
            padding: {top: 5}

            show_bg: true
            draw_bg: {
                color: #fff
            }
            <OptionToggle> {}
            <OptionToggle> {}
            <Label> {
                padding: {top: 10, left: 10}
                width: Fit, height: Fit
                text: "国浓缩缩乳乳"
                draw_text: {
                    color: #fa0322
                    text_style: <REGULAR_TEXT> {font_size: 10.},
                }
            }
            <FillerY> {}
        }
    }
}
