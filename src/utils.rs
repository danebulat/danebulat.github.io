use leptos::prelude::*;

/// Flash a border on an element with the given ID.
/// Applies a blue border that fades out over 1 second.
pub fn flash_element_border(element_id: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id(element_id) {
                // Add the flash animation class
                let _ = element.class_list().add_1("border-flash");

                // Remove the class after animation completes (1s)
                let element_clone = element.clone();
                set_timeout(
                    move || {
                        let _ = element_clone.class_list().remove_1("border-flash");
                    },
                    std::time::Duration::from_millis(1000),
                );
            }
        }
    }
}

/// Scroll to an element and flash its border
pub fn scroll_and_flash(element_id: &str) {
    let element_id_owned = element_id.to_string();

    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id(element_id) {
                // Scroll to the element with smooth behavior
                element.scroll_into_view();

                // Flash after a short delay to ensure scroll completes
                set_timeout(
                    move || {
                        flash_element_border(&element_id_owned);
                    },
                    std::time::Duration::from_millis(100),
                );
            }
        }
    }
}
