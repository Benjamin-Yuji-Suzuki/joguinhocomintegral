# Joguinho com Integral 🧮🎮

Um jogo educativo focado em resolver questões de matemática e cálculo (integrais), desenvolvido em **Rust** utilizando a engine **Bevy**. 

Neste jogo, o jogador assume o papel de um estudante em uma sala de aula desafiadora. O objetivo é resolver integrais corretamente enquanto gerencia o tempo e decide entre focar na prova ou explorar o ambiente.

## 👥 Créditos e Equipe

Este projeto foi desenvolvido em colaboração, unindo programação, arte e som:

* **Benjamin Yuji Suzuki:** Programação e Integração (Arquitetura do jogo, lógica de estado, sistemas ECS e integração geral).
* **Felipe de Freitas:** Arte e Assets (Criação do cenário, sprites da mesa, NPC, cartas e o mapa de exploração).
* **Lucas Coelho Mesquita:** Música e Trilha Sonora. Ouça a trilha oficial aqui: [Música Tema do Jogo](https://www.youtube.com/watch?v=-KvcIyfC81o).

## 🕹️ Funcionalidades

* **Sistema de Questões:** Perguntas dinâmicas sobre integrais com feedback visual imediato.
* **Modo de Exploração:** O jogador pode "levantar" da mesa para visualizar o mapa da sala e interagir com o ambiente.
* **Mecânica de Dicas (Cola):** Acesso a um guia de consulta rápida para auxiliar nas resoluções.
* **Feedback de Mouse:** Destaque dinâmico das cartas ao passar o cursor, facilitando a visualização da resposta selecionada.

## 🚀 Tecnologias Utilizadas

* [Rust](https://www.rust-lang.org/) - Linguagem de programação robusta e performática.
* [Bevy Engine](https://bevyengine.org/) - Engine de jogos ECS (Entity Component System) de última geração.

## 🛠️ Como executar o jogo no seu computador

Para compilar e rodar este jogo, você precisará ter o **Rust** e o **Cargo** instalados na sua máquina.

1. **Clone este repositório e execute o projeto:**
   ```bash
   git clone [https://github.com/Benjamin-Yuji-Suzuki/joguinhocomintegral.git](https://github.com/Benjamin-Yuji-Suzuki/joguinhocomintegral.git)
   cd joguinhocomintegral
   cargo run
