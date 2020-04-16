//! [WebAssembly Programming with Rust](https://pragprog.com/book/khrust/programming-webassembly-with-rust)
#![recursion_limit = "256"]
#[macro_use]
extern crate yew;

mod pubnub;

use log::info;
use std::collections::HashSet;
use stdweb::web::event::IKeyboardEvent;
use yew::html::{Component, Env, Html, Renderable, ShouldRender};

pub struct Context {
    inner: pubnub::Service,
}

impl Context {
    pub fn new(pub_key: &str, sub_key: &str) -> Self {
        let inner = pubnub::Service::new(pub_key, sub_key);
        Self { inner }
    }
}

impl AsMut<pubnub::Service> for Context {
    fn as_mut(&mut self) -> &mut pubnub::Service {
        &mut self.inner
    }
}

#[derive(Debug)]
pub struct Message {
    text: String,
    from: String,
}

pub struct Model {
    alias: String,
    pending_text: String,
    messages: Vec<Message>,
    users: HashSet<String>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            alias: String::new(),
            users: HashSet::new(),
            pending_text: String::new(),
        }
    }
}

#[derive(Debug)]
pub enum Event {
    AddMessage(Message),
    UserOffline(String),
    UserOnline(String),
    SendChat,
    Connect,
    EnterName(String),
    UpdatePendingText(String),
    Noop,
}

impl<C> Component<C> for Model
where
    C: AsMut<pubnub::Service> + 'static,
{
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<C, Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, event: Self::Message, env: &mut Env<C, Self>) -> ShouldRender {
        match event {
            Event::AddMessage(msg) => {
                self.messages.push(msg);
            }
            Event::UserOnline(nick) => {
                info!("Adding user {:?}", nick);
                self.users.insert(nick);
            }
            Event::UserOffline(nick) => {
                info!("Removing user {:?}", nick);
                self.users.remove(&nick);
            }
            Event::SendChat => {
                info!("Called send chat!");
                env.as_mut().send_message(&self.pending_text);
                self.pending_text = String::new();
            }
            Event::Connect => {
                let on_message = env.send_back(Event::AddMessage);
                let on_offline = env.send_back(Event::UserOffline);
                let on_online = env.send_back(Event::UserOnline);
                env.as_mut().connect(
                    "chatengine-demo-chat",
                    &self.alias,
                    on_message,
                    on_offline,
                    on_online,
                );
            }
            Event::EnterName(name) => {
                self.alias = name;
            }
            Event::UpdatePendingText(text) => {
                self.pending_text = text;
            }
            Event::Noop => {}
        }
        true
    }
}

impl<C> Renderable<C, Model> for Model
where
    C: AsMut<pubnub::Service> + 'static,
{
    fn view(&self) -> Html<C, Self> {
        html! {
            <div class="wrapper",>
                <div class="chat-text",>
                    <h1>{ "Messages" }</h1><br/>
                    <ul class="message-list",>
                        { for self.messages.iter().enumerate().map(Self::view_message) }
                    </ul>
                </div>
                <div class="users",>
                    <h1>{ "Users" }</h1><br/>
                    <ul class="user-list",>
                        { for self.users.iter().enumerate().map(Self::view_user) }
                    </ul>
                </div>
                <div class="connect",>
                    <input placeholder="Your Name", value=&self.alias,
                        oninput=|e| Event::EnterName(e.value),>
                    </input>
                    <button onclick=|_| Event::Connect,>{ "Connect" }</button>
                </div>
                <div class="text-entry",>
                    <input placeholder="Message Text",
                        class="pending-text",
                        value=&self.pending_text,
                        oninput=|e| Event::UpdatePendingText(e.value),
                        onkeypress=|e| {
                            if e.key() == "Enter" { Event::SendChat } else { Event::Noop }
                        },>
                    </input>
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_message<C>((_id, message): (usize, &Message)) -> Html<C, Self>
    where
        C: AsMut<pubnub::Service> + 'static,
    {
        html! {
            <li>
                <label>
                    <span class="sender",>{"["}{&message.from}{"]"}</span>
                    <span class="chatmsg",>{&message.text}</span>
                </label>
            </li>
        }
    }
    fn view_user<C>((_id, user): (usize, &String)) -> Html<C, Self>
    where
        C: AsMut<pubnub::Service> + 'static,
    {
        html! {
            <li>
                <label>{ user }</label>
            </li>
        }
    }
}
