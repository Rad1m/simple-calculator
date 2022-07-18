use yew::prelude::*;

enum Msg {
    Plus,
    Minus,
    Multiply,
    Divide,
    Result,
    Reset,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

struct CounterComponent {
    display_number: String,
    sign: bool,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            display_number: String::new(),
            sign: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Plus => {
                if !self.display_number.is_empty() && !self.sign {
                    self.display_number.push_str("+");
                    self.sign = true;
                }
                true // re-render component
            }
            Msg::Minus => {
                if !self.display_number.is_empty() && !self.sign {
                    self.display_number.push_str("-");
                    self.sign = true;
                }
                true // re-render component
            }
            Msg::Multiply => {
                if !self.display_number.is_empty() && !self.sign {
                    self.display_number.push_str("x");
                    self.sign = true;
                }
                true // re-render component
            }
            Msg::Divide => {
                if !self.display_number.is_empty() && !self.sign {
                    self.display_number.push_str("/");
                    self.sign = true;
                }
                true // re-render component
            }
            Msg::Result => {
                let first: f64;
                let second: f64;
                let result: f64;
                if self.display_number.contains("+") {
                    let strings: Vec<&str> = self.display_number.split("+").collect();
                    first = strings[0].parse().unwrap();
                    second = strings[1].parse().unwrap();
                    result = first + second;
                    self.display_number = result.to_string();
                } else if self.display_number.contains("-") {
                    let strings: Vec<&str> = self.display_number.split("-").collect();
                    first = strings[0].parse().unwrap();
                    second = strings[1].parse().unwrap();
                    result = first - second;
                    self.display_number = result.to_string();
                } else if self.display_number.contains("x") {
                    let strings: Vec<&str> = self.display_number.split("x").collect();
                    first = strings[0].parse().unwrap();
                    second = strings[1].parse().unwrap();
                    result = first * second;
                    self.display_number = result.to_string();
                } else if self.display_number.contains("/") {
                    let strings: Vec<&str> = self.display_number.split("/").collect();
                    first = strings[0].parse().unwrap();
                    second = strings[1].parse().unwrap();
                    result = first / second;
                    self.display_number = result.to_string();
                }
                self.sign = false;
                true // re-render component
            }
            Msg::Reset => {
                self.display_number = String::new();
                self.sign = false;
                true // re-render component
            }
            Msg::Zero => {
                if !self.display_number.is_empty() {
                    self.display_number.push_str("0");
                }
                true // re-render component
            }
            Msg::One => {
                self.display_number.push_str("1");
                true // re-render component
            }
            Msg::Two => {
                self.display_number.push_str("2");
                true // re-render component
            }
            Msg::Three => {
                self.display_number.push_str("3");
                true // re-render component
            }
            Msg::Four => {
                self.display_number.push_str("4");
                true // re-render component
            }
            Msg::Five => {
                self.display_number.push_str("5");
                true // re-render component
            }
            Msg::Six => {
                self.display_number.push_str("6");
                true // re-render component
            }
            Msg::Seven => {
                self.display_number.push_str("7");
                true // re-render component
            }
            Msg::Eight => {
                self.display_number.push_str("8");
                true // re-render component
            }
            Msg::Nine => {
                self.display_number.push_str("9");
                true // re-render component
            }
        }
    }

    fn view(&self, content: &Context<Self>) -> Html {
        let link = content.link();
        
        html! {
            <div class="container">
            <div style="width:50%;text-align:center;margin-left:0">
                <p style = "font-size: 30px">{ self.sign }</p>
                <p>{ 
                    if !self.display_number.is_empty() && self.display_number != "0" {
                        &self.display_number
                    } else {"0"}
                 }</p>
                <button onclick={link.callback(|_| Msg::Reset)}>{ "AC" }</button>
                <button onclick={link.callback(|_| Msg::Plus)}>{ "+" }</button>
                <button onclick={link.callback(|_| Msg::Minus)}>{ "-" }</button>
                <button onclick={link.callback(|_| Msg::Multiply)}>{ "x" }</button>
                <button onclick={link.callback(|_| Msg::Seven)}>{ "7" }</button>
                <button onclick={link.callback(|_| Msg::Eight)}>{ "8" }</button>
                <button onclick={link.callback(|_| Msg::Nine)}>{ "9" }</button>
                <button onclick={link.callback(|_| Msg::Divide)}>{"/"}</button>
                <button onclick={link.callback(|_| Msg::Four)}>{ "4" }</button>
                <button onclick={link.callback(|_| Msg::Five)}>{ "5" }</button>
                <button onclick={link.callback(|_| Msg::Six)}>{ "6" }</button>
                <button onclick={link.callback(|_| Msg::Result)}>{ "=" }</button>
                <button onclick={link.callback(|_| Msg::One)}>{ "1" }</button>
                <button onclick={link.callback(|_| Msg::Two)}>{ "2" }</button>
                <button onclick={link.callback(|_| Msg::Three)}>{ "3" }</button>
                <button onclick={link.callback(|_| Msg::Zero)}>{ "0" }</button>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
