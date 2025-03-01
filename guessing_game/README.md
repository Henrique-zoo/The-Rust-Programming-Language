# Jogo de Advinhação
É a adaptação para português do primeiro projeto na documentação oficial do Rust. Consiste em um jogo em que o jogador deve adivinhar um número aleatório, se o chute dele for muito baixo, exibimos a mensagem "Muito baixo!", se for muito alto exibimos "Muito alto!" e se for exato "Você ganhou!". 

Esse projeto é uma ótima introdução à linguagem, apresentando conceitos como tipos algébricos, traits, casamento de padrão, tratamento de erros, conversão de tipos e mutabilidade de variáveis.

## Características dessa implementação
Nessa implementação do projeto, eu traduzi as mensagens para português e adicionei uma funcionalidade no tratamento de erros para inputs inválidos: caso o usuário digite letras ou números não naturais na entrada, o programa exibe a mensagem `Digite um número inteiro positivo!` a não ser que a entrada seja `sair`, nesse caso, a execução do programa termina.

Para uma melhor explicação do código, leia os extensos comentários nele ou leia o [capítulo 2 da documentação](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).