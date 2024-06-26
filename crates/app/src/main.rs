use std::sync::Arc;

use anyhow::Result;
use app_state::AppState;
use assets::Assets;
use gpui::{App, AppContext};

mod app_state;
mod assets;
mod story_workspace;

fn init(app_state: Arc<AppState>, cx: &mut AppContext) -> Result<()> {
    story_workspace::init(app_state.clone(), cx);

    Ok(())
}

fn main() {
    let app_state = Arc::new(AppState {});

    let app = App::new().with_assets(Assets);

    app.run(move |cx| {
        AppState::set_global(Arc::downgrade(&app_state), cx);

        if let Err(e) = init(app_state.clone(), cx) {
            log::error!("{}", e);
            return;
        }

        story_workspace::open_new(app_state.clone(), cx, |_workspace, _cx| {
            // do something
        })
        .detach();
    });
}
