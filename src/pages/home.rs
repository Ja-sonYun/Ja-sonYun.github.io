use yew::prelude::*;

use crate::components::Card;

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
                <div class="absolute bottom-0 inset-x-0 flex justify-center pb-5">
                    <i class="fa-regular fa-chevron-down"></i>
                </div>
            </div>
        }
    }
}
