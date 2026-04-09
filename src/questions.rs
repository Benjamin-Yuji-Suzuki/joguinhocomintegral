use bevy::prelude::*;

#[derive(Debug)]
pub struct Pergunta {
    pub enunciado_img: String,
    pub opcoes: [(String, bool); 4],
    pub explicacao: &'static str,
}

#[derive(Resource)]
pub struct BancoPerguntas {
    pub itens: Vec<Pergunta>,
}

const PERMUTACOES_OPCOES: [[usize; 4]; 24] = [
    [0, 1, 2, 3], [0, 1, 3, 2], [0, 2, 1, 3], [0, 2, 3, 1],
    [0, 3, 1, 2], [0, 3, 2, 1], [1, 0, 2, 3], [1, 0, 3, 2],
    [1, 2, 0, 3], [1, 2, 3, 0], [1, 3, 0, 2], [1, 3, 2, 0],
    [2, 0, 1, 3], [2, 0, 3, 1], [2, 1, 0, 3], [2, 1, 3, 0],
    [2, 3, 0, 1], [2, 3, 1, 0], [3, 0, 1, 2], [3, 0, 2, 1],
    [3, 1, 0, 2], [3, 1, 2, 0], [3, 2, 0, 1], [3, 2, 1, 0],
];

pub fn ordem_opcoes_para_pergunta(pergunta_atual: usize) -> [usize; 4] {
    PERMUTACOES_OPCOES[(pergunta_atual * 7 + 3) % PERMUTACOES_OPCOES.len()]
}

impl Default for BancoPerguntas {
    fn default() -> Self {
        let explicacoes = [
            "Regra da potencia: ∫2x dx = x² + C.",
            "Integral de constante k: ∫k dx = kx + C.",
            "Use a potencia n=1: x^(1+1)/(1+1) = x²/2.",
            "Regra da potencia: ∫x² dx = x³/3 + C.",
            "4 * (x^4/4) = x^4. Coeficiente simplifica.",
            "Integre termo a termo: ∫2x dx + ∫5 dx.",
            "6x² -> 2x³ e -2 -> -2x.",
            "Para x, vira x²/2; para 3x², vira x³.",
            "7*(x⁷/7) = x⁷.",
            "Integre cada potencia separadamente.",
            "Constante sempre gera termo linear em x.",
            "5 sai em evidencia: 5*∫x dx = 5*(x²/2).",
            "Polinomio: integre termo a termo e some C.",
            "8*(x⁸/8) = x⁸.",
            "x⁴ vira x⁵/5; constante 2 vira 2x.",
            "Atencao no termo -4x: integral e -2x².",
            "(1/2)∫x dx = (1/2)(x²/2) = x²/4.",
            "Coeficiente simplifica de novo: 10*(x¹⁰/10).",
            "2x³ -> x⁴/2 e 6x -> 3x².",
            "Questao completa de polinomio simples.",
        ];

        let mut itens = Vec::new();
        // Gerador Automático apontando para as subpastas
        for i in 1..=20 {
            itens.push(Pergunta {
                enunciado_img: format!("questao/q{}.png", i),
                opcoes: [
                    (format!("cartas/q{}_opA.png", i), true),
                    (format!("cartas/q{}_opB.png", i), false),
                    (format!("cartas/q{}_opC.png", i), false),
                    (format!("cartas/q{}_opD.png", i), false),
                ],
                explicacao: explicacoes[i - 1],
            });
        }

        Self { itens }
    }
}