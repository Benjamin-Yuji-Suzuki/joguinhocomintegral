use bevy::prelude::*;

#[derive(Component)]
pub struct Enunciado;

#[derive(Component, Debug, Clone)]
pub struct CartaResposta {
    pub img_path: String,
    pub correta: bool,
}

#[derive(Component)]
pub struct CartaIndice(pub usize);

#[derive(Component)]
pub struct FeedbackTexto;

#[derive(Component)]
pub struct DestaqueMesaImg;

#[derive(Component)]
pub struct PopUpCola;

#[derive(Component)]
pub struct VidaTexto;

#[derive(Component)]
pub struct TempoTexto;

#[derive(Component)]
pub struct GameOverTela;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Mesa;

#[derive(Component)]
pub struct Npc;

#[derive(Component)]
pub struct DeckCartas;

#[derive(Component)]
pub struct DeckCola;

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
    Voltar,
    IniciarJogo, // <- Resolvido!
    VoltarMenu,  // <- Resolvido!
}

#[derive(Component, Clone, Copy)]
pub struct GameOverBotao {
    pub acao: GameOverAcao,
    pub largura: f32,
    pub altura: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameOverAcao {
    VoltarMenu,
}

// --------------------------------------------------------
// NOVOS COMPONENTES (BOTÕES E EXPLORAÇÃO POINT-AND-CLICK)
// --------------------------------------------------------

#[derive(Component)]
pub struct BotaoFecharCola;

#[derive(Component)]
pub struct BotaoLevantar;

#[derive(Component)]
pub struct EntidadeJogo; 

#[derive(Component)]
pub struct EntidadeExploracao;

#[derive(Component)]
pub struct MesaInteragivel;