use icons::BadgeCheck;
use leptos::prelude::*;

use crate::components::ui::{
    badge::Badge,
    item::{
        Item, ItemActions, ItemContent, ItemDescription, ItemGroup, ItemMedia, ItemSeparator,
        ItemTitle,
    },
};

#[component]
fn CertificateItem(year: u16, children: Children) -> impl IntoView {
    view! {
        <Item class="py-3 px-4 rounded-none">
            <ItemMedia>
                <BadgeCheck class="size-4 opacity-[.4]" />
            </ItemMedia>
            <ItemContent>
                {children()}
            </ItemContent>
             <ItemActions>
                <span class="text-xs text-muted-foreground">{year}</span>
             </ItemActions>
        </Item>
    }
}

#[component]
pub fn Certificates() -> impl IntoView {
    view! {
        <section class="border-default">
            <div class="flex flex-wrap gap-2 pb-2">
                <Badge>"Certificates"</Badge>
            </div>
            <ItemGroup attr:role="list" class="w-full rounded-md border">
                <CertificateItem year=2024>
                    <ItemTitle>"PBA Graduate in Hong Kong"</ItemTitle>
                    <ItemDescription>"Polkadot Blockchain Academy."</ItemDescription>
                </CertificateItem>
                <ItemSeparator />
                <CertificateItem year=2023>
                    <ItemTitle>"Cardano Developer Professional"</ItemTitle>
                    <ItemDescription>"EMURGO certified Cardano developer."</ItemDescription>
                </CertificateItem>
                <ItemSeparator />
                <CertificateItem year=2013>
                    <ItemTitle>"MSc in Games Software"</ItemTitle>
                    <ItemDescription>"High-performance graphics programming."</ItemDescription>
                </CertificateItem>
            </ItemGroup>
        </section>
    }
}
