use jinko;
use yew::prelude::*;

enum Msg {
    Update(Option<String>),
    Eval,
}

struct Model {
    input: String,
    output: Vec<String>,
    jk_context: jinko::Context,
}

#[derive(Clone, Copy)]
struct Reader;
impl jinko::JkReader for Reader {
    fn read_to_string(&self, path: &str) -> Result<String, jinko::Error> {
        panic!("Read is not implemented, tried to be called on {}", path);
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: String::new(),
            output: Vec::new(),
            jk_context: jinko::Context::new(Box::new(Reader)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(content) => {
                match content {
                    None => (),
                    Some(content) => self.input = content,
                }
                false
            }
            Msg::Eval => {
                self.jk_context.eval(&self.input).unwrap();
                self.output.push(self.input.clone() + "\n");
                self.input.clear();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p style="border: 5px solid green;">
                    { for self.output.iter().map(|line| {
                        html!{<div> { line } <hr/> </div> }
                      })
                    }
                </p>

                <textarea
                    oninput={ctx.link().callback(|event: InputEvent| Msg::Update(event.data()))}
                    rows="1" cols="80">
                </textarea>
                <button onclick={ctx.link().callback(|_| Msg::Eval)}>{ "eval" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
