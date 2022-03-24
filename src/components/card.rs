use yew::prelude::*;

use crate::components::Contact;

pub enum Msg {
    ClickDomain,
    ClickName,
    ShowContact,
    ClickProfileUrl,
    ClickLeftPlace,
}

pub struct Card {
    profile_url: String,
    domain: String,
    name: String,
    message: String,
    showing_contact: bool,
}

impl Component for Card {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            profile_url: "https://github.com/Ja-sonYun.png".to_string(),
            domain: "abex.dev".to_string(),
            name: "Ja-sonYun".to_string(),
            message: "show my contact?".to_string(),
            showing_contact: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClickDomain => {
                self.domain = "（ツ）/ ".to_string();
                true
            }
            Msg::ClickName => {
                self.name = "github?".to_string();
                self.update(_ctx, Msg::ShowContact)
            }
            Msg::ShowContact => {
                if self.showing_contact {
                    self.message = "show my contact?".to_string();
                    self.showing_contact = false;
                } else {
                    self.message = "close?".to_string();
                    self.showing_contact = true;
                }
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="h-screen flex items-center justify-center">
                    <div class="
                        transition
                        ease-in-out
                        duration-300
                        hover:scale-110
                        w-72
                        bg-gray-800
                        border
                        rounded-lg
                        border-gray-700
                        p-5
                        shadow
                    ">
                        <div class="flex flex-row">
                            <a href="https://github.com/Ja-sonYun">
                                <img
                                    class="
                                        transition
                                        ease-in-out
                                        duration-300
                                        hover:-translate-y-1
                                        hover:scale-110
                                        rounded
                                        cursor-pointer"
                                    src={self.profile_url.clone()}
                                    alt="profile"
                                    width="40"
                                    height="40"
                                />
                            </a>
                            <p class="ml-3">
                                <span
                                    class="text-gray-500 font-semibold cursor-pointer"
                                    onclick={ctx.link().callback(|_| Msg::ClickDomain)}
                                >
                                    {self.domain.clone() + "/"}
                                </span>
                                <span
                                    class="text-gray-300 font-semibold cursor-pointer"
                                    onclick={ctx.link().callback(|_| Msg::ClickName)}
                                >
                                    {self.name.clone()}
                                </span>
                            </p>
                        </div>
                        <div>
                            <details class="pt-4 rounded-lg" open={self.showing_contact}>
                                <summary
                                    class="text-sx
                                           leading-6
                                           text-gray-500
                                           cursor-pointer
                                           hover:text-white"
                                    onclick={ctx.link().callback(|_| Msg::ShowContact)}
                                >
                                    {self.message.clone()}
                                </summary>
                                <div class="pt-2 grid grid-flow-row auto-rows-max">
                                    <Contact
                                        icon_class="fa-brands fa-github"
                                        label="Ja-sonYun"
                                        link_to="https://github.com/Ja-sonYun"
                                    />
                                    <Contact
                                        icon_class="fa-brands fa-whatsapp"
                                        label="wa.me/821074889832"
                                        link_to="https://wa.me/821074889832"
                                    />
                                    <Contact
                                        icon_class="fa-solid fa-envelope"
                                        label="jason@abex.dev"
                                        link_to="mailto: jason@abex.dev"
                                    />
                                    <Contact
                                        icon_class="fa-brands fa-discord"
                                        label="Jason Yun#8644"
                                        link_to="/"
                                    />
                                </div>
                            </details>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
