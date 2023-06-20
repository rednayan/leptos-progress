use leptos::{ev::SubmitEvent, html::Input, *};

fn main() {
    mount_to_body(|cx| view! { cx,  <App /> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (x, set_x) = create_signal(cx, 0);
    let (y, set_y) = create_signal(cx, 0);
    let (progress, set_progress) = create_signal(cx, 0);
    let double_progress = move || progress() * 2;
    let values = vec![1, 2, 3, 4];
    view! {
        cx,
        <div
            style="position: absolute"
            style:left=move || format!("{}px",x() + 100)
            style:bottom=move || format!("{}px",y() + 100)
        >"Move"</div>
        <button on:click = move |_| {
            set_count.update(|n| *n += 1);
            set_x.update(|n| *n += 100);
            set_y.update(|n| *n += 100);
            set_progress.update(|n| *n += 10);
        }
        class:red = move ||
            count() % 2 == 1
        >
        "Click me: "
        {move || count.get()}
        </button>
        <ProgressBar
            progress = progress
        />
        <ProgressBar
            progress = Signal::derive(cx,double_progress)
        />
        <ul> {values.into_iter().map(|x| view! {cx, <li>{x}</li>}).collect::<Vec<_>>()}</ul>
        <DynamicList initial_length = 10/>
        <ControlledForm />
        <UncontrolledFrom />
    }
}

#[component]
fn ProgressBar(
    cx: Scope,
    #[prop(into)] progress: Signal<i32>,
    #[prop(default = 50)] max: u16,
) -> impl IntoView {
    view! {
        cx,
        <progress
            max = max
            value=move || progress()
        />
    }
}

#[component]
fn DynamicList(cx: Scope, initial_length: usize) -> impl IntoView {
    let mut next_counter_id = initial_length;

    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(cx, id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(cx, initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(cx, next_counter_id + 1);
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        next_counter_id += 1;
    };

    view! { cx,
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    view=move |cx, (id, (count, set_count))| {
                        view! { cx,
                            <li>
                                <button
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, _)| counter_id != &id)
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn ControlledForm(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "controlled".to_string());
    view! {
        cx,
        <input
            on:input = move |ev| {
                set_name(event_target_value(&ev))
            }
        prop:value = name
        />
        <p> {move || name()} </p>
    }
}

#[component]
fn UncontrolledFrom(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "uncontrolled form".to_string());
    let input_element: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element().expect("<input> to exist").value();
        set_name(value);
    };
    view! {
        cx,
        <form on:submit=on_submit>
            <input type="text" value=name node_ref = input_element/>
            <input type="submit" value="submit" />
        </form>
        <p>{name}</p>

    }
}
