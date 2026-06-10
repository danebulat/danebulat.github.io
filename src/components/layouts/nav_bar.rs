use leptos::prelude::*;

use crate::components::ThemeSwitch;
use crate::components::ui::navigation_menu::{
    NavigationMenu, NavigationMenuItem, NavigationMenuLink, NavigationMenuList,
    navigation_menu_trigger_style,
};
use crate::utils::scroll_and_flash;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <NavigationMenu class="max-w-3xl mx-auto w-full border-b mb-4 pt-3 pb-2">
            <NavigationMenuList class="justify-start gap-2">
                <NavigationMenuItem>
                    <NavigationMenuLink
                        class=navigation_menu_trigger_style()
                        on:click=move |_| scroll_and_flash("summary-section")
                    >
                        "Summary"
                    </NavigationMenuLink>
                </NavigationMenuItem>

                <NavigationMenuItem>
                    <NavigationMenuLink
                        class=navigation_menu_trigger_style()
                        on:click=move |_| scroll_and_flash("work-section")
                    >
                        "Recent Work"
                    </NavigationMenuLink>
                </NavigationMenuItem>

                <NavigationMenuItem>
                    <NavigationMenuLink
                        class=navigation_menu_trigger_style()
                        on:click=move |_| scroll_and_flash("skills-section")
                    >
                        "Skills"
                    </NavigationMenuLink>
                </NavigationMenuItem>

                <li class="ml-auto">
                    <ThemeSwitch />
                </li>
            </NavigationMenuList>
        </NavigationMenu>
    }
}
