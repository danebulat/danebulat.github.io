use leptos::prelude::*;
use std::collections::HashMap;

use crate::components::ui::{
    badge::{Badge, BadgeVariant},
    item::{Item, ItemContent, ItemGroup},
    tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
};

#[component]
pub fn Skills() -> impl IntoView {
    view! {
        <section id="skills-section" class="max-w-3xl mx-auto border-default">
            <Tabs default_value="all" class="flex-1">
                <div class="flex flex-col gap-2 w-full">
                    <Badge>"Skills"</Badge>
                    <TabsList class="gap-2">
                        <TabsTrigger value="all">"All"</TabsTrigger>
                        <TabsTrigger value="software">"Software"</TabsTrigger>
                        <TabsTrigger value="libraries">"Libraries"</TabsTrigger>
                        <TabsTrigger value="devops">"DevOps"</TabsTrigger>
                    </TabsList>
                </div>
                <ItemGroup attr:role="list" class="w-full rounded-md border pb-1">
                    <Item class="py-2 px-4 rounded-none">
                        <ItemContent>
                            <TabsContent value="all">
                                <SkillBadges key="all".into() />
                            </TabsContent>
                            <TabsContent value="software">
                                <SkillBadges key="software".into() />
                            </TabsContent>
                            <TabsContent value="libraries">
                                <SkillBadges key="libraries".into() />
                            </TabsContent>
                            <TabsContent value="devops">
                                <SkillBadges key="devops".into() />
                            </TabsContent>
                        </ItemContent>
                    </Item>
                </ItemGroup>
            </Tabs>
        </section>
    }
}

#[component]
fn SkillBadges(key: String) -> impl IntoView {
    view! {
    <div class="flex flex-wrap gap-x-2 gap-y-4">
        {skills().get(&key).unwrap().clone().into_iter().map(|skill| view! {
            <Badge variant=BadgeVariant::Secondary>{skill}</Badge>
        }).collect_view()}
    </div>
    }
}

fn skills() -> HashMap<String, Vec<String>> {
    let mut skills = hashmap! {
        "software" => [
            "algorithms", "apis", "async", "blockchain", "browsers", "cargo", "cli",
            "concurrency", "containers", "csr", "docker", "fullstack", "graphql", "ipc",
            "memory", "microservices", "node", "open source", "parallism", "pointers", "rest",
            "rpc", "rust", "sql", "ssr", "threads", "tooling", "traits", "trunk", "typescript",
            "vite", "ux", "wasm",
        ],
        "libraries" => [
            "actix-web", "axum", "electron", "js-sys", "leptos", "react", "serde",
            "sqlx", "subxt", "tailwind", "wasm-bindgen", "web-sys",
        ],
        "devops" => [
            "ansible", "ci", "command line", "databases", "deployment", "emacs",
            "git", "github actions", "hosting", "linux", "postgres", "redis", "servers",
            "toml", "yaml", "vim", "vps",
        ],
    };

    // Generate "all" key by combining, sorting, and deduplicating all skills.
    let mut all = Vec::new();
    for key in ["software", "libraries", "devops"] {
        if let Some(items) = skills.get(key) {
            all.extend(items.clone());
        }
    }

    all.sort();
    all.dedup();
    skills.insert("all".to_string(), all);

    skills
}
