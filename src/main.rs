mod app;
mod log;
mod audio;
mod waves;
mod sytrus;
mod handle;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
