mod app;
mod processor_bridge;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
