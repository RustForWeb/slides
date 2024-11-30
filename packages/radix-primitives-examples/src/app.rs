use leptos::*;

use crate::examples::{
    avatar::AvatarExample, checkbox::CheckboxExample, label::LabelExample,
    progress::ProgressExample, switch::SwitchExample, toggle::ToggleExample,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-10">
            <AvatarExample />
            <CheckboxExample />
            <LabelExample />
            <ProgressExample />
            <SwitchExample />
            <ToggleExample />
        </div>
    }
}
