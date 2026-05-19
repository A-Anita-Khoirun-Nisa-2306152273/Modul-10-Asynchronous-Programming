use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    current_user: String,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            current_user: username,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let value = input.value();
                    if value.trim().is_empty() {
                        return false;
                    }
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(value),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                Some(Msg::SubmitMessage)
            } else {
                None
            }
        });

        html! {
            <div class="flex w-screen h-screen bg-gray-900">

                // ── Sidebar ──────────────────────────────────────────────
                <div class="flex-none w-64 h-screen bg-gradient-to-b from-purple-950 to-indigo-950 flex flex-col shadow-2xl">

                    // App header
                    <div class="p-5 border-b border-purple-800">
                        <div class="flex items-center gap-3">
                            <span class="text-3xl">{"💬"}</span>
                            <div>
                                <h1 class="text-white font-extrabold text-lg tracking-tight">{"YewChat"}</h1>
                                <p class="text-purple-400 text-xs">{"Real-time WebSocket"}</p>
                            </div>
                        </div>
                    </div>

                    // Online users list
                    <div class="flex-grow overflow-y-auto p-3">
                        <div class="flex items-center gap-2 text-purple-400 text-xs uppercase tracking-widest mb-3 px-1">
                            <span class="w-2 h-2 bg-green-400 rounded-full inline-block"></span>
                            {format!("{} online", self.users.len())}
                        </div>
                        {
                            self.users.clone().iter().map(|u| {
                                let is_me = u.name == self.current_user;
                                html!{
                                    <div class={
                                        if is_me {
                                            "flex items-center gap-3 mx-1 my-1 p-2 rounded-xl bg-purple-800 bg-opacity-50 border border-purple-600"
                                        } else {
                                            "flex items-center gap-3 mx-1 my-1 p-2 rounded-xl hover:bg-white hover:bg-opacity-5 transition-colors duration-150"
                                        }
                                    }>
                                        <div class="relative flex-shrink-0">
                                            <img
                                                class="w-10 h-10 rounded-full border-2 border-purple-500"
                                                src={u.avatar.clone()}
                                                alt="avatar"
                                            />
                                            <span class="absolute bottom-0 right-0 w-3 h-3 bg-green-400 rounded-full border-2 border-indigo-950"></span>
                                        </div>
                                        <div class="overflow-hidden">
                                            <div class="text-white text-sm font-semibold truncate">
                                                {u.name.clone()}
                                            </div>
                                            <div class="flex items-center gap-1">
                                                <span class="text-green-400 text-xs">{"● Online"}</span>
                                                if is_me {
                                                    <span class="text-purple-400 text-xs">{"· you"}</span>
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>

                    // Sidebar footer
                    <div class="p-4 border-t border-purple-800 text-center">
                        <p class="text-purple-500 text-xs">{"Powered by Rust 🦀 & Yew"}</p>
                    </div>
                </div>

                // ── Main chat area ────────────────────────────────────────
                <div class="grow h-screen flex flex-col overflow-hidden">

                    // Chat header
                    <div class="flex-none h-16 bg-gray-800 border-b border-gray-700 flex items-center justify-between px-6 shadow-md">
                        <div class="flex items-center gap-3">
                            <span class="text-2xl">{"🗨️"}</span>
                            <div>
                                <h2 class="text-white font-bold">{"General Chat"}</h2>
                                <p class="text-gray-400 text-xs">
                                    {format!("{} participant{} active", self.users.len(), if self.users.len() == 1 { "" } else { "s" })}
                                </p>
                            </div>
                        </div>
                        <div class="flex items-center gap-2 bg-gray-700 rounded-full px-3 py-1">
                            <span class="w-2 h-2 bg-green-400 rounded-full"></span>
                            <span class="text-gray-300 text-xs font-medium">{"Live"}</span>
                        </div>
                    </div>

                    // Messages area
                    <div class="grow overflow-y-auto p-6 flex flex-col gap-4">
                        if self.messages.is_empty() {
                            <div class="flex flex-col items-center justify-center h-full text-center select-none">
                                <div class="text-6xl mb-4">{"👋"}</div>
                                <p class="text-gray-300 text-xl font-semibold">{"No messages yet"}</p>
                                <p class="text-gray-500 text-sm mt-2">{"Say hello and start the conversation!"}</p>
                            </div>
                        }
                        {
                            self.messages.iter().map(|m| {
                                let is_me = m.from == self.current_user;
                                let avatar = self.users.iter()
                                    .find(|u| u.name == m.from)
                                    .map(|u| u.avatar.clone())
                                    .unwrap_or_default();

                                if is_me {
                                    html!{
                                        <div class="flex justify-end items-end gap-2">
                                            <div class="max-w-xs lg:max-w-md xl:max-w-lg">
                                                <p class="text-xs text-gray-500 text-right mb-1 pr-1">{m.from.clone()}</p>
                                                <div class="bg-gradient-to-br from-purple-600 to-indigo-700 text-white rounded-tl-2xl rounded-tr-2xl rounded-bl-2xl px-4 py-2 shadow-lg">
                                                    if m.message.ends_with(".gif") {
                                                        <img class="rounded-lg max-w-full" src={m.message.clone()} alt="gif"/>
                                                    } else {
                                                        <p class="text-sm leading-relaxed">{m.message.clone()}</p>
                                                    }
                                                </div>
                                            </div>
                                            <img class="w-8 h-8 rounded-full border-2 border-purple-600 flex-shrink-0" src={avatar} alt="avatar"/>
                                        </div>
                                    }
                                } else {
                                    html!{
                                        <div class="flex items-end gap-2">
                                            <img class="w-8 h-8 rounded-full border-2 border-gray-600 flex-shrink-0" src={avatar} alt="avatar"/>
                                            <div class="max-w-xs lg:max-w-md xl:max-w-lg">
                                                <p class="text-xs text-gray-500 mb-1 pl-1">{m.from.clone()}</p>
                                                <div class="bg-gray-700 text-gray-100 rounded-tr-2xl rounded-tl-2xl rounded-br-2xl px-4 py-2 shadow-md">
                                                    if m.message.ends_with(".gif") {
                                                        <img class="rounded-lg max-w-full" src={m.message.clone()} alt="gif"/>
                                                    } else {
                                                        <p class="text-sm leading-relaxed">{m.message.clone()}</p>
                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }
                            }).collect::<Html>()
                        }
                    </div>

                    // Input area
                    <div class="flex-none bg-gray-800 border-t border-gray-700 px-6 py-4">
                        <div class="flex items-center gap-3">
                            <input
                                ref={self.chat_input.clone()}
                                type="text"
                                placeholder="💭  Type a message and press Enter..."
                                class="flex-grow py-3 px-5 bg-gray-700 text-white placeholder-gray-400 rounded-full outline-none focus:ring-2 focus:ring-purple-500 text-sm transition-all duration-200"
                                name="message"
                                onkeypress={onkeypress}
                            />
                            <button
                                onclick={submit}
                                class="w-11 h-11 flex-shrink-0 bg-gradient-to-br from-purple-600 to-indigo-700 hover:from-purple-500 hover:to-indigo-600 rounded-full flex justify-center items-center shadow-lg transition-all duration-200 active:scale-95"
                            >
                                <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white w-5 h-5">
                                    <path d="M0 0h24v24H0z" fill="none"></path>
                                    <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                                </svg>
                            </button>
                        </div>
                        <p class="text-gray-600 text-xs text-center mt-2">
                            {"Press Enter to send  •  End message with .gif to send a GIF"}
                        </p>
                    </div>
                </div>

            </div>
        }
    }
}
