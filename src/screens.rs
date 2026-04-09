use bevy::app::AppExit;
use bevy::prelude::*;

use crate::components::*;
use crate::gameplay::iniciar_jogo;
use crate::questions::BancoPerguntas;
use crate::state::{EstadoJogo, TelaAtual};

pub fn spawn_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let fonte = asset_server.load("FiraSans-Bold.ttf");

    // Background do menu (mesmo do jogo)
    commands.spawn((
        Sprite::from_image(asset_server.load("background.png")),
        Transform::from_xyz(0.0, 0.0, -1.0),
        Background,
    ));

    commands.spawn((
        Text2d::new("RPGCAL - Integrais"),
        TextFont {
            font: fonte.clone(),
            font_size: 64.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 230.0, 6.0),
        MenuUI,
    ));

    spawn_botao_menu(
        commands,
        &fonte,
        "[1] Comecar",
        Vec3::new(0.0, 100.0, 5.0),
        Color::srgba(0.2, 0.5, 0.2, 0.75),
        MenuAcao::Comecar,
    );
    spawn_botao_menu(
        commands,
        &fonte,
        "[2] Tutorial",
        Vec3::new(0.0, 20.0, 5.0),
        Color::srgba(0.2, 0.3, 0.6, 0.75),
        MenuAcao::Tutorial,
    );
    spawn_botao_menu(
        commands,
        &fonte,
        "[3] Sair",
        Vec3::new(0.0, -60.0, 5.0),
        Color::srgba(0.6, 0.2, 0.2, 0.75),
        MenuAcao::Sair,
    );

    commands.spawn((
        Text2d::new("Clique com o mouse ou use 1/2/3."),
        TextFont {
            font: fonte,
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        Transform::from_xyz(0.0, -200.0, 6.0),
        MenuUI,
    ));
}

fn spawn_botao_menu(
    commands: &mut Commands,
    fonte: &Handle<Font>,
    texto: &str,
    pos: Vec3,
    cor: Color,
    acao: MenuAcao,
) {
    commands.spawn((
        Sprite::from_color(cor, Vec2::new(460.0, 64.0)),
        Transform::from_translation(pos),
        MenuUI,
        MenuBotao {
            acao,
            largura: 460.0,
            altura: 64.0,
        },
    ));
    commands.spawn((
        Text2d::new(texto),
        TextFont {
            font: fonte.clone(),
            font_size: 34.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(pos.x, pos.y, pos.z + 1.0),
        MenuUI,
    ));
}

pub fn spawn_tutorial(commands: &mut Commands, asset_server: &Res<AssetServer>, iniciar_depois: bool) {
    let fonte = asset_server.load("FiraSans-Bold.ttf");
    let rodape = "Use os botoes abaixo.";

    // Background do tutorial (mesmo do jogo)
    commands.spawn((
        Sprite::from_image(asset_server.load("background.png")),
        Transform::from_xyz(0.0, 0.0, -1.0),
        Background,
    ));

    commands.spawn((
        Text2d::new("Tutorial rapido: integrais iniciais"),
        TextFont {
            font: fonte.clone(),
            font_size: 54.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 250.0, 5.0),
        TutorialUI,
    ));
    commands.spawn((
        Text2d::new(
            "1) Regra da potencia:\n∫ x^n dx = x^(n+1)/(n+1) + C\n\n2) Constante:\n∫ k dx = kx + C\n\n3) Soma de termos:\n∫(ax^m + bx^n + c)dx = ∫ax^m dx + ∫bx^n dx + ∫c dx\n\nExemplos:\n∫2x dx = x^2 + C\n∫3 dx = 3x + C\n∫x dx = x^2/2 + C",
        ),
        TextFont {
            font: fonte.clone(),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.95, 0.85)),
        Transform::from_xyz(0.0, 20.0, 5.0),
        TutorialUI,
    ));
    commands.spawn((
        Text2d::new(rodape),
        TextFont {
            font: fonte.clone(),
            font_size: 34.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.6)),
        Transform::from_xyz(0.0, -250.0, 5.0),
        TutorialUI,
    ));

    if iniciar_depois {
        spawn_botao_tutorial(
            commands,
            &fonte,
            "Iniciar Jogo",
            Vec3::new(180.0, -310.0, 5.0),
            Color::srgba(0.2, 0.5, 0.2, 0.85),
            TutorialAcao::IniciarJogo,
        );
    }

    spawn_botao_tutorial(
        commands,
        &fonte,
        "Voltar ao Menu",
        Vec3::new(-180.0, -310.0, 5.0),
        Color::srgba(0.6, 0.2, 0.2, 0.85),
        TutorialAcao::VoltarMenu,
    );
}

fn spawn_botao_tutorial(
    commands: &mut Commands,
    fonte: &Handle<Font>,
    texto: &str,
    pos: Vec3,
    cor: Color,
    acao: TutorialAcao,
) {
    commands.spawn((
        Sprite::from_color(cor, Vec2::new(300.0, 56.0)),
        Transform::from_translation(pos),
        TutorialUI,
        TutorialBotao {
            acao,
            largura: 300.0,
            altura: 56.0,
        },
    ));
    commands.spawn((
        Text2d::new(texto),
        TextFont {
            font: fonte.clone(),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(pos.x, pos.y, pos.z + 1.0),
        TutorialUI,
    ));
}

pub fn menu_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut app_exit: MessageWriter<AppExit>,
    mut tela_atual: ResMut<TelaAtual>,
    q_menu: Query<Entity, With<MenuUI>>,
    asset_server: Res<AssetServer>,
) {
    if *tela_atual != TelaAtual::Menu {
        return;
    }
    if keys.just_pressed(KeyCode::Digit1) {
        executar_acao_menu(
            MenuAcao::Comecar,
            &mut commands,
            &mut app_exit,
            &mut tela_atual,
            &q_menu,
            &asset_server,
        );
    } else if keys.just_pressed(KeyCode::Digit2) {
        executar_acao_menu(
            MenuAcao::Tutorial,
            &mut commands,
            &mut app_exit,
            &mut tela_atual,
            &q_menu,
            &asset_server,
        );
    } else if keys.just_pressed(KeyCode::Digit3) || keys.just_pressed(KeyCode::Escape) {
        executar_acao_menu(
            MenuAcao::Sair,
            &mut commands,
            &mut app_exit,
            &mut tela_atual,
            &q_menu,
            &asset_server,
        );
    }
}

pub fn menu_mouse_click(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_botoes: Query<(&Transform, &MenuBotao), With<MenuUI>>,
    mut app_exit: MessageWriter<AppExit>,
    mut tela_atual: ResMut<TelaAtual>,
    q_menu: Query<Entity, With<MenuUI>>,
    asset_server: Res<AssetServer>,
) {
    if *tela_atual != TelaAtual::Menu || !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Ok(window) = q_windows.single() else { return; };
    let Ok((camera, camera_transform)) = q_camera.single() else { return; };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            for (transform, botao) in q_botoes.iter() {
                let pos = transform.translation;
                let hw = botao.largura * 0.5;
                let hh = botao.altura * 0.5;
                if world_pos.x > pos.x - hw
                    && world_pos.x < pos.x + hw
                    && world_pos.y > pos.y - hh
                    && world_pos.y < pos.y + hh
                {
                    executar_acao_menu(
                        botao.acao,
                        &mut commands,
                        &mut app_exit,
                        &mut tela_atual,
                        &q_menu,
                        &asset_server,
                    );
                    break;
                }
            }
        }
    }
}

fn executar_acao_menu(
    acao: MenuAcao,
    commands: &mut Commands,
    app_exit: &mut MessageWriter<AppExit>,
    tela_atual: &mut ResMut<TelaAtual>,
    q_menu: &Query<Entity, With<MenuUI>>,
    asset_server: &Res<AssetServer>,
) {
    match acao {
        MenuAcao::Comecar => {
            for entity in q_menu.iter() {
                commands.entity(entity).despawn();
            }
            // Nao despawn o background, ele continua
            **tela_atual = TelaAtual::TutorialInicio;
            spawn_tutorial(commands, asset_server, true);
        }
        MenuAcao::Tutorial => {
            for entity in q_menu.iter() {
                commands.entity(entity).despawn();
            }
            // Nao despawn o background
            **tela_atual = TelaAtual::TutorialLivre;
            spawn_tutorial(commands, asset_server, false);
        }
        MenuAcao::Sair => {
            app_exit.write(AppExit::Success);
        }
    }
}

pub fn tutorial_input(
    _commands: Commands,
    _keys: Res<ButtonInput<KeyCode>>,
    _tela_atual: ResMut<TelaAtual>,
    _estado_jogo: ResMut<EstadoJogo>,
    _banco_perguntas: Res<BancoPerguntas>,
    _asset_server: Res<AssetServer>,
    _q_tutorial: Query<Entity, With<TutorialUI>>,
) {
    // Tutorial agora usa apenas botoes clicaveis (point-click).
}

pub fn tutorial_mouse_click(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_botoes: Query<(&Transform, &TutorialBotao), With<TutorialUI>>,
    mut tela_atual: ResMut<TelaAtual>,
    mut estado_jogo: ResMut<EstadoJogo>,
    banco_perguntas: Res<BancoPerguntas>,
    asset_server: Res<AssetServer>,
    q_tutorial: Query<Entity, With<TutorialUI>>,
    q_background: Query<Entity, With<Background>>,
) {
    if (*tela_atual != TelaAtual::TutorialInicio && *tela_atual != TelaAtual::TutorialLivre)
        || !buttons.just_pressed(MouseButton::Left)
    {
        return;
    }

    let Ok(window) = q_windows.single() else { return; };
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            for (transform, botao) in q_botoes.iter() {
                let pos = transform.translation;
                let hw = botao.largura * 0.5;
                let hh = botao.altura * 0.5;
                if world_pos.x > pos.x - hw
                    && world_pos.x < pos.x + hw
                    && world_pos.y > pos.y - hh
                    && world_pos.y < pos.y + hh
                {
                    for entity in q_tutorial.iter() {
                        commands.entity(entity).despawn();
                    }
                    match botao.acao {
                        TutorialAcao::VoltarMenu => {
                            *tela_atual = TelaAtual::Menu;
                            // Despawna o background do tutorial/jogo se existir
                            for entity in q_background.iter() {
                                commands.entity(entity).despawn();
                            }
                            spawn_menu(&mut commands, &asset_server);
                        }
                        TutorialAcao::IniciarJogo => {
                            if *tela_atual == TelaAtual::TutorialInicio {
                                *estado_jogo = EstadoJogo::default();
                                *tela_atual = TelaAtual::Jogo;
                                // Despawna o background do menu
                                for entity in q_background.iter() {
                                    commands.entity(entity).despawn();
                                }
                                iniciar_jogo(
                                    &mut commands,
                                    &asset_server,
                                    &banco_perguntas,
                                    &estado_jogo,
                                );
                            } else {
                                *tela_atual = TelaAtual::Menu;
                                // Despawna o background do tutorial
                                for entity in q_background.iter() {
                                    commands.entity(entity).despawn();
                                }
                                spawn_menu(&mut commands, &asset_server);
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
}

pub fn game_over_mouse_click(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_botoes: Query<(&Transform, &GameOverBotao), With<GameOverTela>>,
    mut tela_atual: ResMut<TelaAtual>,
    mut estado_jogo: ResMut<EstadoJogo>,
    _banco_perguntas: Res<BancoPerguntas>,
    asset_server: Res<AssetServer>,
    q_game_over: Query<Entity, With<GameOverTela>>,
    q_background: Query<Entity, With<Background>>,
) {
    if *tela_atual != TelaAtual::Jogo || !estado_jogo.game_over || !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = q_windows.single() else { return; };
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            for (transform, botao) in q_botoes.iter() {
                let pos = transform.translation;
                let hw = botao.largura * 0.5;
                let hh = botao.altura * 0.5;
                if world_pos.x > pos.x - hw
                    && world_pos.x < pos.x + hw
                    && world_pos.y > pos.y - hh
                    && world_pos.y < pos.y + hh
                {
                    // Despawn de tudo do game over e background
                    for entity in q_game_over.iter() {
                        commands.entity(entity).despawn();
                    }
                    for entity in q_background.iter() {
                        commands.entity(entity).despawn();
                    }
                    match botao.acao {
                        GameOverAcao::VoltarMenu => {
                            *tela_atual = TelaAtual::Menu;
                            *estado_jogo = EstadoJogo::default();
                            spawn_menu(&mut commands, &asset_server);
                        }
                    }
                    break;
                }
            }
        }
    }
}