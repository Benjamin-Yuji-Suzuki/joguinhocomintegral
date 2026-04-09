use bevy::prelude::*;
use bevy::ecs::relationship::Relationship; 

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
    // BACKGROUND (Z-index movido para -5.0 para ficar sempre no fundo de tudo)
    commands.spawn((
        Sprite::from_image(asset_server.load("background.png")),
        Transform::from_xyz(0.0, 0.0, -5.0),
        Background,
        EntidadeJogo,
    ));

    // MESA NO ALTO
    commands.spawn((
        Sprite::from_image(asset_server.load("mesa.png")),
        Transform::from_xyz(0.0, 180.0, -1.0).with_scale(Vec3::new(1.25, 1.3, 1.0)),
        Mesa,
        EntidadeJogo,
    ));

    // NPC NO ALTO (Tamanho reduzido em ~15%)
    commands.spawn((
        Sprite::from_image(asset_server.load("npc.png")),
        Transform::from_xyz(0.0, 350.0, -1.5).with_scale(Vec3::new(0.76, 0.76, 1.0)),
        Npc,
        EntidadeJogo,
    ));

    // DECK DE CARTAS
    commands.spawn((
        Sprite::from_image(asset_server.load("deck_de_cartas.png")),
        Transform::from_xyz(-460.0, 100.0, 2.0),
        DeckCartas,
        EntidadeJogo,
    ));

    // DECK DA COLA
    commands.spawn((
        Sprite::from_image(asset_server.load("deck_cola.png")),
        Transform::from_xyz(550.0, 100.0, 2.0).with_scale(Vec3::new(0.8, 0.8, 1.0)),
        DeckCola,
        EntidadeJogo,
    ));

    let fonte_matematica = asset_server.load("FiraSans-Bold.ttf");
    let primeira_pergunta = &banco_perguntas.itens[estado_jogo.pergunta_atual];

    commands.spawn((
        Sprite::from_image(asset_server.load(&primeira_pergunta.enunciado_img)),
        Transform::from_xyz(0.0, 140.0, 1.0).with_scale(Vec3::new(0.65, 0.65, 1.0)),
        Enunciado,
        EntidadeJogo,
    ));

    commands.spawn((
        Sprite::default(),
        Transform::from_xyz(0.0, 50.0, 1.0).with_scale(Vec3::new(0.65, 0.65, 1.0)),
        Visibility::Hidden,
        DestaqueMesaImg,
    ));

    // FONTE AMARELA (Feedback)
    commands.spawn((
        Text2d::new("Passe o mouse sobre as cartas e clique para selecionar."),
        TextFont { font: fonte_matematica.clone(), font_size: 30.0, ..default() },
        TextColor(Color::srgb(1.0, 1.0, 0.0)),
        Transform::from_xyz(0.0, -250.0, 1.0),
        FeedbackTexto,
        EntidadeJogo,
    ));

    // EXIBIR TEMPO 
    commands.spawn((
        Text::new("Tempo: 03:00"),
        TextFont { font: fonte_matematica.clone(), font_size: 36.0, ..default() },
        TextColor(Color::WHITE),
        Node { 
            position_type: PositionType::Absolute, 
            right: px(20.0), 
            top: px(16.0), 
            padding: UiRect::all(Val::Px(5.0)),
            ..default() 
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)), 
        TempoTexto,
    ));

    // BOTÃO LEVANTAR
    let pos_btn_levantar = Vec3::new(-530.0, 300.0, 5.0);
    commands.spawn((
        Sprite::from_color(Color::srgba(0.8, 0.3, 0.2, 0.9), Vec2::new(140.0, 50.0)),
        Transform::from_translation(pos_btn_levantar),
        BotaoLevantar,
        EntidadeJogo,
    ));
    commands.spawn((
        Text2d::new("Levantar"),
        TextFont { font: fonte_matematica.clone(), font_size: 26.0, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(pos_btn_levantar.x, pos_btn_levantar.y, pos_btn_levantar.z + 1.0),
        EntidadeJogo,
    ));

    let ordem = ordem_opcoes_para_pergunta(estado_jogo.pergunta_atual);
    for i in 0..4 {
        let pos_x = (i as f32 - 1.5) * 200.0;
        let (img_path, correta) = &primeira_pergunta.opcoes[ordem[i]];
        commands.spawn((
            Sprite::from_image(asset_server.load("carta.png")),
            Transform::from_xyz(pos_x, -100.0, 1.0).with_scale(Vec3::new(0.65, 0.65, 1.0)),
            CartaIndice(i),
            CartaResposta { img_path: img_path.clone(), correta: *correta },
            EntidadeJogo,
        ));
    }

    spawn_popup_cola(commands, &asset_server);
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
    if *tela_atual != TelaAtual::Jogo { return; }
    if estado_jogo.game_over {
        if let Ok((_, mut vis)) = q_destaque.single_mut() { *vis = Visibility::Hidden; }
        return;
    }

    let Ok(window) = q_windows.single() else { return; };
    let Ok((camera, camera_transform)) = q_camera.single() else { return; };

    let mut hover_img_path = None;
    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            for (carta, transform) in q_cartas.iter() {
                let pos = transform.translation;
                if world_pos.x > pos.x - 50.0 && world_pos.x < pos.x + 50.0
                    && world_pos.y > pos.y - 75.0 && world_pos.y < pos.y + 75.0 {
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
    mut tela_atual: ResMut<TelaAtual>, 
    q_windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_cartas: Query<(&CartaResposta, &Transform)>,
    mut q_feedback: Query<&mut Text2d, With<FeedbackTexto>>,
    (q_deck, q_cola, q_botao_levantar): (
        Query<&Transform, With<DeckCartas>>,
        Query<&Transform, With<DeckCola>>,
        Query<&Transform, With<BotaoLevantar>>
    ),
    mut q_popup_base: Query<(&mut Visibility, Option<&Children>), (With<PopUpCola>, Without<BotaoFecharCola>)>,
    q_botao_fechar: Query<(&GlobalTransform, &ChildOf), With<BotaoFecharCola>>, 
    // Atualizado: agora ignora o Background na hora de esconder os elementos
    mut q_entidades_jogo: Query<&mut Visibility, (With<EntidadeJogo>, Without<PopUpCola>, Without<DestaqueMesaImg>, Without<Background>)>,
    mut q_destaque: Query<&mut Visibility, (With<DestaqueMesaImg>, Without<EntidadeJogo>, Without<PopUpCola>)>,
) {
    if *tela_atual != TelaAtual::Jogo || estado_jogo.game_over || estado_jogo.proxima_pergunta_em > 0.0 || !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    
    let Ok(window) = q_windows.single() else { return; };
    let Ok((camera, camera_transform)) = q_camera.single() else { return; };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            
            if let Ok(botao_transform) = q_botao_levantar.single() {
                let b_pos = botao_transform.translation;
                if world_pos.x > b_pos.x - 70.0 && world_pos.x < b_pos.x + 70.0
                    && world_pos.y > b_pos.y - 25.0 && world_pos.y < b_pos.y + 25.0 {
                    
                    *tela_atual = TelaAtual::Exploracao;

                    // Como colocamos o Without<Background> ali em cima, o background continua visível
                    for mut vis in q_entidades_jogo.iter_mut() { *vis = Visibility::Hidden; }
                    for (mut vis, _) in q_popup_base.iter_mut() { *vis = Visibility::Hidden; }
                    if let Ok(mut vis) = q_destaque.single_mut() { *vis = Visibility::Hidden; }

                    spawn_exploracao(&mut commands, &asset_server);
                    return;
                }
            }

            for (button_global_transform, parent) in q_botao_fechar.iter() {
                if let Ok((parent_vis, _)) = q_popup_base.get(parent.get()) {
                    if *parent_vis == Visibility::Visible {
                        let button_pos = button_global_transform.translation();
                        if world_pos.x > button_pos.x - 30.0 && world_pos.x < button_pos.x + 30.0
                            && world_pos.y > button_pos.y - 30.0 && world_pos.y < button_pos.y + 30.0 {
                            for (mut vis, _) in q_popup_base.iter_mut() {
                                *vis = Visibility::Hidden;
                            }
                            return; 
                        }
                    }
                }
            }

            if let Ok(deck_transform) = q_deck.single() {
                let deck_pos = deck_transform.translation;
                if world_pos.x > deck_pos.x - 60.0 && world_pos.x < deck_pos.x + 60.0
                    && world_pos.y > deck_pos.y - 80.0 && world_pos.y < deck_pos.y + 80.0 {
                    if let Ok(mut texto_feedback) = q_feedback.single_mut() {
                        texto_feedback.0 = "Pergunta pulada! Proxima em 1s...".to_string();
                    }
                    estado_jogo.erros += 1;
                    estado_jogo.proxima_pergunta_em = 1.0;
                    return;
                }
            }

            if let Ok(cola_transform) = q_cola.single() {
                let cola_pos = cola_transform.translation;
                if world_pos.x > cola_pos.x - 60.0 && world_pos.x < cola_pos.x + 60.0
                    && world_pos.y > cola_pos.y - 80.0 && world_pos.y < cola_pos.y + 80.0 {
                    for (mut vis, _) in q_popup_base.iter_mut() {
                        *vis = Visibility::Visible;
                    }
                    return;
                }
            }

            for (carta, transform) in q_cartas.iter() {
                let pos = transform.translation;
                if world_pos.x > pos.x - 50.0 && world_pos.x < pos.x + 50.0
                    && world_pos.y > pos.y - 75.0 && world_pos.y < pos.y + 75.0 {
                    let explicacao = banco_perguntas.itens[estado_jogo.pergunta_atual].explicacao;
                    if let Ok(mut texto_feedback) = q_feedback.single_mut() {
                        if carta.correta {
                            estado_jogo.acertos += 1;
                            texto_feedback.0 = format!("ACERTOU!\n{}\nProxima em 1s...", explicacao);
                        } else {
                            estado_jogo.erros += 1;
                            texto_feedback.0 = format!("ERROU!\n{}\nProxima em 1s...", explicacao);
                        }
                    }
                    estado_jogo.proxima_pergunta_em = 1.0;
                    break;
                }
            }
        }
    }
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
    q_entidades_exploracao: Query<Entity, With<EntidadeExploracao>>,
) {
    if (*tela_atual != TelaAtual::Jogo && *tela_atual != TelaAtual::Exploracao) || estado_jogo.game_over { return; }
    
    if estado_jogo.tempo_restante > 0.0 {
        estado_jogo.tempo_restante -= time.delta_secs();
        if estado_jogo.tempo_restante < 0.0 { estado_jogo.tempo_restante = 0.0; }
    }
    
    if estado_jogo.tempo_restante <= 0.0 {
        estado_jogo.game_over = true;
        if let Ok(mut feedback) = q_feedback.single_mut() {
            feedback.0 = format!("TEMPO ESGOTADO!\nAcertos: {} | Erros: {}", estado_jogo.acertos, estado_jogo.erros);
        }
        
        if *tela_atual == TelaAtual::Exploracao {
            for entity in q_entidades_exploracao.iter() {
                commands.entity(entity).despawn();
            }
        }

        spawn_game_over_tela(&mut commands, &asset_server);
        return;
    }
    
    if *tela_atual == TelaAtual::Jogo && estado_jogo.proxima_pergunta_em > 0.0 {
        estado_jogo.proxima_pergunta_em -= time.delta_secs();
        if estado_jogo.proxima_pergunta_em <= 0.0 {
            if let Ok(mut feedback) = q_feedback.single_mut() {
                feedback.0 = "".to_string();
            }

            estado_jogo.pergunta_atual += 1;
            if estado_jogo.pergunta_atual < banco_perguntas.itens.len() {
                aplicar_pergunta_atual(estado_jogo.pergunta_atual, &banco_perguntas, &mut q_enunciado, &mut q_cartas, &asset_server);
            } else {
                estado_jogo.game_over = true;
                if let Ok(mut feedback) = q_feedback.single_mut() {
                    feedback.0 = "FIM DE JOGO! Todas as questoes concluidas.".to_string();
                }
            }
        }
    }
}

pub fn update_hud(
    estado_jogo: Res<EstadoJogo>,
    tela_atual: Res<TelaAtual>,
    mut q_tempo: Query<&mut Text, With<TempoTexto>>,
) {
    if *tela_atual != TelaAtual::Jogo && *tela_atual != TelaAtual::Exploracao { return; }
    
    let total = estado_jogo.tempo_restante.max(0.0) as i32;
    if let Ok(mut tempo_texto) = q_tempo.single_mut() { 
        tempo_texto.0 = format!("Tempo: {:02}:{:02}", total / 60, total % 60); 
    }
}

pub fn spawn_exploracao(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let fonte = asset_server.load("FiraSans-Bold.ttf");

    // O mapa agora tem Z-index -3.0 para ficar na FRENTE do background.png (que está em -5.0)
    commands.spawn((
        Sprite::from_image(asset_server.load("mapa.png")),
        Transform::from_xyz(0.0, 0.0, -3.0), 
        EntidadeExploracao,
    ));

    commands.spawn((
        Transform::from_xyz(0.0, 230.0, 0.0),
        GlobalTransform::default(),
        EntidadeExploracao,
        MesaInteragivel, 
    ));

    // TEXTO DE INSTRUÇÃO (Cor branca e Y subindo de -300.0 para -260.0)
    commands.spawn((
        Text2d::new("Clique na mesa para sentar novamente"),
        TextFont { font: fonte, font_size: 26.0, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, -260.0, 5.0),
        EntidadeExploracao,
    ));
}

pub fn update_exploracao(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mut tela_atual: ResMut<TelaAtual>,
    q_windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_mesa: Query<&Transform, With<MesaInteragivel>>,
    // Atualizado: ignora o Background na hora de retornar para o modo Jogo
    mut q_entidades_jogo: Query<&mut Visibility, (With<EntidadeJogo>, Without<PopUpCola>, Without<DestaqueMesaImg>, Without<Background>)>,
    q_entidades_exploracao: Query<Entity, With<EntidadeExploracao>>,
) {
    if *tela_atual != TelaAtual::Exploracao || !buttons.just_pressed(MouseButton::Left) { 
        return; 
    }

    let Ok(window) = q_windows.single() else { return; };
    let Ok((camera, camera_transform)) = q_camera.single() else { return; };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            
            if let Ok(mesa_transform) = q_mesa.single() {
                let m_pos = mesa_transform.translation;
                
                if world_pos.x > m_pos.x - 280.0 && world_pos.x < m_pos.x + 280.0
                    && world_pos.y > m_pos.y - 100.0 && world_pos.y < m_pos.y + 100.0 {
                    *tela_atual = TelaAtual::Jogo; 
                    for entity in q_entidades_exploracao.iter() {
                        commands.entity(entity).despawn();
                    }
                    for mut vis in q_entidades_jogo.iter_mut() {
                        *vis = Visibility::Visible;
                    }
                }
            }
        }
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

fn spawn_popup_cola(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.95), Vec2::new(1000.0, 600.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
        PopUpCola,
        Visibility::Hidden, 
    ));

    let popup_image = commands.spawn((
        Sprite::from_image(asset_server.load("tutorialcola.png")),
        Transform::from_xyz(0.0, 0.0, 11.0),
        PopUpCola,
        Visibility::Hidden,
    )).id();

    let fechar_button = commands.spawn((
        Sprite::from_color(Color::NONE, Vec2::new(60.0, 60.0)),
        Transform::from_xyz(460.0, 400.0, 1.0),
        BotaoFecharCola,
    )).id();

    commands.entity(popup_image).add_children(&[fechar_button]);
}

fn spawn_game_over_tela(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    crate::screens::spawn_game_over_menu(commands, asset_server);
}