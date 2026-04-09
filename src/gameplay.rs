use bevy::prelude::*;

use crate::components::*;
use crate::questions::{ordem_opcoes_para_pergunta, BancoPerguntas};
use crate::state::{EstadoJogo, TelaAtual};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    crate::screens::spawn_menu(&mut commands, &asset_server);
    
    let musica: Handle<AudioSource> = asset_server.load("musica.ogg");
    commands.spawn((
        AudioPlayer(musica),
        PlaybackSettings::LOOP,
    ));
}

pub fn iniciar_jogo(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    banco_perguntas: &Res<BancoPerguntas>,
    estado_jogo: &EstadoJogo,
) {
    commands.spawn((
        Sprite::from_image(asset_server.load("background.png")),
        Transform::from_xyz(0.0, 0.0, -2.0),
        Background,
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("mesa.png")),
        Transform::from_xyz(0.0, 80.0, -1.0).with_scale(Vec3::new(1.25, 1.3, 1.0)),
        Mesa,
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("npc.png")),
        Transform::from_xyz(0.0, 250.0, -1.5).with_scale(Vec3::new(0.9, 0.9, 1.0)),
        Npc,
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("deck_de_cartas.png")),
        Transform::from_xyz(-550.0, -250.0, 2.0),
        DeckCartas,
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("deck_cola.png")),
        Transform::from_xyz(550.0, -250.0, 2.0).with_scale(Vec3::new(0.8, 0.8, 1.0)),
        DeckCola,
    ));

    let fonte_matematica = asset_server.load("FiraSans-Bold.ttf");
    let primeira_pergunta = &banco_perguntas.itens[estado_jogo.pergunta_atual];

    commands.spawn((
        Sprite::from_image(asset_server.load(&primeira_pergunta.enunciado_img)),
        Transform::from_xyz(0.0, 140.0, 1.0).with_scale(Vec3::new(0.65, 0.65, 1.0)),
        Enunciado,
    ));

    commands.spawn((
        Sprite::default(),
        Transform::from_xyz(0.0, 50.0, 1.0).with_scale(Vec3::new(0.65, 0.65, 1.0)),
        Visibility::Hidden,
        DestaqueMesaImg,
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
        let (img_path, correta) = &primeira_pergunta.opcoes[ordem[i]];
        commands.spawn((
            Sprite::from_image(asset_server.load("carta.png")),
            Transform::from_xyz(pos_x, -100.0, 1.0).with_scale(Vec3::new(0.65, 0.65, 1.0)),
            CartaIndice(i),
            CartaResposta {
                img_path: img_path.clone(),
                correta: *correta,
            },
        ));
    }

    spawn_popup_cola(commands, &asset_server, banco_perguntas, estado_jogo.pergunta_atual);
}

pub fn handle_mouse_hover(
    estado_jogo: Res<EstadoJogo>,
    tela_atual: Res<TelaAtual>,
    q_windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_cartas: Query<(&CartaResposta, &Transform)>,
    mut q_destaque: Query<(&mut Sprite, &mut Visibility), With<DestaqueMesaImg>>,
    asset_server: Res<AssetServer>,
) {
    if *tela_atual != TelaAtual::Jogo {
        return;
    }
    if estado_jogo.game_over {
        if let Ok((_, mut vis)) = q_destaque.single_mut() {
            *vis = Visibility::Hidden;
        }
        return;
    }

    let Ok(window) = q_windows.single() else { return; };
    let Ok((camera, camera_transform)) = q_camera.single() else { return; };

    let mut hover_img_path = None;
    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            for (carta, transform) in q_cartas.iter() {
                let pos = transform.translation;
                if world_pos.x > pos.x - 50.0
                    && world_pos.x < pos.x + 50.0
                    && world_pos.y > pos.y - 75.0
                    && world_pos.y < pos.y + 75.0
                {
                    hover_img_path = Some(carta.img_path.clone());
                    break;
                }
            }
        }
    }
    
    if let Ok((mut sprite, mut vis)) = q_destaque.single_mut() {
        if let Some(path) = hover_img_path {
            sprite.image = asset_server.load(path);
            *vis = Visibility::Visible;
        } else {
            *vis = Visibility::Hidden;
        }
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
    q_deck: Query<&Transform, With<DeckCartas>>,
    q_cola: Query<&Transform, With<DeckCola>>,
    mut q_popup: Query<&mut Visibility, With<PopUpCola>>, // Pega as visibilidades do PopUp
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
            
            if let Ok(deck_transform) = q_deck.single() {
                let deck_pos = deck_transform.translation;
                if world_pos.x > deck_pos.x - 60.0
                    && world_pos.x < deck_pos.x + 60.0
                    && world_pos.y > deck_pos.y - 80.0
                    && world_pos.y < deck_pos.y + 80.0
                {
                    if let Ok(mut texto_feedback) = q_feedback.single_mut() {
                        texto_feedback.0 = "Pergunta pulada! Proxima em 1s...".to_string();
                    }
                    estado_jogo.erros += 1;
                    estado_jogo.proxima_pergunta_em = 1.0;
                    return;
                }
            }

            // Lógica de clique no Papel Cola
            if let Ok(cola_transform) = q_cola.single() {
                let cola_pos = cola_transform.translation;
                if world_pos.x > cola_pos.x - 60.0
                    && world_pos.x < cola_pos.x + 60.0
                    && world_pos.y > cola_pos.y - 80.0
                    && world_pos.y < cola_pos.y + 80.0
                {
                    // Alterna a visibilidade do PopUp sem precisar recriar
                    for mut popup_vis in q_popup.iter_mut() {
                        if *popup_vis == Visibility::Hidden {
                            *popup_vis = Visibility::Visible;
                        } else {
                            *popup_vis = Visibility::Hidden;
                        }
                    }
                    return;
                }
            }

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
                                "ACERTOU!\n{}\nProxima pergunta em 1s...", explicacao
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
                                    "ERROU!\n{}\nVidas restantes: {}\nProxima pergunta em 1s...",
                                    explicacao, estado_jogo.vidas.max(0)
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
    mut commands: Commands,
    mut estado_jogo: ResMut<EstadoJogo>,
    tela_atual: Res<TelaAtual>,
    banco_perguntas: Res<BancoPerguntas>,
    mut q_enunciado: Query<&mut Sprite, (With<Enunciado>, Without<FeedbackTexto>)>,
    mut q_cartas: Query<(&CartaIndice, &mut CartaResposta)>,
    mut q_feedback: Query<&mut Text2d, (With<FeedbackTexto>, Without<Enunciado>)>,
    asset_server: Res<AssetServer>,
    q_popup_entities: Query<Entity, With<PopUpCola>>, // Busca as entidades do PopUp antigo
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
    aplicar_pergunta_atual(estado_jogo.pergunta_atual, &banco_perguntas, &mut q_enunciado, &mut q_cartas, &asset_server);
    
    // Despawna as entidades do PopUp antigo e cria o novo com o texto atualizado
    for entity in q_popup_entities.iter() {
        commands.entity(entity).despawn();
    }
    spawn_popup_cola(&mut commands, &asset_server, &banco_perguntas, estado_jogo.pergunta_atual);
}

pub fn update_timer(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut estado_jogo: ResMut<EstadoJogo>,
    tela_atual: Res<TelaAtual>,
    banco_perguntas: Res<BancoPerguntas>,
    mut q_enunciado: Query<&mut Sprite, (With<Enunciado>, Without<FeedbackTexto>)>,
    mut q_cartas: Query<(&CartaIndice, &mut CartaResposta)>,
    mut q_feedback: Query<&mut Text2d, (With<FeedbackTexto>, Without<Enunciado>)>,
    q_popup_entities: Query<Entity, With<PopUpCola>>, // Busca as entidades do PopUp antigo
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
                aplicar_pergunta_atual(estado_jogo.pergunta_atual, &banco_perguntas, &mut q_enunciado, &mut q_cartas, &asset_server);
                
                // Despawna as entidades do PopUp antigo e cria o novo com o texto atualizado
                for entity in q_popup_entities.iter() {
                    commands.entity(entity).despawn();
                }
                spawn_popup_cola(&mut commands, &asset_server, &banco_perguntas, estado_jogo.pergunta_atual);
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
    q_enunciado: &mut Query<&mut Sprite, (With<Enunciado>, Without<FeedbackTexto>)>,
    q_cartas: &mut Query<(&CartaIndice, &mut CartaResposta)>,
    asset_server: &Res<AssetServer>,
) {
    let pergunta = &banco_perguntas.itens[pergunta_atual];
    let ordem = ordem_opcoes_para_pergunta(pergunta_atual);
    
    if let Ok(mut enunciado_sprite) = q_enunciado.single_mut() {
        enunciado_sprite.image = asset_server.load(&pergunta.enunciado_img);
    }
    
    for (indice, mut carta) in q_cartas.iter_mut() {
        let (path, correta) = &pergunta.opcoes[ordem[indice.0]];
        carta.img_path = path.clone();
        carta.correta = *correta;
    }
}

fn spawn_popup_cola(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    banco_perguntas: &Res<BancoPerguntas>,
    pergunta_atual: usize,
) {
    let fonte_matematica = asset_server.load("FiraSans-Bold.ttf");
    let explicacao = banco_perguntas.itens[pergunta_atual].explicacao;

    let didatica = format!(
        "GUIA DIDATICO: INTEGRAIS\n\n1) Regra Principal: \n∫ x^n dx = (x^(n+1))/(n+1) + C \n\n2) No nosso caso, ∫ 2x dx: \n  2x = 2 * x^1 \n  n = 1 \n\n3) Aplicando a Regra: \n  x^(1+1)/(1+1) = x^2/2 \n\n4) Finalizando: \n  2 * (x^2 / 2) + C = x^2 + C \n\nExplicacao do Banco: \n'{}' \n\nReferência: Tabela Integrais 123 PDF.", explicacao
    );

    commands.spawn((
        Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.95), Vec2::new(1000.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
        PopUpCola,
        Visibility::Hidden, 
    ));

    commands.spawn((
        Text2d::new(didatica),
        TextFont {
            font: fonte_matematica.clone(),
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 11.0),
        PopUpCola,
        Visibility::Hidden,
    ));
}

fn spawn_game_over_tela(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    crate::screens::spawn_game_over_menu(commands, asset_server);
}