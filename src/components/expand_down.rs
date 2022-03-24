use yew::prelude::*;

pub struct ExpandDown;

impl Component for ExpandDown {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="absolute bottom-0 inset-x-0 flex justify-center pb-5 animate-bounce">
                <i class="fa-regular fa-chevron-down"></i>
            </div>
        }
    }
}
