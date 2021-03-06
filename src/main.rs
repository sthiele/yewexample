extern crate yew;
extern crate yewexample;

use yew::prelude::*;
use yewexample::Model;

fn main() {
    yew::initialize();
    let app : App<_,Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
