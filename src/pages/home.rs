use yew::prelude::*;

use crate::components::Card;
use crate::components::ExpandDown;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="relative h-screen">
                <Card/>
                <ExpandDown/>
                <div class="absolute bottom-0 right-0">
                    <a href="https://yew.rs" class="hover:text-red-500">{"made with yew❤️"}</a>
                </div>
            </div>
        }
    }
}
