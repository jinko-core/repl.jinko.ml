use yew::prelude::*;

enum Msg {
    Update(String),
    Eval,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    input: String,
    output: Vec<String>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input: String::new(),
            output: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(content) => {
                self.input = content;
                false
            },
            Msg::Eval => {
                self.output.push(self.input.clone() + "\n");
                self.input.clear();
                true
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p style="border: 5px solid green;">
                    { for self.output.iter().map(|line| {
                        html!{<div> { line } <hr/> </div> }
                      })
                    }
                </p>

                <textarea
                    oninput=self.link.callback(|event: InputData| Msg::Update(event.value))
                    rows="1" cols="80">
                </textarea>
                <button onclick=self.link.callback(|_| Msg::Eval)>{ "eval" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
