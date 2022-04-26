use yew::prelude::*;
use web_sys::{HtmlElement, HtmlInputElement};

enum Msg {
    AddOne,
    Update(String)
}

struct Model {
    input: String,
    todos: Vec<String>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: String::from("111"),
            todos: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message, ) -> bool {
        match msg {
            Msg::AddOne => {
                self.todos.push(self.input.clone());
                println!("{}",self.input);
                self.input = "".to_string();
                true
            }
            Msg::Update(s) => {
                self.input = s;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let view_todo = |(i, todo): (usize, &String)| {
            html! {
                <li>{format!("{} |", todo)}</li>
            }
        };
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <>
                <div>
                    <input
                      value={self.input.clone()}
                      oninput={link.callback(|e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Msg::Update(input.value())
                    })}
                    />
                    <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                </div>
                <ul>
                    {
                        for self.todos.iter().enumerate().map(view_todo)
                    }
                </ul>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}