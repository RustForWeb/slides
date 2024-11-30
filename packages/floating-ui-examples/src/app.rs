use leptos::*;

use crate::examples::{flip::FlipExample, placement::PlacementExample, tooltip::TooltipExample};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex m-3 gap-3">
            <PlacementExample />
            <FlipExample />
            <TooltipExample />
        </div>
    }
}
