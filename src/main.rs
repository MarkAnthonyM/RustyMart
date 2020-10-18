mod api;
mod types;
mod pages;

use pages::Home;
use yew::prelude::*;

fn main() {
    yew::start_app::<Home>();
}
