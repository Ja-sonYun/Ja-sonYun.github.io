use yew::prelude::*;

enum Msg {
    ClickDomain,
    ClickName,
    ClickMessage,
    ClickProfileUrl,
    ClickLeftPlace,
}

pub struct Card {
    profile_url: String,
    domain: String,
    name: String,
    message: String,
}

impl Component for Card {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            profile_url: "https://github.com/Ja-sonYun.png".to_string(),
            domain: "abex.dev".to_string(),
            name: "Ja-sonYun".to_string(),
            message: "yes?".to_string(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="h-screen flex items-center justify-center bg-gray-900">
                    <a href="#" class="w-72 bg-slate-900 border rounded-lg border-gray-700 p-5 shadow hover:bg-gray-700 delay-100 duration-200">

                        <div class="flex flex-row">
                            <img
                                class="rounded"
                                src={self.profile_url.clone()}
                                alt="profile"
                                width="40"
                                height="40"
                            />
                            // <img src="https://picsum.photos/40/40" class="rounded" />
                            <p class="ml-3">
                                <span class="text-gray-500 font-semibold">{self.domain.clone() + "/"}</span>
                                <span class="text-gray-300 font-semibold">{self.name.clone()}</span>
                            </p>
                        </div>

                        <p class="text-xs text-gray-500 mt-3">
                            {self.message.clone()}
                        </p>

                    </a>
                </div>
            </div>
        }
    }
}
