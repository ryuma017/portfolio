use yew::prelude::*;

pub struct Works;

impl Component for Works {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{"No content"}</h1>
        }
    }
}
