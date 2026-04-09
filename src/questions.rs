use bevy::prelude::*;

#[derive(Debug)]
pub struct Pergunta {
    pub enunciado: &'static str,
    pub opcoes: [(&'static str, bool); 4],
    pub explicacao: &'static str,
}

#[derive(Resource)]
pub struct BancoPerguntas {
    pub itens: Vec<Pergunta>,
}

const PERMUTACOES_OPCOES: [[usize; 4]; 24] = [
    [0, 1, 2, 3],
    [0, 1, 3, 2],
    [0, 2, 1, 3],
    [0, 2, 3, 1],
    [0, 3, 1, 2],
    [0, 3, 2, 1],
    [1, 0, 2, 3],
    [1, 0, 3, 2],
    [1, 2, 0, 3],
    [1, 2, 3, 0],
    [1, 3, 0, 2],
    [1, 3, 2, 0],
    [2, 0, 1, 3],
    [2, 0, 3, 1],
    [2, 1, 0, 3],
    [2, 1, 3, 0],
    [2, 3, 0, 1],
    [2, 3, 1, 0],
    [3, 0, 1, 2],
    [3, 0, 2, 1],
    [3, 1, 0, 2],
    [3, 1, 2, 0],
    [3, 2, 0, 1],
    [3, 2, 1, 0],
];

pub fn ordem_opcoes_para_pergunta(pergunta_atual: usize) -> [usize; 4] {
    PERMUTACOES_OPCOES[(pergunta_atual * 7 + 3) % PERMUTACOES_OPCOES.len()]
}

impl Default for BancoPerguntas {
    fn default() -> Self {
        Self {
            itens: vec![
                Pergunta {
                    enunciado: "Resolva a integral: ∫ 2x dx",
                    opcoes: [("x² + C", true), ("x²", false), ("2", false), ("2x² + C", false)],
                    explicacao: "Regra da potencia: ∫2x dx = x² + C.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ 3 dx",
                    opcoes: [("3x + C", true), ("3 + C", false), ("x³ + C", false), ("x + C", false)],
                    explicacao: "Integral de constante k: ∫k dx = kx + C.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ x dx",
                    opcoes: [("x²/2 + C", true), ("x² + C", false), ("1 + C", false), ("ln(x) + C", false)],
                    explicacao: "Use a potencia n=1: x^(1+1)/(1+1) = x²/2.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ x² dx",
                    opcoes: [("x³/3 + C", true), ("2x + C", false), ("x²/2 + C", false), ("x³ + C", false)],
                    explicacao: "Regra da potencia: ∫x² dx = x³/3 + C.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ 4x³ dx",
                    opcoes: [("x⁴ + C", true), ("4x⁴ + C", false), ("x³ + C", false), ("4x² + C", false)],
                    explicacao: "4 * (x^4/4) = x^4. Coeficiente simplifica.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ (2x + 5) dx",
                    opcoes: [("x² + 5x + C", true), ("2x² + 5 + C", false), ("x² + 5 + C", false), ("x² + 5x", false)],
                    explicacao: "Integre termo a termo: ∫2x dx + ∫5 dx.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ (6x² - 2) dx",
                    opcoes: [("2x³ - 2x + C", true), ("6x³ - 2 + C", false), ("2x² - 2x + C", false), ("2x³ - 2 + C", false)],
                    explicacao: "6x² -> 2x³ e -2 -> -2x.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ (3x² + x) dx",
                    opcoes: [("x³ + x²/2 + C", true), ("3x³ + x² + C", false), ("x³ + x + C", false), ("x³ + x² + C", false)],
                    explicacao: "Para x, vira x²/2; para 3x², vira x³.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ 7x⁶ dx",
                    opcoes: [("x⁷ + C", true), ("7x⁷ + C", false), ("x⁶ + C", false), ("x⁷/7 + C", false)],
                    explicacao: "7*(x⁷/7) = x⁷.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ (x³ - x) dx",
                    opcoes: [("x⁴/4 - x²/2 + C", true), ("x⁴ - x² + C", false), ("x² - x + C", false), ("x⁴/3 - x² + C", false)],
                    explicacao: "Integre cada potencia separadamente.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ 9 dx",
                    opcoes: [("9x + C", true), ("9 + C", false), ("x⁹ + C", false), ("x + C", false)],
                    explicacao: "Constante sempre gera termo linear em x.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ 5x dx",
                    opcoes: [("(5/2)x² + C", true), ("5x² + C", false), ("x² + C", false), ("(1/5)x² + C", false)],
                    explicacao: "5 sai em evidencia: 5*∫x dx = 5*(x²/2).",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ (2x² + 4x + 1) dx",
                    opcoes: [("(2/3)x³ + 2x² + x + C", true), ("2x³ + 4x² + x + C", false), ("(2/3)x³ + 2x + 1 + C", false), ("(2/3)x³ + 2x² + x", false)],
                    explicacao: "Polinomio: integre termo a termo e some C.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ (8x⁷) dx",
                    opcoes: [("x⁸ + C", true), ("8x⁸ + C", false), ("x⁷ + C", false), ("x⁸/8 + C", false)],
                    explicacao: "8*(x⁸/8) = x⁸.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ (x⁴ + 2) dx",
                    opcoes: [("x⁵/5 + 2x + C", true), ("x⁵ + 2 + C", false), ("x⁴/4 + 2x + C", false), ("x⁵/5 + 2 + C", false)],
                    explicacao: "x⁴ vira x⁵/5; constante 2 vira 2x.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ (3x² - 4x + 2) dx",
                    opcoes: [("x³ - 2x² + 2x + C", true), ("3x³ - 4x² + 2 + C", false), ("x³ - 4x² + 2x + C", false), ("x³ - 2x² + 2x", false)],
                    explicacao: "Atencao no termo -4x: integral e -2x².",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ (x/2) dx",
                    opcoes: [("x²/4 + C", true), ("x²/2 + C", false), ("x/2 + C", false), ("2x² + C", false)],
                    explicacao: "(1/2)∫x dx = (1/2)(x²/2) = x²/4.",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ (10x⁹) dx",
                    opcoes: [("x¹⁰ + C", true), ("10x¹⁰ + C", false), ("x⁹ + C", false), ("x¹⁰/10 + C", false)],
                    explicacao: "Coeficiente simplifica de novo: 10*(x¹⁰/10).",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ (2x³ + 6x) dx",
                    opcoes: [("x⁴/2 + 3x² + C", true), ("2x⁴ + 6x² + C", false), ("x⁴ + 3x + C", false), ("x⁴/2 + 3x²", false)],
                    explicacao: "2x³ -> x⁴/2 e 6x -> 3x².",
                },
                Pergunta {
                    enunciado: "Resolva a integral: ∫ (x² + x + 1) dx",
                    opcoes: [("x³/3 + x²/2 + x + C", true), ("x³ + x² + x + C", false), ("x³/3 + x²/2 + 1 + C", false), ("x³/3 + x²/2 + x", false)],
                    explicacao: "Questao completa de polinomio simples.",
                },
            ],
        }
    }
}
