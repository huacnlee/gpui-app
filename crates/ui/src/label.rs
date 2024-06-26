use gpui::{div, Div, IntoElement, ParentElement, RenderOnce, SharedString, Styled, WindowContext};

use crate::theme::ActiveTheme;

#[derive(IntoElement)]
pub struct Label {
    base: Div,
    label: SharedString,
    multiple_lines: bool,
}

impl Label {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            label: label.into(),
            multiple_lines: true,
        }
    }

    pub fn multiple_lines(mut self) -> Self {
        self.multiple_lines = true;
        self
    }
}

impl Styled for Label {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Label {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let text = if !self.multiple_lines {
            SharedString::from(self.label.replace('\n', "␤"))
        } else {
            self.label
        };

        div()
            .text_color(cx.theme().foreground)
            .child(self.base.child(text))
    }
}
