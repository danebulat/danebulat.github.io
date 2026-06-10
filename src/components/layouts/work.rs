use icons::ChevronRight;
use leptos::prelude::*;

use crate::components::ui::{
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonSize, ButtonVariant},
    item::{Item, ItemActions, ItemContent, ItemDescription, ItemTitle, ItemVariant},
    sheet::{Sheet, SheetContent, SheetDirection, SheetHeader, SheetTitle, SheetTrigger},
    tooltip::{Tooltip, TooltipContent, TooltipPosition},
};

struct WorkItem {
    title: String,
    description: String,
    badges: Vec<String>,
    href: String,
    highlight: bool,
}

fn work_items() -> Vec<WorkItem> {
    vec![
        WorkItem {
            title: "Rust Tetris".into(),
            description: "Tetris clone written in Rust.".into(),
            badges: vec!["rust".into(), "rendering".into(), "games".into()],
            href: "https://github.com/danebulat/rust-tetris".into(),
            highlight: false,
        },
        WorkItem {
            title: "Polkadot Live App".into(),
            description: "Decentralized blockchain notifications.".into(),
            badges: vec!["blockchain".into(), "electron".into(), "react".into()],
            href: "https://github.com/polkadot-live/polkadot-live-app".into(),
            highlight: true,
        },
        WorkItem {
            title: "Polkadot Live Documentation".into(),
            description: "Application documentation portal.".into(),
            badges: vec!["docker".into(), "vocs".into(), "typescript".into()],
            href: "https://github.com/polkadot-live/docs".into(),
            highlight: true,
        },
        WorkItem {
            title: "Polkadot Live Landing".into(),
            description: "Application landing page.".into(),
            badges: vec!["react".into(), "tailwind".into(), "csr".into()],
            href: "https://github.com/polkadot-live/landing".into(),
            highlight: false,
        },
    ]
}

#[component]
pub fn Work() -> impl IntoView {
    view! {
        <section id="work-section" class="h-full flex flex-col border-default">
            <div class="flex flex-wrap gap-2 pb-2 items-center">
                <Badge>"Recent Work"</Badge>
                <span class="ml-auto h-full flex items-center">
                    <WorkSheet />
                </span>
            </div>

            <div class="flex flex-col gap-2 w-full flex-1">
                {work_items().into_iter().filter(|item| item.highlight).map(|item| {
                    view! { <WorkItem item=item />}
                }).collect_view()}
            </div>
        </section>
    }
}

#[component]
pub fn WorkSheet() -> impl IntoView {
    view! {
        <Sheet>
            <SheetTrigger size=ButtonSize::Badge variant=ButtonVariant::Outline>
                "View More"
            </SheetTrigger>
            <SheetContent class="p-4" direction=SheetDirection::Right>
                <SheetHeader class="p-0">
                    <div class="flex flex-col gap-4">
                        <SheetTitle>"Recent Work"</SheetTitle>
                        {work_items().into_iter().map(|item| {
                            view! { <WorkItem item=item />}
                        }).collect_view()}
                    </div>
                </SheetHeader>
            </SheetContent>
        </Sheet>
    }
}

#[component]
fn WorkItem(item: WorkItem) -> impl IntoView {
    view! {
        <Item variant=ItemVariant::Outline class="flex-1">
            <ItemContent class="flex-1 flex flex-col gap-2">
                <ItemTitle>{item.title}</ItemTitle>
                <ItemDescription>{item.description}</ItemDescription>
                <div class="flex flex-wrap gap-x-2 gap-y-4">
                    {item.badges.into_iter().map(|label| view! {
                        <Badge variant=BadgeVariant::Secondary>{label}</Badge>
                    }).collect_view()}
                </div>
            </ItemContent>
            <ItemActions>
                <Tooltip>
                    <Button size=ButtonSize::Icon variant=ButtonVariant::Secondary>
                        <a href={item.href} target="_blank" class="w-full h-full flex items-center justify-center">
                            <ChevronRight class="size-4" />
                        </a>
                    </Button>
                    <TooltipContent position=TooltipPosition::Left>"View GitHub"</TooltipContent>
                </Tooltip>
            </ItemActions>
        </Item>
    }
}
