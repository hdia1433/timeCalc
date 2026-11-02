use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView
{
    let (count, setCount) = signal(0);
    let doubleCount = move || count.get() * 2;

    view! 
    {
        <button
            on:click=move |_| *setCount.write() += 1
        >
            "Click me"
        </button>
        <ProgressBar progress=count/>
        <ProgressBar
            progress=doubleCount
        />
    }
}

#[component]
fn ProgressBar(
    #[prop(default=50)]
    max: u16,   
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView
{
    view! 
    {
        <progress
            max=max
            value=progress
        />
    }
}