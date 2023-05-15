mod app;
mod log;
mod audio;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
