use std::sync::Arc;

use gpui::*;
use wry::dpi::LogicalSize;
use wry::raw_window_handle::HasWindowHandle;
use wry::{dpi, Rect, WebView};

struct WebViewTest {
    view: Arc<WebView>,
}

impl WebViewTest {
    fn new(handle: &dyn HasWindowHandle) -> Self {
        let view = Arc::new(
            wry::WebViewBuilder::new_as_child(&handle)
                .with_html(format!(
                    "<html><body>Hello, world! I'm webview</body></html>"
                ))
                .build()
                .expect("Failed to create webview."),
        );

        Self { view }
    }
}

impl Render for WebViewTest {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("parent")
            .block()
            .overflow_y_scroll()
            .size_full()
            .bg(rgb(0xff0000))
            .justify_center()
            .items_center()
            .child(
                div()
                    .size(Length::Definite(DefiniteLength::Absolute(
                        AbsoluteLength::Pixels(Pixels(100.0)),
                    )))
                    .bg(rgb(0x00ff00))
                    .child("This is webview"),
            )
            .child(WebViewElement {
                view: self.view.clone(),
            })
    }
}

struct WebViewElement {
    view: Arc<WebView>,
}
impl IntoElement for WebViewElement {
    type Element = WebViewElement;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for WebViewElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.flex_grow = 1.0;
        style.size = Size::full();
        let id = cx.request_layout(style, []);
        (id, ())
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        // TODO: Find better way of detecting view visibility
        if bounds.top() > cx.viewport_size().height || bounds.bottom() < Pixels::ZERO {
            self.view.set_visible(false).unwrap();
        } else {
            self.view.set_visible(true).unwrap();

            self.view
                .set_bounds(Rect {
                    size: dpi::Size::Logical(LogicalSize {
                        width: (bounds.size.width.0 - 50.0).into(),
                        height: (bounds.size.height.0 / 2.0).into(),
                    }),
                    position: dpi::Position::Logical(dpi::LogicalPosition::new(
                        bounds.origin.x.into(),
                        bounds.origin.y.into(),
                    )),
                })
                .unwrap();
        }
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        // Nothing to do here
    }
}

fn main() {
    App::new().run(move |cx| {
        let window_bounds = Bounds::centered(None, size(px(1200.0), px(900.0)), cx);

        cx.spawn(|mut cx| async move {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(window_bounds)),
                titlebar: Some(TitlebarOptions {
                    title: None,
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(9.0), px(9.0))),
                }),
                window_min_size: Size {
                    width: px(640.),
                    height: px(480.),
                },
                kind: WindowKind::Normal,
                ..Default::default()
            };

            let window = cx
                .open_window(options, |cx| {
                    let view = WebViewTest::new(cx.raw_window_handle());
                    cx.new_view(|_cx| view)
                })
                .unwrap();

            window
                .update(&mut cx, |_, cx| {
                    cx.activate_window();
                    cx.set_window_title("GPUI App");
                    cx.on_release(|_, _, _cx| {
                        // exit app
                        std::process::exit(0);
                    })
                    .detach();
                })
                .unwrap();
        })
        .detach();
    });
}
