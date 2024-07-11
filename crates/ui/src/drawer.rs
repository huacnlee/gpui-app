use std::{cell::RefCell, rc::Rc};

use crate::theme::{ActiveTheme, Colorize};
use gpui::{
    anchored, deferred, div, point, prelude::FluentBuilder as _, px, AnchorCorner,
    AnchoredPositionMode, AnyElement, AppContext, Bounds, DismissEvent, DispatchPhase, Element,
    ElementId, GlobalElementId, Hitbox, InteractiveElement as _, IntoElement, LayoutId, Length,
    ManagedView, MouseDownEvent, ParentElement as _, Pixels, Style, Styled as _, View,
    VisualContext as _, WindowContext,
};

pub fn init(cx: &mut AppContext) {}

pub struct Drawer<M: ManagedView> {
    id: ElementId,
    trigger: Option<Box<dyn FnOnce(&WindowContext) -> AnyElement + 'static>>,
    content: Option<Rc<dyn Fn(&mut WindowContext) -> View<M> + 'static>>,
    size: Pixels,
    mask: bool,
}

impl<M> Drawer<M>
where
    M: ManagedView,
{
    /// Create a new Popover with `view` mode.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            size: px(300.),
            mask: true,
        }
    }

    /// Set the size of the drawer.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into();
        self
    }

    /// Set whether the drawer should have a mask behind it.
    pub fn mask(mut self, mask: bool) -> Self {
        self.mask = mask;
        self
    }

    pub fn trigger<T>(mut self, trigger: T) -> Self
    where
        T: IntoElement + 'static,
    {
        self.trigger = Some(Box::new(|_| trigger.into_any_element()));
        self
    }

    /// Set the content of the popover.
    ///
    /// The `content` is a closure that returns an `AnyElement`.
    pub fn content<C>(mut self, content: C) -> Self
    where
        C: Fn(&mut WindowContext) -> View<M> + 'static,
    {
        self.content = Some(Rc::new(content));
        self
    }

    fn render_trigger(&mut self, cx: &mut WindowContext) -> impl IntoElement {
        let base = div().id("drawer-trigger");

        if self.trigger.is_none() {
            return base;
        }

        let trigger = self.trigger.take().unwrap();

        base.child((trigger)(cx)).into_element()
    }

    fn with_element_state<R>(
        &mut self,
        id: &GlobalElementId,
        cx: &mut WindowContext,
        f: impl FnOnce(&mut Self, &mut DrawerElementState<M>, &mut WindowContext) -> R,
    ) -> R {
        cx.with_optional_element_state::<DrawerElementState<M>, _>(Some(id), |element_state, cx| {
            let mut element_state = element_state.unwrap().unwrap_or_default();
            let result = f(self, &mut element_state, cx);
            (result, Some(element_state))
        })
    }
}

pub struct DrawerElementState<M> {
    trigger_layout_id: Option<LayoutId>,
    trigger_element: Option<AnyElement>,
    trigger_bounds: Option<Bounds<Pixels>>,
    drawer_layout_id: Option<LayoutId>,
    drawer_element: Option<AnyElement>,
    content_view: Rc<RefCell<Option<View<M>>>>,
}

impl<M> Default for DrawerElementState<M> {
    fn default() -> Self {
        Self {
            trigger_layout_id: None,
            trigger_element: None,
            drawer_layout_id: None,
            drawer_element: None,
            content_view: Rc::new(RefCell::new(None)),
            trigger_bounds: None,
        }
    }
}

pub struct PrepaintState {
    hitbox: Hitbox,
    /// Trigger bounds for limit a rect to handle mouse click.
    trigger_bounds: Option<Bounds<Pixels>>,
    drawer_bounds: Option<Bounds<Pixels>>,
}

impl<M> IntoElement for Drawer<M>
where
    M: ManagedView,
{
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl<M: ManagedView> Element for Drawer<M> {
    type RequestLayoutState = DrawerElementState<M>;
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn request_layout(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let style = Style::default();

        self.with_element_state(id.unwrap(), cx, |view, element_state, cx| {
            let mut trigger_element = view.render_trigger(cx).into_any_element();
            let trigger_layout_id = trigger_element.request_layout(cx);

            let mut drawer_layout_id = None;
            let mut drawer_element = None;

            if let Some(content_view) = element_state.content_view.borrow_mut().as_mut() {
                let content_view_mut = element_state.content_view.clone();
                let mut element = deferred(
                    anchored()
                        .snap_to_window()
                        .position(point(px(0.0) - view.size, px(0.0)))
                        .anchor(AnchorCorner::TopLeft)
                        .child(
                            div()
                                .w(view.size)
                                .occlude()
                                .shadow_xl()
                                .bg(cx.theme().popover)
                                .border_l_1()
                                .border_color(cx.theme().border)
                                .child(content_view.clone())
                                .on_mouse_down_out(move |_, cx| {
                                    // Update the element_state.content_view to `None`,
                                    // so that the `paint`` method will not paint it.
                                    *content_view_mut.borrow_mut() = None;
                                    cx.refresh();
                                }),
                        ),
                )
                .with_priority(1)
                .into_any();

                drawer_layout_id = Some(element.request_layout(cx));
                drawer_element = Some(element);
            }

            let layout_id = cx.request_layout(
                style,
                drawer_layout_id.into_iter().chain(Some(trigger_layout_id)),
            );

            (
                layout_id,
                DrawerElementState {
                    trigger_layout_id: Some(trigger_layout_id),
                    drawer_layout_id,
                    drawer_element,
                    trigger_element: Some(trigger_element),
                    ..Default::default()
                },
            )
        })
    }

    fn prepaint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _bounds: gpui::Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        if let Some(element) = &mut request_layout.trigger_element {
            element.prepaint(cx);
        }
        if let Some(element) = &mut request_layout.drawer_element {
            element.prepaint(cx);
        }

        let trigger_bounds = request_layout
            .trigger_layout_id
            .map(|id| cx.layout_bounds(id));

        // Prepare the popover, for get the bounds of it for open window size.
        let drawer_bounds = request_layout
            .drawer_layout_id
            .map(|id| cx.layout_bounds(id));

        let hitbox = cx.insert_hitbox(trigger_bounds.unwrap_or_default(), false);

        PrepaintState {
            trigger_bounds,
            drawer_bounds,
            hitbox,
        }
    }

    fn paint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        _bounds: gpui::Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        self.with_element_state(id.unwrap(), cx, |this, element_state, cx| {
            element_state.trigger_bounds = prepaint.trigger_bounds;

            if let Some(mut element) = request_layout.trigger_element.take() {
                element.paint(cx);
            }

            if let Some(mut element) = request_layout.drawer_element.take() {
                element.paint(cx);
                return;
            }

            // When mouse click down in the trigger bounds, open the popover.
            let Some(content_build) = this.content.take() else {
                return;
            };
            let old_content_view = element_state.content_view.clone();
            let hitbox_id = prepaint.hitbox.id;
            cx.on_mouse_event(move |event: &MouseDownEvent, phase, cx| {
                if phase == DispatchPhase::Bubble && hitbox_id.is_hovered(cx) {
                    cx.stop_propagation();
                    cx.prevent_default();

                    let new_content_view = (content_build)(cx);
                    let old_content_view1 = old_content_view.clone();

                    let previous_focus_handle = cx.focused();
                    cx.subscribe(&new_content_view, move |modal, _: &DismissEvent, cx| {
                        if modal.focus_handle(cx).contains_focused(cx) {
                            if let Some(previous_focus_handle) = previous_focus_handle.as_ref() {
                                cx.focus(previous_focus_handle);
                            }
                        }
                        *old_content_view1.borrow_mut() = None;
                        cx.refresh();
                    })
                    .detach();

                    cx.focus_view(&new_content_view);
                    *old_content_view.borrow_mut() = Some(new_content_view);
                    cx.refresh();
                }
            });
        });
    }
}
