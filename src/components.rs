use bevy::prelude::*;

#[derive(Component)]
pub struct Enunciado;

#[derive(Component, Debug, Clone)]
pub struct CartaResposta {
    pub texto: String,
    pub correta: bool,
}

#[derive(Component)]
pub struct CartaIndice(pub usize);

#[derive(Component)]
pub struct FeedbackTexto;

#[derive(Component)]
pub struct TextoDestaqueMesa;

#[derive(Component)]
pub struct VidaTexto;

#[derive(Component)]
pub struct TempoTexto;

#[derive(Component)]
pub struct GameOverTela;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct TutorialUI;

#[derive(Component, Clone, Copy)]
pub struct MenuBotao {
    pub acao: MenuAcao,
    pub largura: f32,
    pub altura: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuAcao {
    Comecar,
    Tutorial,
    Sair,
}

#[derive(Component, Clone, Copy)]
pub struct TutorialBotao {
    pub acao: TutorialAcao,
    pub largura: f32,
    pub altura: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TutorialAcao {
    IniciarJogo,
    VoltarMenu,
}
