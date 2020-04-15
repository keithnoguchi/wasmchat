//! Chapter 5: Advanced JavaScript Integration with [Yew]
//!
//! [yew]: https://lib.rs/yew
use stdweb::web::Date;
use yew::{prelude::*, services::console::ConsoleService};

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

pub struct Context {
    pub console: ConsoleService,
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

#[derive(Default, Debug)]
pub struct Model {
    value: i64,
}

impl<C> Component<C> for Model
where
    C: AsMut<ConsoleService>,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<C, Self>) -> Self {
        Self::new()
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<C, Self>) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                env.as_mut().log("plus one");
            }
            Msg::Decrement => {
                self.value -= 1;
                env.as_mut().log("minus one");
            }
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg, env);
                    env.as_mut().log("bulk action");
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
                    <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
                    <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                    <button onclick=|_| Msg::Bulk(vec![Msg::Increment,
                                                   Msg::Increment]),>
                        { "Increment Twice" }
                    </button>
                </nav>
                <p>{ self.value }</p>
                <p>{ Date::new().to_string() }</p>
            </div>
        }
    }
}

impl Model {
    fn new() -> Self {
        Self::default()
    }
}
