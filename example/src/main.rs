use leptos::*;

mod components;

use components::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum IconType {
    Solid,
    Outline,
    Mini,
}

pub fn main() {
    mount_to_body(move |cx| {
        view! { cx, <App /> }
    });
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (active_icon_type, set_icon_type) = create_signal(cx, IconType::Solid);

    create_effect(cx, move |_| {
        log!("{:?}", active_icon_type.get());
    });

    view! { cx,
        <div class="min-h-full">
            <nav class="border-b border-gray-200 bg-white">
                <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
                    <div class="flex h-16 justify-between">
                        <div class="flex">
                            <div class="hidden sm:-my-px sm:ml-6 sm:flex sm:space-x-8">
                                <NavLink
                                    icon_type=IconType::Solid
                                    active_icon_type
                                    set_icon_type />
                                <NavLink
                                    icon_type=IconType::Outline
                                    active_icon_type
                                    set_icon_type />
                                <NavLink
                                    icon_type=IconType::Mini
                                    active_icon_type
                                    set_icon_type />
                            </div>
                        </div>
                    </div>
                </div>
            </nav>

            <div class="py-10">
                <main>
                    <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
                        {move || match active_icon_type() {
                            IconType::Solid => view! { cx, <Solid24IconGrid /> },
                            IconType::Outline => view! { cx, <Outline24IconGrid /> },
                            IconType::Mini => view! { cx, <Solid20IconGrid /> },
                        }}
                    </div>
                </main>
            </div>
        </div>
    }
}

#[component]
fn NavLink(
    cx: Scope,
    #[prop()] icon_type: IconType,
    #[prop()] active_icon_type: ReadSignal<IconType>,
    #[prop()] set_icon_type: WriteSignal<IconType>,
) -> impl IntoView {
    let icon_type_name = format!("{icon_type:?}");
    view! { cx,
        <button
            class=move || {
                if active_icon_type() == icon_type {
                    "border-indigo-500 text-gray-900 inline-flex items-center border-b-2 px-1 pt-1 text-sm font-medium"
                } else {
                    "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center border-b-2 px-1 pt-1 text-sm font-medium"
                }
            }
            on:click=move |_| set_icon_type.update(|t| *t = icon_type)>
            {icon_type_name}
        </button>
    }
}

#[component]
fn Solid24IconGrid(cx: Scope) -> impl IntoView {
    let icons = size_24_solid(cx)
        .into_iter()
        .map(|icon| {
            view! { cx, <IconCell class="w-6 h-6">{icon}</IconCell>}
        })
        .collect_view(cx);

    view! { cx,
        <IconGrid>{icons}</IconGrid>
    }
}

#[component]
fn Outline24IconGrid(cx: Scope) -> impl IntoView {
    let icons = size_24_outline(cx)
        .into_iter()
        .map(|icon| {
            view! { cx, <IconCell class="w-6 h-6">{icon}</IconCell>}
        })
        .collect_view(cx);

    view! { cx,
        <IconGrid>{icons}</IconGrid>
    }
}

#[component]
fn Solid20IconGrid(cx: Scope) -> impl IntoView {
    let icons = size_20_solid(cx)
        .into_iter()
        .map(|icon| {
            view! { cx, <IconCell class="w-4 h-4">{icon}</IconCell>}
        })
        .collect_view(cx);

    view! { cx,
        <IconGrid>{icons}</IconGrid>
    }
}

#[component]
fn IconGrid(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <ul role="list" class="grid grid-cols-4 gap-x-4 gap-y-8 sm:grid-cols-8 sm:gap-x-6 lg:grid-cols-12 xl:gap-x-8">
            {children(cx)}
        </ul>
    }
}

#[component]
fn IconCell(
    cx: Scope,
    #[prop(optional, into)] class: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <li class="relative flex flex-col items-center">
            <div class=class>{children(cx)}</div>
            // TODO: list the name of the icon underneath it
            // <p class="pointer-events-none block text-sm font-medium text-gray-500">"file.size"</p>
        </li>
    }
}
