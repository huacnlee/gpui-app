use gpui::{
    actions, div, px, AnchorCorner, AppContext, DismissEvent, Element, EventEmitter, FocusHandle,
    FocusableView, InteractiveElement, IntoElement, MouseButton, MouseDownEvent,
    ParentElement as _, Render, Styled as _, View, ViewContext, VisualContext, WindowContext,
};
use ui::{
    button::{Button, ButtonSize},
    divider::Divider,
    drawer::Drawer,
    h_flex,
    input::TextInput,
    popover::{Popover, PopoverContent},
    popup_menu::PopupMenu,
    prelude::FluentBuilder,
    switch::Switch,
    v_flex, Clickable, IconName,
};

struct Form {
    input1: View<TextInput>,
}

impl Form {
    fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            input1: cx.new_view(TextInput::new),
        })
    }
}

impl FocusableView for Form {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.input1.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for Form {}

impl Render for Form {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .p_4()
            .size_full()
            .child("This is a form container.")
            .child(self.input1.clone())
            .child(
                Button::primary("submit", "Submit", cx)
                    .on_click(cx.listener(|_, _, cx| cx.emit(DismissEvent))),
            )
    }
}

pub struct DrawerStory {
    focus_handle: FocusHandle,
    form: View<Form>,
}

impl DrawerStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(Self::new)
    }

    fn new(cx: &mut ViewContext<Self>) -> Self {
        let form = Form::new(cx);
        Self {
            form,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl FocusableView for DrawerStory {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.form.focus_handle(cx)
    }
}

impl Render for DrawerStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let form = self.form.clone();

        v_flex()
            .p_4()
            .mb_5()
            .size_full()
            .min_h(px(400.))
            .gap_6()
            .child(
                h_flex().items_center().justify_between().child(
                    v_flex().gap_4().child(
                        Drawer::new("info-top-left")
                            .size(px(450.))
                            .trigger(Button::new("info-top-left", cx).label("Open Drawer..."))
                            .content(move |_| form.clone()),
                    ),
                ),
            )
    }
}
