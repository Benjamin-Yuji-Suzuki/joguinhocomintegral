use bevy::prelude::*;

#[derive(Resource)]
pub struct EstadoJogo {
    pub vidas: i32,
    pub tempo_restante: f32,
    pub game_over: bool,
    pub pergunta_atual: usize,
    pub acertos: i32,
    pub erros: i32,
    pub proxima_pergunta_em: f32,
}

#[derive(Resource, Clone, Copy, PartialEq, Eq)]
pub enum TelaAtual {
    Menu,
    TutorialInicio,
    TutorialLivre,
    Jogo,
    Exploracao,
}

impl Default for EstadoJogo {
    fn default() -> Self {
        Self {
            vidas: 3,
            tempo_restante: 180.0,
            game_over: false,
            pergunta_atual: 0,
            acertos: 0,
            erros: 0,
            proxima_pergunta_em: 0.0,
        }
    }
}
