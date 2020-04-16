//! [Pubnub] [chat] wrapper
//!
//! [pubnub]: https://pubnub.com
//! [chat]: https://www.pubnub.com/docs/chat/quickstart
use super::Message;
use log::info;
use stdweb::{js, Value};
use yew::callback::Callback;

pub struct Service {
    lib: Option<Value>,
    chat: Option<Value>,
}

impl Service {
    pub fn new(publish_key: &str, subscribe_key: &str) -> Self {
        info!("Creating new instance of pubnub chatengine service");
        let chat_engine = js! {
            const clientUUID = PubNub.generateUUID();
            let ce = new PubNub({
                publishKey: @{publish_key},
                subscribeKey: @{subscribe_key},
                uuid: clientUUID,
            });
            console.log("Chat engine core created");
            return ce;
        };
        Self {
            lib: Some(chat_engine),
            chat: None,
        }
    }

    pub fn send_message(&mut self, msg: &str) {
        js! {
            let m = @{msg};
            myChat.emit("message", {
                text: m
            });
        }
    }

    pub fn connect(
        &mut self,
        topic: &str,
        _nickname: &str,
        onmessage: Callback<Message>,
        onoffline: Callback<String>,
        ononline: Callback<String>,
    ) {
        let lib = self.lib.as_ref().expect("No pubnub library");
        let chat_callback = move |text: String, from: String| {
            let msg = Message { text, from };
            onmessage.emit(msg);
        };
        let useroffline_callback = move |username: String| {
            onoffline.emit(username);
        };
        let useronline_callback = move |username: String| {
            ononline.emit(username);
        };
        let chat = js! {
            var pn = @{lib};
            var chat_cb = @{chat_callback};
            var online_cb = @{useronline_callback};
            var offline_cb = @{useroffline_callback};

            pn.on("$.ready", function(data) {
                console.log("PubNub Chat Engine Ready!");
                me = data.me;
                myChat = new pn.Chat(@{topic});
                myChat.on("$.connected", () => {
                    console.log("The chat is connected!");
                    myChat.on("message", (message) => {
                        chat_cb(message.data.text, message.sender.state.nickName);
                        console.log("message: " + messgae.data.text +
                                    " from " + message.sender.state.nickName);
                    });
                    myChat.on("$.online.*", (data) => {
                        console.log("User is Online: ", data.user);
                        online_cb(data.user.state.nickName);
                    });
                    myChat.on("$.offline.*", (data) => {
                        console.log("User is Offline: ", data.user);
                        offline_cb(data.user.state.nickName);
                    });
                });
            });
            console.log("pubnub connecting");
            return myChat;
        };
        self.chat = Some(chat);
    }
}
