#[macro_use]
extern crate yew;
extern crate stdweb;

use stdweb::web::Date;
// use yew::services::ConsoleService;
use yew::prelude::*;

type Context = ();

pub struct Model {value: i64, }

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl Component<Context> for Model {
    // Some details omitted. Explore the examples to get more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model { value : 0, }
    }

    fn update(&mut self, msg: Self::Message, l: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
//                 self.console.log("plus one");
            }
            Msg::Decrement => {
                self.value = self.value - 1;
//                 self.console.log("minus one");
            }
            Msg::Bulk(list) => for msg in list {
                self.update(msg, l);
//                 self.console.log("Bulk action");
            },
        }
        true        
        
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
                    <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                    <button onclick=|_| Msg::Bulk(vec![Msg::Increment, Msg::Increment]),>{ "Increment Twice" }</button>
                </nav>
                <p>{ self.value }</p>
                <p>{ Date::new().to_string() }</p>
            </div>
        }
    }
}
