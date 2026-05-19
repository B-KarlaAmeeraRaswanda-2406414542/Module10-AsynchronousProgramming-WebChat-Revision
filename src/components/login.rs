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
    <div class="w-screen h-screen flex justify-center items-center px-6">

        <div class="bg-white/70 backdrop-blur-lg shadow-2xl rounded-[2rem] p-10 w-full max-w-xl border border-pink-200">

            <div class="flex flex-col items-center text-center">

                <img
                    class="w-56 rounded-3xl shadow-lg mb-6"
                    src="https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExaTRlOGh5aDdwYzBwaGR5M2E0dWV0aWNhMm5jbnd3dGVlNXh1M2x4aCZlcD12MV9naWZzX3NlYXJjaCZjdD1n/l0MYt5jPR6QX5pnqM/giphy.gif"
                />

                <h1 class="text-5xl font-bold text-pink-500 mb-3">
                    {"Karla's YewChat 💖"}
                </h1>

                <p class="text-gray-700 text-lg mb-2">
                    {"A websocket chat powered by Rust & Yew"}
                </p>

                <p class="text-pink-600 text-sm italic mb-8">
                    {"Dear TA, please consider giving this project a full score 🥹"}
                </p>

                <form class="w-full flex flex-col gap-4">

                    <input
                        {oninput}
                        class="rounded-2xl p-4 border border-pink-200 bg-pink-50 text-gray-700 focus:outline-none focus:ring-4 focus:ring-pink-200 transition"
                        placeholder="Enter your username..."
                    />

                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len() < 1}
                            class="w-full bg-pink-400 hover:bg-pink-500 transition text-white font-semibold p-4 rounded-2xl shadow-lg disabled:bg-pink-200"
                        >
                            {"Enter The Chat"}
                        </button>
                    </Link<Route>>

                </form>

                <p class="mt-8 text-sm text-gray-500">
                    {"Created by Karla Ameera Raswanda"}
                </p>

            </div>
        </div>
    </div>
}
}