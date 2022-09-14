use pulldown_cmark::{html::{push_html}, Options, Parser};
use yew::prelude::*;
use yew::virtual_dom::VNode;

pub struct Markdown;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
   pub  prop: String,
}

impl Component for Markdown {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        log::info!("Old: {:?}", ctx.props().prop);
        let parser = Parser::new_ext(&ctx.props().prop, options);
        let mut parsed_text = String::new();
        push_html(&mut parsed_text, parser);
        let html_text = format!("<div class='markdown-body'>{}</div>", parsed_text);

        let div = gloo::utils::document().create_element("div").unwrap();
        div.set_inner_html(&html_text);
        // usage
        log::info!("Update: {:?}", html_text);
        let vnode = VNode::VRef(div.into());
        vnode 
    }
}