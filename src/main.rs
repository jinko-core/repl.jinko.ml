use jinko;
use yew::prelude::*;

enum Msg {
    Update(InputEvent),
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
            Msg::Update(event) => {
                match event.input_type().as_str() {
                    "deleteContentBackward" => {
                        self.input.pop();
                    }
                    "insertText" => {
                        event.data().as_ref().map(|data| self.input.push_str(data));
                    }
                    "insertLineBreak" => self.input.push_str("\n"),
                    &_ => unimplemented!("\"{}\" is not handled", event.input_type().as_str()),
                }

                log::debug!("New input {}", self.input);
                false
            }

            Msg::Eval => {
                log::debug!("{}", self.input);
                self.jk_context.set_code(self.input.clone());
                self.output.push(format!("> {}\n", self.input));
                if let Ok(Some(res)) = self.jk_context.eval(&self.input) {
                    self.output.push(format!("{:?}\n", res));
                }

                self.jk_context.clear_errors();
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
                    oninput={ctx.link().callback(|event: InputEvent| Msg::Update(event))}
                    value={""}
                    rows="50" cols="80">
                </textarea>
                <button onclick={ctx.link().callback(|_| Msg::Eval)}>{ "eval" }</button>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
     yew::Renderer::<Model>::new().render();
}
