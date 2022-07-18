use yew::prelude::*;

enum Msg {
    AddOne,
    RemoveOne,
    Multiply,
    Divide,
}

struct CounterComponent {
    count: i32,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            count: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true // re-render component
            }
            Msg::RemoveOne => {
                self.count -= 1;
                true // re-render component
            }
            Msg::Multiply => {
                self.count *= 2;
                true // re-render component
            }
            Msg::Divide => {
                self.count /= 2;
                true // re-render component
            }
        }
    }

    fn view(&self, content: &Context<Self>) -> Html {
        let link = content.link();
        html! {
            <div class="container">
                <p>{ self.count }</p>
                <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button onclick={link.callback(|_| Msg::RemoveOne)}>{ "-1" }</button>
                <button onclick={link.callback(|_| Msg::Multiply)}>{ "Multiply" }</button>
                <button onclick={link.callback(|_| Msg::Divide)}>{ "Divide" }</button>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
