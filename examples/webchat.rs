//! Advanced JavaScript Integration with [Yew]
//!
//! [yew]: https://lib.rs/yew
use std::env;

use webchat::{Context, Model};
use yew::app::App;

fn main() {
    let mut args = env::args().skip(1);
    let pub_key = args.next().unwrap_or_else(|| "pub key".to_string());
    let sub_key = args.next().unwrap_or_else(|| "sub key".to_string());
    web_logger::init();
    yew::initialize();
    let app: App<_, Model> = App::new(Context::new(&pub_key, &sub_key));
    app.mount_to_body();
    yew::run_loop();
}
