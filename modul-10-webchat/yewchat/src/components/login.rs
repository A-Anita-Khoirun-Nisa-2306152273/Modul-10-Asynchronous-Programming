use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="min-h-screen w-screen bg-gradient-to-br from-indigo-950 via-purple-900 to-pink-900 flex items-center justify-center">
            <div class="w-full max-w-sm mx-4">
                // Card
                <div class="bg-white bg-opacity-10 rounded-3xl p-10 shadow-2xl border border-white border-opacity-20">
                    // Logo & Title
                    <div class="text-center mb-8">
                        <div class="text-7xl mb-3">{"💬"}</div>
                        <h1 class="text-4xl font-extrabold text-white tracking-tight">{"YewChat"}</h1>
                        <p class="text-purple-300 text-sm mt-2">{"Where every conversation comes alive ✨"}</p>
                    </div>

                    // Form
                    <div class="flex flex-col gap-4">
                        <div class="relative">
                            <span class="absolute left-4 top-1/2 transform -translate-y-1/2 text-lg">{"👤"}</span>
                            <input
                                {oninput}
                                class="w-full pl-12 pr-4 py-3 rounded-xl bg-white bg-opacity-15 text-white placeholder-purple-300 border border-white border-opacity-25 focus:outline-none focus:border-purple-400 focus:bg-opacity-20"
                                placeholder="Enter your username"
                            />
                        </div>
                        <Link<Route> to={Route::Chat}>
                            <button
                                {onclick}
                                disabled={username.len() < 1}
                                class="w-full py-3 rounded-xl bg-purple-600 hover:bg-purple-500 disabled:opacity-40 disabled:cursor-not-allowed text-white font-bold uppercase tracking-widest text-sm transition-colors duration-200 shadow-lg"
                            >
                                {"🚀  Start Chatting!"}
                            </button>
                        </Link<Route>>
                    </div>

                    // Divider & footer
                    <div class="mt-8 pt-6 border-t border-white border-opacity-15 text-center">
                        <p class="text-purple-300 text-xs font-medium tracking-widest uppercase">
                            {"Connect  •  Share  •  Express"}
                        </p>
                        <p class="text-purple-500 text-xs mt-2">
                            {"Built with Rust 🦀 & Yew WebAssembly"}
                        </p>
                    </div>
                </div>

                // Tagline below card
                <p class="text-center text-purple-400 text-xs mt-4 opacity-60">
                    {"Real-time WebSocket chat — no page refresh needed"}
                </p>
            </div>
        </div>
    }
}
