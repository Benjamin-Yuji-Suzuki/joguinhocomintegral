use bevy::prelude::*;

mod components;
mod gameplay;
mod questions;
mod screens;
mod state;

use gameplay::*;
use questions::*;
use screens::*;
use state::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "rpgcal".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(EstadoJogo::default())
        .insert_resource(BancoPerguntas::default())
        .insert_resource(TelaAtual::Menu)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                menu_input,
                menu_mouse_click,
                tutorial_input,
                tutorial_mouse_click,
                update_timer,
                update_hud,
                handle_mouse_hover,
                handle_mouse_clicks,
                processar_proxima_pergunta,
            ),
        )
        .run();
}
