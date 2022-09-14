use crate::md::{Markdown};
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

pub struct App {
    text: String,
}

pub enum Msg {
    Change(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            text: "".to_string(),
        }
    }

    fn update(&mut self,_ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Change(val) => {
                self.text = val;
                if self.text.ends_with("\n") {
                    //if let Some(text)=self.text.strip_suffix('\n');//.to_string();
                    match self.text.strip_suffix('\n'){
                        Some(text) => { self.text = text.to_string();}
                        None => {}
                    }
                    self.text.push_str("  \n");
                }
                log::info!("Add: {:?}", self.text);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: InputElement = e.target_unchecked_into();
            let value = input.value();
            Msg::Change(value)
        });

        //let oninput = ctx.link().callback(|e: InputData| Msg::Change(e.value()))
        html! {
            <>
                <header>
                    <p>
                        {"Yew Markdown Preview"}
                    </p>
                </header>
                <div class={"container"}>
                    <textarea
                       {oninput} 
                  //     {onkeypress}
                       value={self.text.to_string()}
                    />
                    <Markdown prop={self.text.to_string()}/>
                </div>
            </>
        }
    }
}