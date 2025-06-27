use floating_ui_leptos::{
    ARROW_NAME, Arrow, ArrowData, ArrowOptions, DetectOverflowOptions, Flip, FlipOptions,
    IntoReference, MiddlewareVec, Offset, OffsetOptions, Padding, Placement, Shift, ShiftOptions,
    Side, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::{
    html::{Button, Div},
    *,
};

use crate::components::{Chrome, GridItem};

#[component]
pub fn TooltipExample() -> impl IntoView {
    let reference_ref = create_node_ref::<Button>();
    let floating_ref = create_node_ref::<Div>();
    let arrow_ref = create_node_ref::<Div>();

    let (open, set_open) = create_signal(false);

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(6.0))),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default().detect_overflow(
            DetectOverflowOptions::default().padding(Padding::All(5.0)),
        ))),
        Box::new(Arrow::new(ArrowOptions::new(arrow_ref))),
    ];

    let UseFloatingReturn {
        placement,
        floating_styles,
        middleware_data,
        ..
    } = use_floating(
        reference_ref.into_reference(),
        floating_ref,
        UseFloatingOptions::default()
            .open(open.into())
            .placement(Placement::Top.into())
            .middleware(middleware.into())
            .while_elements_mounted_auto_update(),
    );

    let static_side = Signal::derive(move || placement.get().side().opposite());
    let arrow_data =
        Signal::derive(move || -> Option<ArrowData> { middleware_data.get().get_as(ARROW_NAME) });
    let arrow_x = Signal::derive(move || {
        arrow_data
            .get()
            .and_then(|arrow_data| arrow_data.x.map(|x| format!("{x}px")))
    });
    let arrow_y = Signal::derive(move || {
        arrow_data
            .get()
            .and_then(|arrow_data| arrow_data.y.map(|y| format!("{y}px")))
    });

    view! {
        <GridItem
            title="Tooltip"
            description="Places your floating element relative to another element."
            chrome=move || view! {
                <Chrome
                    label="Hover the button"
                    center=true
                    shadow=false
                >
                    <button
                        _ref=reference_ref
                        id="button"
                        class="bg-[#dce2ec] text-[#1f2028] border-none rounded-[4px] py-0 px-[8px] text-[1.125rem] leading-[1.778]"
                        aria-describedby="tooltip"
                        on:mouseenter=move |_| set_open.set(true)
                        on:mouseleave=move |_| set_open.set(false)
                        on:focus=move |_| set_open.set(true)
                        on:blur=move |_| set_open.set(false)
                    >
                        My button
                    </button>

                    <div
                        _ref=floating_ref
                        id="tooltip"
                        class="w-max absolute top-0 left-0 bg-[#1f2028] text-[#ffffff] font-bold p-[4px] rounded-[4px] text-[0.875rem] leading-[1.25rem] pointer-events-none"
                        role="tooltip"
                        style:display=move || match open.get() {
                            true => "block",
                            false => "none"
                        }
                        style:position=move || floating_styles.get().style_position()
                        style:top=move || floating_styles.get().style_top()
                        style:left=move || floating_styles.get().style_left()
                        style:transform=move || floating_styles.get().style_transform()
                        style:will-change=move || floating_styles.get().style_will_change()
                    >
                        My tooltip with more content
                        <div
                            _ref=arrow_ref
                            id="arrow"
                            class="absolute bg-[#1f2028] w-[8px] h-[8px] rotate-45"
                            style:left=move || match static_side.get() {
                                Side::Left => Some("-4px".into()),
                                _ => arrow_x.get()
                            }
                            style:top=move || match static_side.get() {
                                Side::Top => Some("-4px".into()),
                                _ => arrow_y.get()
                            }
                            style:right=move || match static_side.get() {
                                Side::Right => Some("-4px"),
                                _ => None
                            }
                            style:bottom=move || match static_side.get() {
                                Side::Bottom => Some("-4px"),
                                _ => None
                            }
                        />
                    </div>
                </Chrome>
            }
        />
    }
}
