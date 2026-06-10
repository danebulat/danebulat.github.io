use icons::{Github, Linkedin, Twitter};
use leptos::prelude::*;

use crate::components::ui::{
    avatar::{Avatar, AvatarFallback, AvatarImage},
    button::{Button, ButtonVariant},
    card::{Card, CardDescription, CardTitle},
    tooltip::{Tooltip, TooltipContent},
};

#[component]
pub fn Summary() -> impl IntoView {
    view! {
        <section id="summary-section" class="max-w-3xl mx-auto border-default">
            <Card class="flex flex-col gap-6 px-6 sm:flex-row">
                <div class="w-full rounded-lg sm:w-1/5 sm:h-32 flex items-center justify-center">
                    <Avatar class="w-24 h-24">
                        <AvatarImage attr:src="icon.png" attr:alt="Avatar" />
                        <AvatarFallback>"RS"</AvatarFallback>
                    </Avatar>
                </div>
                <div class="flex-1 pt-0 text-center sm:text-left">
                    <CardTitle class="mb-3">"Dane Bulat"</CardTitle>
                    <CardDescription>
                        <div class="flex flex-col gap-4 pt-1 items-center sm:items-start">
                            <p>
                                "Engineering high-performance software, scalable architectures, and full-stack platforms with Rust and TypeScript. Expertise spans desktop applications, distributed systems, and decentralized technologies."
                            </p>
                            <div class="flex gap-4">
                                <Tooltip>
                                    <Button variant=ButtonVariant::Secondary>
                                        <a href="https://github.com/danebulat" target="_blank">
                                            <Github class="size-5 opacity-[.6]" />
                                        </a>
                                    </Button>
                                    <TooltipContent>"GitHub"</TooltipContent>
                                </Tooltip>
                                <Tooltip>
                                    <Button variant=ButtonVariant::Secondary>
                                        <a href="https://x.com/danebulat" target="_blank">
                                            <Twitter class="size-5 opacity-[.6]" />
                                        </a>
                                    </Button>
                                    <TooltipContent>"X"</TooltipContent>
                                </Tooltip>
                                <Tooltip>
                                    <Button variant=ButtonVariant::Secondary>
                                        <a href="https://www.linkedin.com/in/danebulat/" target="_blank">
                                            <Linkedin class="size-5 opacity-[.6]" />
                                        </a>
                                    </Button>
                                    <TooltipContent>"LinkedIn"</TooltipContent>
                                </Tooltip>
                            </div>
                        </div>
                    </CardDescription>
                </div>
            </Card>
        </section>
    }
}
