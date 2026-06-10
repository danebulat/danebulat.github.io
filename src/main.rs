use leptos::{logging, mount};
use leptos_csr_site::app::App;

fn main() {
    console_error_panic_hook::set_once();
    logging::log!("csr mode - mounting to body");
    mount::mount_to_body(App);
}
