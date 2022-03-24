use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub icon_class: String,
    pub label: String,
    pub link_to: String,
}

pub struct Contact;

impl Component for Contact {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            icon_class,
            label,
            link_to,
        } = ctx.props();

        html! {
            <a href={link_to.clone()} class="pt-2 group flex items-center">
                <i class={icon_class}></i>
                <p class="pl-2 text-sm font-medium text-slate-300 group-hover:text-white">{label}</p>
            </a>
        }
    }
}
