use gpui::{
    div, px, CursorStyle, InteractiveElement, ParentElement, Render, StatefulInteractiveElement,
    Styled, View, VisualContext as _, WindowContext,
};

use ui::{
    button::{Button, ButtonStyle},
    checkbox::Checkbox,
    h_flex,
    label::Label,
    tooltip::Tooltip,
    v_flex, Selection,
};

pub struct TooltipStory;

impl TooltipStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self::new(cx))
    }

    fn new(_: &mut WindowContext) -> Self {
        Self {}
    }
}

impl Render for TooltipStory {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        v_flex()
            .w(px(360.))
            .gap_5()
            .child(
                div()
                    .cursor(CursorStyle::PointingHand)
                    .child(
                        Button::new("button")
                            .label("Hover me")
                            .style(ButtonStyle::Primary),
                    )
                    .id("tooltip-1")
                    .tooltip(|cx| Tooltip::text("This is a Button", cx)),
            )
            .child(
                div()
                    .cursor(CursorStyle::PointingHand)
                    .child(
                        Button::new("button-meta")
                            .label("With meta, Hover me")
                            .style(ButtonStyle::Primary),
                    )
                    .id("tooltip-2")
                    .tooltip(|cx| Tooltip::with_meta("This is a Button", "Click if you want", cx)),
            )
            .child(
                h_flex()
                    .justify_center()
                    .cursor(CursorStyle::PointingHand)
                    .child(Label::new("Hover me"))
                    .id("tooltip-3")
                    .tooltip(|cx| Tooltip::text("This is a Label", cx)),
            )
            .child(
                div()
                    .cursor(CursorStyle::PointingHand)
                    .child(
                        Checkbox::new("check")
                            .label("Remember me")
                            .checked(Selection::Selected),
                    )
                    .id("tooltip-4")
                    .tooltip(|cx| Tooltip::text("Checked!", cx)),
            )
    }
}
