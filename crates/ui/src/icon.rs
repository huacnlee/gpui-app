use gpui::{svg, Element, IntoElement, RenderOnce, SharedString, Styled as _, Svg, WindowContext};

#[derive(IntoElement)]
pub enum IconName {
    Check,
    Minus,
    Dash,
    Maximize,
    Minimize,
    Close,
    ChevronDown,
}

impl IconName {
    pub fn path(self) -> SharedString {
        match self {
            IconName::Check => "icons/check.svg",
            IconName::Minus => "icons/minus.svg",
            IconName::Dash => "icons/dash.svg",
            IconName::Maximize => "icons/maximize.svg",
            IconName::Minimize => "icons/minimize.svg",
            IconName::Close => "icons/close.svg",
            IconName::ChevronDown => "icons/chevron-down.svg",
        }
        .into()
    }
}

impl RenderOnce for IconName {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        Icon::new(self)
    }
}

#[derive(IntoElement)]
pub struct Icon {
    base: Svg,
    path: SharedString,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            base: svg(),
            path: name.path(),
        }
    }

    /// Set the icon path of the Assets bundle
    ///
    /// For example: `icons/foo.svg`
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = path.into();
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        self.base.flex_none().path(self.path)
    }
}
