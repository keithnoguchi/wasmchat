//! Advanced JavaScript Integration with [Yew]
//!
//! [yew]: https://lib.rs/yew
use wasmchat::ch05::{Context, Model};
use yew::{self, app::App, services::console::ConsoleService};

fn main() {
    yew::initialize();
    let ctx = Context {
        console: ConsoleService::new(),
    };
    let app: App<_, Model> = App::new(ctx);
    app.mount_to_body();
    yew::run_loop();
}
