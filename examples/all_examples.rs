use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_eventlistener::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_picking_core::events::{Click, Pointer};
use bevy_utils::intern::Internable;
use std::ops::Add;
use std::process::Command;
use std::{env, thread};

mod examples {
    pub mod bevy_ui {
        include!("bevy_ui.rs");
    }
    pub mod debug {
        include!("debug.rs");
    }
    pub mod deselection { include!("deselection.rs"); }
    pub mod drag_and_drop { include!("drag_and_drop.rs"); }
    pub mod egui { include!("egui.rs"); }
    pub mod event_listener { include!("event_listener.rs"); }
    pub mod gltf { include!("gltf.rs"); }
    pub mod many_buttons { include!("many_buttons.rs"); }
    pub mod many_events { include!("many_events.rs"); }
    pub mod minimal { include!("minimal.rs"); }
    pub mod minimal_2d { include!("minimal_2d.rs"); }
    pub mod multiple_windows { include!("multiple_windows.rs"); }
    pub mod rapier { include!("rapier.rs"); }
    pub mod render_layers { include!("render_layers.rs"); }
    pub mod split_screen { include!("split_screen.rs"); }
    pub mod sprite { include!("sprite.rs"); }
    pub mod tinted_highlight { include!("tinted_highlight.rs"); }
    pub mod two_passes { include!("two_passes.rs"); }
    pub mod virtual_pointer { include!("virtual_pointer.rs"); }
}

fn get_list() -> Vec<(&'static str, fn())> {
    vec![
        ("bevy_ui", examples::bevy_ui::main),
        ("debug", examples::debug::main),
        ("deselection", examples::deselection::main),
        ("drag_and_drop", examples::drag_and_drop::main),
        ("egui", examples::egui::main),
        ("event_listener", examples::event_listener::main),
        ("gltf", examples::gltf::main),
        ("many_buttons", examples::many_buttons::main),
        ("many_events", examples::many_events::main),
        ("minimal", examples::minimal::main),
        ("minimal_2d", examples::minimal_2d::main),
        ("multiple_windows", examples::multiple_windows::main),
        ("rapier", examples::rapier::main),
        ("render_layers", examples::render_layers::main),
        ("split_screen", examples::split_screen::main),
        ("sprite", examples::sprite::main),
        ("tinted_highlight", examples::tinted_highlight::main),
        ("two_passes", examples::two_passes::main),
        ("virtual_pointer", examples::virtual_pointer::main),
    ]
}

pub fn main() {
    if let Some(request) = env::args().nth(1) {
        for (label, func) in get_list() {
            if request == label {
                func();
                return;
            }
        }
    }
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(low_latency_window_plugin())
                .disable::<LogPlugin>(),
            DefaultPickingPlugins,
            bevy_egui::EguiPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let padding_left_right = 25.0;
    let padding_top_bottom = 10.0;
    let padding_left_right_val = Val::Px(padding_left_right);
    let padding_top_bottom_val = Val::Px(padding_top_bottom);
    let padding = UiRect::new(
        padding_left_right_val,
        padding_left_right_val,
        padding_top_bottom_val,
        padding_top_bottom_val,
    );
    let margin = UiRect::new(
        padding_left_right_val,
        padding_left_right_val,
        padding_top_bottom_val,
        padding_top_bottom_val,
    );

    let font_size = 20.0;

    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                // This creates 4 exactly evenly sized columns
                grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
                // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                // This creates 4 exactly evenly sized rows
                // grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                // Set a 12px gap/gutter between rows and columns
                // row_gap: Val::Px(12.0),
                // column_gap: Val::Px(12.0),
                ..default()
            },
            ..default()
        })
        .with_children(move |parent| {
            for (index, (label, ..)) in get_list().iter().enumerate() {
                let label = label.clone();
                let color_offset = 0.05 * (index + 1) as f32;
                parent
                    .spawn(ButtonBundle {
                        background_color: BackgroundColor::from(Color::BLACK.add(Color::rgb(
                            color_offset + ((index + 1) % 2) as f32 * 0.1,
                            color_offset + ((index + 1) % 3) as f32 * 0.1,
                            color_offset + ((index + 1) % 4) as f32 * 0.1,
                        ))),
                        style: Style {
                            margin,
                            padding,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(SpatialBundle::default())
                    .insert(On::<Pointer<Click>>::run(move || {
                        _ = Command::new(env::current_exe().unwrap())
                            .args([label])
                            .spawn();
                    }))
                    .with_children(|parent| {
                        parent
                            .spawn(TextBundle {
                                text: Text::from_section(
                                    label.clone(),
                                    TextStyle {
                                        font_size,
                                        color: Color::BLACK,
                                        ..default()
                                    },
                                ),
                                ..default()
                            })
                            .insert(SpatialBundle::default())
                        ;
                    });
            }
        });
}
