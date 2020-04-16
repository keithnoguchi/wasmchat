//! Advanced JavaScript Integration with [Yew]
//!
//! [yew]: https://lib.rs/yew
#[macro_use]
extern crate yew;
use yew::app::App;

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(Context::new());
    app.mount_to_body();
    yew::run_loop();
}

use stdweb::web::Date;
use yew::{
    html::{Component, Env, Html, Renderable, ShouldRender},
    services::console::ConsoleService,
};

enum Event {
    Increment,
    Decrement,
    Bulk(Vec<Event>),
}

struct Context {
    inner: ConsoleService,
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.inner
    }
}

impl Context {
    fn new() -> Self {
        Self {
            inner: ConsoleService::new(),
        }
    }
}

#[derive(Default, Debug)]
struct Model {
    counter: i64,
}

impl<C> Component<C> for Model
where
    C: AsMut<ConsoleService>,
{
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<C, Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, event: Self::Message, env: &mut Env<C, Self>) -> ShouldRender {
        match event {
            Event::Increment => {
                self.counter += 1;
                env.as_mut().log("plus one");
            }
            Event::Decrement => {
                self.counter -= 1;
                env.as_mut().log("minus one");
            }
            Event::Bulk(events) => {
                for event in events {
                    self.update(event, env);
                    env.as_mut().log("bulk events");
                }
            }
        }
        true
    }
}

impl<C> Renderable<C, Self> for Model
where
    C: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<C, Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Event::Increment,>{ "Increment" }</button>
                    <button onclick=|_| Event::Decrement,>{ "Decrement" }</button>
                    <button onclick=|_| Event::Bulk(vec![Event::Increment,
                                                    Event::Increment]),>
                        { "Increment Twice" }
                    </button>
                </nav>
                <p>{ self.counter }</p>
                <p>{ Date::new().to_string() }</p>
            </div>
        }
    }
}
