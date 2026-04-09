use bevy::prelude::*;

use crate::components::*;
use crate::questions::{ordem_opcoes_para_pergunta, BancoPerguntas};
use crate::state::{EstadoJogo, TelaAtual};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    crate::screens::spawn_menu(&mut commands, &asset_server);
}

pub fn iniciar_jogo(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    banco_perguntas: &Res<BancoPerguntas>,
    estado_jogo: &EstadoJogo,
) {
    commands.spawn((
        Sprite::from_image(asset_server.load("background.png")),
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    let fonte_matematica = asset_server.load("FiraSans-Bold.ttf");
    let primeira_pergunta = &banco_perguntas.itens[estado_jogo.pergunta_atual];

    commands.spawn((
        Text2d::new(primeira_pergunta.enunciado),
        TextFont {
            font: fonte_matematica.clone(),
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 200.0, 1.0),
        Enunciado,
    ));

    commands.spawn((
        Text2d::new(""),
        TextFont {
            font: fonte_matematica.clone(),
            font_size: 50.0,
            ..default()
        },
        TextColor(Color::BLACK),
        Transform::from_xyz(0.0, 80.0, 1.0),
        TextoDestaqueMesa,
    ));

    commands.spawn((
        Text2d::new("Passe o mouse sobre as cartas e clique para selecionar."),
        TextFont {
            font: fonte_matematica.clone(),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.0)),
        Transform::from_xyz(0.0, -250.0, 1.0),
        FeedbackTexto,
    ));

    commands.spawn((
        Text::new("Vidas: 3"),
        TextFont {
            font: fonte_matematica.clone(),
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.2, 0.2)),
        Node {
            position_type: PositionType::Absolute,
            left: px(20.0),
            top: px(16.0),
            ..default()
        },
        VidaTexto,
    ));

    commands.spawn((
        Text::new("Tempo: 03:00"),
        TextFont {
            font: fonte_matematica,
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            right: px(20.0),
            top: px(16.0),
            ..default()
        },
        TempoTexto,
    ));

    let ordem = ordem_opcoes_para_pergunta(estado_jogo.pergunta_atual);
    for i in 0..4 {
        let pos_x = (i as f32 - 1.5) * 200.0;
        let (texto, correta) = primeira_pergunta.opcoes[ordem[i]];
        commands.spawn((
            Sprite::from_image(asset_server.load("carta.png")),
            Transform::from_xyz(pos_x, -100.0, 1.0),
            CartaIndice(i),
            CartaResposta {
                texto: texto.to_string(),
                correta,
            },
        ));
    }
}

pub fn handle_mouse_hover(
    estado_jogo: Res<EstadoJogo>,
    tela_atual: Res<TelaAtual>,
    q_windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_cartas: Query<(&CartaResposta, &Transform)>,
    mut q_destaque: Query<&mut Text2d, With<TextoDestaqueMesa>>,
) {
    if *tela_atual != TelaAtual::Jogo {
        return;
    }
    if estado_jogo.game_over {
        if let Ok(mut texto_ui) = q_destaque.single_mut() {
            texto_ui.0 = String::new();
        }
        return;
    }

    let Ok(window) = q_windows.single() else { return; };
    let Ok((camera, camera_transform)) = q_camera.single() else { return; };

    let mut texto_para_exibir = "";
    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            for (carta, transform) in q_cartas.iter() {
                let pos = transform.translation;
                if world_pos.x > pos.x - 50.0
                    && world_pos.x < pos.x + 50.0
                    && world_pos.y > pos.y - 75.0
                    && world_pos.y < pos.y + 75.0
                {
                    texto_para_exibir = &carta.texto;
                    break;
                }
            }
        }
    }
    if let Ok(mut texto_ui) = q_destaque.single_mut() {
        texto_ui.0 = texto_para_exibir.to_string();
    }
}

pub fn handle_mouse_clicks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    banco_perguntas: Res<BancoPerguntas>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut estado_jogo: ResMut<EstadoJogo>,
    tela_atual: Res<TelaAtual>,
    q_windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_cartas: Query<(&CartaResposta, &Transform)>,
    mut q_feedback: Query<&mut Text2d, With<FeedbackTexto>>,
) {
    if *tela_atual != TelaAtual::Jogo
        || estado_jogo.game_over
        || estado_jogo.proxima_pergunta_em > 0.0
        || !buttons.just_pressed(MouseButton::Left)
    {
        return;
    }
    let Ok(window) = q_windows.single() else { return; };
    let Ok((camera, camera_transform)) = q_camera.single() else { return; };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            for (carta, transform) in q_cartas.iter() {
                let pos = transform.translation;
                if world_pos.x > pos.x - 50.0
                    && world_pos.x < pos.x + 50.0
                    && world_pos.y > pos.y - 75.0
                    && world_pos.y < pos.y + 75.0
                {
                    let explicacao = banco_perguntas.itens[estado_jogo.pergunta_atual].explicacao;
                    if let Ok(mut texto_feedback) = q_feedback.single_mut() {
                        if carta.correta {
                            estado_jogo.acertos += 1;
                            texto_feedback.0 = format!(
                                "ACERTOU! {}\n{}\nProxima pergunta em 1s...",
                                carta.texto, explicacao
                            );
                        } else {
                            estado_jogo.erros += 1;
                            estado_jogo.vidas -= 1;
                            if estado_jogo.vidas <= 0 {
                                estado_jogo.game_over = true;
                                texto_feedback.0 = format!(
                                    "GAME OVER!\nAcertos: {} | Erros: {}",
                                    estado_jogo.acertos, estado_jogo.erros
                                );
                                spawn_game_over_tela(&mut commands, &asset_server);
                            } else {
                                texto_feedback.0 = format!(
                                    "ERROU! Voce selecionou {}\n{}\nVidas restantes: {}\nProxima pergunta em 1s...",
                                    carta.texto, explicacao, estado_jogo.vidas.max(0)
                                );
                            }
                        }
                    }
                    if !estado_jogo.game_over {
                        estado_jogo.proxima_pergunta_em = 1.0;
                    }
                    break;
                }
            }
        }
    }
}

pub fn processar_proxima_pergunta(
    mut estado_jogo: ResMut<EstadoJogo>,
    tela_atual: Res<TelaAtual>,
    banco_perguntas: Res<BancoPerguntas>,
    mut q_enunciado: Query<&mut Text2d, (With<Enunciado>, Without<FeedbackTexto>)>,
    mut q_cartas: Query<(&CartaIndice, &mut CartaResposta)>,
    mut q_feedback: Query<&mut Text2d, (With<FeedbackTexto>, Without<Enunciado>)>,
) {
    if *tela_atual != TelaAtual::Jogo || estado_jogo.game_over || estado_jogo.proxima_pergunta_em > 0.0
    {
        return;
    }
    if estado_jogo.pergunta_atual == 0 {
        return;
    }
    if estado_jogo.pergunta_atual >= banco_perguntas.itens.len() {
        estado_jogo.game_over = true;
        if let Ok(mut feedback) = q_feedback.single_mut() {
            let total = (estado_jogo.acertos + estado_jogo.erros).max(1);
            let aproveitamento = (estado_jogo.acertos as f32 / total as f32) * 100.0;
            feedback.0 = format!(
                "PARABENS! Voce concluiu as 20 questoes.\nAcertos: {} | Erros: {} | Aproveitamento: {:.0}%",
                estado_jogo.acertos, estado_jogo.erros, aproveitamento
            );
        }
        return;
    }
    aplicar_pergunta_atual(estado_jogo.pergunta_atual, &banco_perguntas, &mut q_enunciado, &mut q_cartas);
}

pub fn update_timer(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut estado_jogo: ResMut<EstadoJogo>,
    tela_atual: Res<TelaAtual>,
    banco_perguntas: Res<BancoPerguntas>,
    mut q_enunciado: Query<&mut Text2d, (With<Enunciado>, Without<FeedbackTexto>)>,
    mut q_cartas: Query<(&CartaIndice, &mut CartaResposta)>,
    mut q_feedback: Query<&mut Text2d, (With<FeedbackTexto>, Without<Enunciado>)>,
) {
    if *tela_atual != TelaAtual::Jogo || estado_jogo.game_over {
        return;
    }
    if estado_jogo.tempo_restante > 0.0 {
        estado_jogo.tempo_restante -= time.delta_secs();
        if estado_jogo.tempo_restante < 0.0 {
            estado_jogo.tempo_restante = 0.0;
        }
    }
    if estado_jogo.tempo_restante <= 0.0 {
        estado_jogo.game_over = true;
        if let Ok(mut feedback) = q_feedback.single_mut() {
            feedback.0 = format!(
                "TEMPO ESGOTADO!\nAcertos: {} | Erros: {}",
                estado_jogo.acertos, estado_jogo.erros
            );
        }
        spawn_game_over_tela(&mut commands, &asset_server);
        return;
    }
    if estado_jogo.proxima_pergunta_em > 0.0 {
        estado_jogo.proxima_pergunta_em -= time.delta_secs();
        if estado_jogo.proxima_pergunta_em <= 0.0 {
            estado_jogo.pergunta_atual += 1;
            if estado_jogo.pergunta_atual < banco_perguntas.itens.len() {
                aplicar_pergunta_atual(estado_jogo.pergunta_atual, &banco_perguntas, &mut q_enunciado, &mut q_cartas);
            }
        }
    }
}

pub fn update_hud(
    estado_jogo: Res<EstadoJogo>,
    tela_atual: Res<TelaAtual>,
    mut q_vida: Query<&mut Text, (With<VidaTexto>, Without<TempoTexto>)>,
    mut q_tempo: Query<&mut Text, (With<TempoTexto>, Without<VidaTexto>)>,
) {
    if *tela_atual != TelaAtual::Jogo {
        return;
    }
    if let Ok(mut vida_texto) = q_vida.single_mut() {
        vida_texto.0 = format!("Vidas: {}", estado_jogo.vidas.max(0));
    }
    let total = estado_jogo.tempo_restante.max(0.0) as i32;
    if let Ok(mut tempo_texto) = q_tempo.single_mut() {
        tempo_texto.0 = format!("Tempo: {:02}:{:02}", total / 60, total % 60);
    }
}

fn aplicar_pergunta_atual(
    pergunta_atual: usize,
    banco_perguntas: &Res<BancoPerguntas>,
    q_enunciado: &mut Query<&mut Text2d, (With<Enunciado>, Without<FeedbackTexto>)>,
    q_cartas: &mut Query<(&CartaIndice, &mut CartaResposta)>,
) {
    let pergunta = &banco_perguntas.itens[pergunta_atual];
    let ordem = ordem_opcoes_para_pergunta(pergunta_atual);
    if let Ok(mut enunciado) = q_enunciado.single_mut() {
        enunciado.0 = pergunta.enunciado.to_string();
    }
    for (indice, mut carta) in q_cartas.iter_mut() {
        let (texto, correta) = pergunta.opcoes[ordem[indice.0]];
        carta.texto = texto.to_string();
        carta.correta = correta;
    }
}

fn spawn_game_over_tela(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Spawn do menu com botao (sem imagem do casseb, usa background.png do jogo)
    crate::screens::spawn_game_over_menu(commands, asset_server);
}
