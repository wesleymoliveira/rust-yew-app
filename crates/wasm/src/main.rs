use wasm::App;
use web_logger;

fn main() {
    web_logger::init();
    yew::start_app::<App>();
}
