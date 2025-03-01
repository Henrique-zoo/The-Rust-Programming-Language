use rand::Rng; // Rng é um trait
use std::{cmp::Ordering, io}; // io é uma crate, Ordering é um tipo soma da biblioteca `cmp`

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng() // O gerador de número
                                .gen_range(1..=100); // gera um número número aleatório no range definido por star..=end [start, end]

    loop {
        println!("Por favor, digite o seu chute. (digite 'sair' para sair)");

        let mut guess_str = String::new(); // :: é uma sintaxe que indica que um método está associada à um tipo ou trait

        // std::io::stdin() retorna uma instância do tipo Stdin do módulo io do crate std (std::io::Stdin)
        io::stdin()
            .read_line(&mut guess_str) // O método read_line é chamado no tratador de input padrão (std::io::Stdin). Seu argumento é a string mutável `guess_str`. À ela, é anexada a entrada do usuário (por isso a string de argumento tem que se mutável)
            // `guess_str` é passado como referência, i.e, o método acessa o espaço de memória em que a variável está definida, não criamos uma cópia da variável em outro espaço de memória para aplicar o método.
            // .read_line retorna um Result - um tipo soma que pode ser ou um `Ok` ou `Err`
            .expect("Falha ao ler a linha");
            // O expect:
                // Causa um crash no programa e retorna uma mensagem de erro se a instância do Result for `Err`
                // Retorna o valor que `Ok` está guardadndo se a instância do Result for `Ok`

        // Aqui, convertemos guess_str para um inteiro sem sinal. No `parse()`,  não é necessário denotar o tipo, pois isso é compreendido pelo compilador pelo tipo explicitamente anotado (u32) da variável que recebe o resultado dessa expressão
        let guess: u32 = match guess_str.trim().parse() { // Essa expressão (junto com a comparação no match) tem mais um efeito colateral: o interpretador entende que o tipo da variável secret number é u32.
            Ok(num) => num,
            Err(_) => {
                if guess_str.trim() == "sair" {
                    break;
                } else {
                    println!("Digite um número inteiro positivo!");
                    continue;
                }
            }
        }; // Note que nem toda variável pode ser convertida para um inteiro. Por isso, o parse() retorna, assim como o read_line(), um Result. Tratamos ele de forma a terminar o programa se o usuário digitou "quit" e ignorar o input se ele digitou uma letra qualquer.

        // Essa linha tem o mesmo resultado de `println!("You guessed: {guess}")`
        println!("Você chutou: {}", guess); // {} é um placeholder para a expressão após a vírgula

        // O match representa o casamento de padrões do Rust. Ele associa a cada variante de Ordering um comportamento do programa
        match guess.cmp(&secret_number) { // O método cmp compara dois valores e retorna um algo do tipo Ordering (o resultado da comparação entre guess e secret_number)
            Ordering::Less    => println!("Muito pequeno!"), // Comportamento se guess < secret_number
            Ordering::Greater => println!("Muito grande!"), // Comportamento se guess > secret_number
            Ordering::Equal   => {
                println!("Você ganhou!"); // Comportamento se guess == secret_number
                break; // Termina o jogo
            }
        } // Note que, aqui, `::` não tem o mesmo significado das ocorrências anteriores
    }
}
