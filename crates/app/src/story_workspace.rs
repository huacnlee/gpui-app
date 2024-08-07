use gpui::*;
use prelude::FluentBuilder as _;
use private::serde::Deserialize;
use story::{
    ButtonStory, CalendarStory, DropdownStory, IconStory, ImageStory, InputStory, ListStory,
    PickerStory, PopupStory, ProgressStory, ResizableStory, ScrollableStory, StoryContainer,
    SwitchStory, TableStory, TextStory, TooltipStory,
};
use workspace::{TitleBar, Workspace};

use std::sync::Arc;
use ui::{
    button::{Button, ButtonStyle},
    popover::Popover,
    popup_menu::PopupMenu,
    theme::{ActiveTheme, Theme},
    Clickable as _, IconName, Sizable,
};

use crate::app_state::AppState;

#[derive(Clone, PartialEq, Eq, Deserialize)]
struct SelectLocale(SharedString);

impl_actions!(locale_switcher, [SelectLocale]);

actions!(workspace, [Open, CloseWindow]);

pub fn init(_app_state: Arc<AppState>, cx: &mut AppContext) {
    cx.on_action(|_action: &Open, _cx: &mut AppContext| {});

    Theme::init(cx);
    ui::init(cx);
    story::init(cx);
}

pub struct StoryWorkspace {
    workspace: View<Workspace>,
    locale_selector: View<LocaleSelector>,
}

impl StoryWorkspace {
    pub fn new(
        _app_state: Arc<AppState>,
        workspace: View<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe_window_appearance(|_workspace, cx| {
            Theme::sync_system_appearance(cx);
        })
        .detach();

        StoryContainer::add_pane(
            "Buttons",
            "Displays a button or a component that looks like a button.",
            ButtonStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "Input",
            "A control that allows the user to input text.",
            InputStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "Text",
            "Links, paragraphs, checkboxes, and more.",
            TextStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "Switch",
            "A control that allows the user to toggle between two states.",
            SwitchStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "Dropdowns",
            "Displays a list of options for the user to pick from—triggered by a button.",
            DropdownStory::new(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "Picker",
            "Picker is a component that allows the user to select an item from a list of options.",
            PickerStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "Popup",
            "A popup displays content on top of the main page.",
            PopupStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "Tooltip",
            "Displays a short message when users hover over an element.",
            TooltipStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "List",
            "A list displays a series of items.",
            ListStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "Icon",
            "Icon use examples",
            IconStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "Image",
            "Render SVG image and Chart",
            ImageStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        // StoryContainer::add_panel(
        //     WebViewStory::view(cx).into(),
        //     workspace.clone(),
        //     DockPosition::Right,
        //     px(450.),
        //     cx,
        // );

        StoryContainer::add_pane(
            "Table",
            "Powerful table and datagrids built.",
            TableStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "Progress",
            "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.",
            ProgressStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "Resizable",
            "Accessible resizable panel groups and layouts with keyboard support.",
            ResizableStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "Scrollable",
            "A scrollable area with scroll bar.",
            ScrollableStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        StoryContainer::add_pane(
            "Calendar",
            "A calendar component.",
            CalendarStory::view(cx).into(),
            workspace.clone(),
            cx,
        )
        .detach();

        let locale_selector = cx.new_view(LocaleSelector::new);
        Self {
            workspace,
            locale_selector,
        }
    }

    pub fn new_local(
        app_state: Arc<AppState>,
        cx: &mut AppContext,
    ) -> Task<anyhow::Result<WindowHandle<Self>>> {
        let window_bounds = Bounds::centered(None, size(px(1600.0), px(1200.0)), cx);

        cx.spawn(|mut cx| async move {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(window_bounds)),
                titlebar: Some(TitlebarOptions {
                    title: None,
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(9.0), px(9.0))),
                }),
                window_min_size: Some(gpui::Size {
                    width: px(640.),
                    height: px(480.),
                }),
                kind: WindowKind::Normal,
                ..Default::default()
            };

            let window = cx.open_window(options, |cx| {
                let workspace = cx.new_view(|cx| Workspace::new(None, cx));
                cx.new_view(|cx| Self::new(app_state.clone(), workspace, cx))
            })?;

            window
                .update(&mut cx, |_, cx| {
                    cx.activate_window();
                    cx.set_window_title("GPUI App");
                    cx.on_release(|_, _, cx| {
                        // exit app
                        cx.quit();
                    })
                    .detach();
                })
                .expect("failed to update window");

            Ok(window)
        })
    }
}

pub fn open_new(
    app_state: Arc<AppState>,
    cx: &mut AppContext,
    init: impl FnOnce(&mut StoryWorkspace, &mut ViewContext<StoryWorkspace>) + 'static + Send,
) -> Task<()> {
    let task: Task<std::result::Result<WindowHandle<StoryWorkspace>, anyhow::Error>> =
        StoryWorkspace::new_local(app_state, cx);
    cx.spawn(|mut cx| async move {
        if let Some(workspace) = task.await.ok() {
            workspace
                .update(&mut cx, |workspace, cx| init(workspace, cx))
                .expect("failed to init workspace");
        }
    })
}

impl Render for StoryWorkspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .flex()
            .flex_1()
            .flex_col()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                TitleBar::new("main-title", Box::new(CloseWindow))
                    .when(cfg!(not(windows)), |this| {
                        this.on_click(|event, cx| {
                            if event.up.click_count == 2 {
                                cx.zoom_window();
                            }
                        })
                    })
                    // left side
                    .child(div().flex().items_center().child("GPUI App"))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_end()
                            .px_2()
                            .mr_3()
                            .gap_2()
                            .child(self.locale_selector.clone())
                            .child(
                                Button::new("theme-mode", cx)
                                    .map(|this| {
                                        if cx.theme().mode.is_dark() {
                                            this.icon(IconName::Sun)
                                        } else {
                                            this.icon(IconName::Moon)
                                        }
                                    })
                                    .small()
                                    .ghost()
                                    .on_click(move |_, cx| {
                                        let mode = match cx.theme().mode.is_dark() {
                                            true => ui::theme::ThemeMode::Light,
                                            false => ui::theme::ThemeMode::Dark,
                                        };

                                        Theme::change(mode, cx);
                                    }),
                            )
                            .child(
                                Button::new("github", cx)
                                    .icon(IconName::GitHub)
                                    .small()
                                    .style(ButtonStyle::Ghost)
                                    .on_click(|_, cx| {
                                        cx.open_url("https://github.com/huacnlee/gpui-component")
                                    }),
                            ),
                    ),
            )
            .child(self.workspace.clone())
    }
}

struct LocaleSelector {
    focus_handle: FocusHandle,
}

impl LocaleSelector {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    fn on_select_locale(&mut self, locale: &SelectLocale, cx: &mut ViewContext<Self>) {
        ui::set_locale(&locale.0);
        cx.refresh();
    }
}

impl Render for LocaleSelector {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let locale = ui::locale().to_string();

        div()
            .id("locale-selector")
            .track_focus(&focus_handle)
            .on_action(cx.listener(Self::on_select_locale))
            .child(
                Popover::new("locale-selector")
                    .anchor(AnchorCorner::TopRight)
                    .trigger(Button::new("btn", cx).small().ghost().icon(IconName::Globe))
                    .content(move |cx| {
                        PopupMenu::build(cx, |this, _cx| {
                            this.menu_with_check(
                                "English",
                                locale == "en",
                                Box::new(SelectLocale("en".into())),
                            )
                            .menu_with_check(
                                "简体中文",
                                locale == "zh-CN",
                                Box::new(SelectLocale("zh-CN".into())),
                            )
                        })
                    }),
            )
    }
}
