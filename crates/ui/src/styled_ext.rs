use crate::theme::ActiveTheme;
use gpui::{hsla, point, px, BoxShadow, Styled, WindowContext};
use smallvec::{smallvec, SmallVec};

pub enum ElevationIndex {
    Surface,
    PopoverSurface,
    ModalSurface,
}

impl ElevationIndex {
    pub fn shadow(self) -> SmallVec<[BoxShadow; 2]> {
        match self {
            ElevationIndex::Surface => smallvec![],

            ElevationIndex::PopoverSurface => smallvec![BoxShadow {
                color: hsla(0., 0., 0., 0.12),
                offset: point(px(0.), px(2.)),
                blur_radius: px(3.),
                spread_radius: px(0.),
            }],

            ElevationIndex::ModalSurface => smallvec![
                BoxShadow {
                    color: hsla(0., 0., 0., 0.1),
                    offset: point(px(0.), px(4.)),
                    blur_radius: px(6.),
                    spread_radius: px(-1.),
                },
                BoxShadow {
                    color: hsla(0., 0., 0., 0.1),
                    offset: point(px(0.), px(2.)),
                    blur_radius: px(4.),
                    spread_radius: px(-2.),
                }
            ],
        }
    }
}

fn elevated<E: Styled>(this: E, cx: &WindowContext, index: ElevationIndex) -> E {
    this.bg(cx.theme().popover)
        .rounded(px(8.))
        .border_1()
        .border_color(cx.theme().border)
        .shadow(index.shadow())
}

/// Extends [`gpui::Styled`] with specific styling methods.
pub trait StyledExt: Styled + Sized {
    /// Horizontally stacks elements.
    ///
    /// Sets `flex()`, `flex_row()`, `items_center()`
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// Vertically stacks elements.
    ///
    /// Sets `flex()`, `flex_col()`
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }

    /// Located above the app background
    fn elevation_1(self, cx: &WindowContext) -> Self {
        elevated(self, cx, ElevationIndex::Surface)
    }

    /// Appear above most UI elements
    fn elevation_2(self, cx: &WindowContext) -> Self {
        elevated(self, cx, ElevationIndex::PopoverSurface)
    }

    // Above all other UI elements and are located above the wash layer
    fn elevation_3(self, cx: &WindowContext) -> Self {
        elevated(self, cx, ElevationIndex::ModalSurface)
    }
}

impl<E: Styled> StyledExt for E {}
