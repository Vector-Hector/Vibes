mod app;
mod log;
mod audio;
mod waves;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
