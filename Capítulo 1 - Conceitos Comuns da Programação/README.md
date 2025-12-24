# **Sumário**
1. [Variáveis e Constantes](#variáveis-e-constantes)
    1. [Variáveis](#variáveis)
    2. [Constantes](#constantes)
2. [Tipos de Dados](#tipos-de-dados)
    1. [Tipos Escalares](#tipos-escalares)
    2. [Tipos Compostos](#tipos-compostos)
3. [Funções](#funções)
    1. [Comandos e Expressões](#comandos-e-expressões)
    2. [Funções com Valores de Saída](#funções-com-valores-de-saída)
        1. [Por que o return é um comando?](#por-que-o-return-é-um-comando)
4. [Comentários](#comentários)
5. [Controle de Fluxo](#controle-de-fluxo)
    1. [Expressão `If`](#expressão-if)
    2. [Estruturas de Repetição](#estruturas-de-repetição)
        1. [Repetição com `loop`](#repetição-com-loop)
        2. [Repetições Condicionais com `while`](#repetições-condicionais-com-while)
        3. [Percorrendo Coleções com `for`](#percorrendo-coleções-com-for)
---

# **Variáveis e Constantes**
Em linguagens de alto nível, são a ferramenta básica para manipulação de dados.

## **Variáveis**
Em Rust, variáveis são declaradas pela palavra-chave `let`. Por padrão, variáveis são imutáveis. Para fazê-las mutáveis, podemos utilizar a palavra-chave `mut`.

```rust
let x = 5; // Uma variável imutável
let mut y = 5; // Uma variável mutável

x = 6; // Erro de compilação
y = y + x; // Ignorando a última linha, tudo certo
```

### **Sombreamento**
Variáveis imutáveis não podem ter seu valor alterado, porém, o Rust permite que declaremos novas variáveis com o mesmo nome de outras já declaradas. Isso faz com que a variável declarada primeiro seja "sombreada" pela variável declarada depois. O sombreamento dura até que a nova variável saia do escopo ou até que ela, também, seja sombreada.

```rust
let x = 5; // Variável imutável `x`
println!("{x}"); // > 5
let x = 6; // Nova variável imutável `x`
println!("{x}"); // > 6

```

As diferenças entre o sombreamento e variáveis mutáveis são:
- Se tentarmos reatribuir um valor à variável `x` sem utilizar o `let`, teremos um erro de compilação;
- Após realizar as transformações na variável, teremos uma variável imutável;
- Podemos alterar o tipo da variável.

```rust
let espacos = "   ";
let espacos = espacos.len();
```

## **Constantes**
São declaradas pela palavra-chave `const`.
Assim como as variáveis imutáveis, contantes não podem mudar, a diferença é que não se pode utilizar o modificador `mut` em constantes, ou seja, constantes não são só imutáveis por padrão, elas são **sempre** imutáveis.

Na declaração de constantes, o tipo deve ser explicitamente declarado. Além disso, por padrão, os nomes de constantes em Rust são escritos em caixa alta e seguem o *snake_case*.

```rust
const TRES_HORAS_EM_SEGUNDOS: u32 = 60 * 60 * 3;
```

À constantes, só podem ser atribuidos valores constantes, não expressões que só podem ser computadas em tempo de execução. Além disso, constantes podem ser declaradas globalmente e utilizadas em todo o código.

# **Tipos de Dados**
Rust é uma linguagem de tipagem estática, forte mas, muitas vezes implícita. Isso quer dizer que o compilador deve saber os tipos de todas as variáveis em tempo de compilação mas, em muitos casos, consegue inferí-los.

Em Rust, existem dois subconjuntos de tipos: tipos escalares e tipos compostos. Veremos o que é cada um a seguir.

## **Tipos Escalares**
Um tipo escalar representa um único valor.

### **Números Inteiros**
Em Rust, existem vários tipos de inteiros, que variam de acordo com o espaço de memória que eles ocupam e de eles terem ou não sinal

Lenght | Signed | Unsigned
-------|--------|---------
 8-bit |  `i8`  |   `u8`
 16-bit|  `i16` |  `u16`
 32-bit|  `i32` |  `u32`
 64-bit|  `i64` |  `u64`
128-bit| `i128` |  `u128`
  arch | `isize`| `usize`

Os tipos `isize` e `usize` dependem da arquitetura do computador em que o programa está rodando (`x64` ou `x32`).

Literais de inteiros pode ser escritos em qualquer uma das formas da tabela a seguir. Note que se pode utilizar `_` quando for conveniente para separar o número como for conveniente de acordo com a representação. Por padrão, o compilador infere o tipo de um inteiro como `i32`.

Representação  | Exemplo 
---------------|--------
Decimal        | `98_222`
Hexa           | `0xff`
Octa           | `0o77` 
Binário        | `0b1111_0000` 
Byte(apenas `u8`)| `b'A'`

### **Números Racionais**
O Rust tem dois tipos primitivos para números com pontos flutuantes, estes são `f32` e `f64` que tem, respectivamente, 32 bits e 64 bits. Variáveis *float* sem tipagem explícita são implicitamente interpretadas como `f64`.

```rust
let x = 2.0; // f64

let y: f32 = 3.0; // f32
```

### **Valores Booleanos**
Um booleano em Rust ocupa um byte. Ele pode assumir os valores `true` or `false`.

```rust
let condicao: bool = false; // com anotação de tipo explícita
```

### **Caracteres**
O tipo `char` é o mais primitivo tipo alfabético de uma linguagem de programação. Eles são especificados com aspas simples.

```rust
let c = 'z';
let z: char = 'Z'; // com tipo explicitamente anotado
let gatinho_apaixonado = '😻';
```

Caracteres em Rust são codificados em `UTF-8`. Isso implica que, diferente de muitas outras linguagens, o tipo `char` aceita caracteres com acento, caracteres chineses, coreanos, etc. e até emojis (como no exemplo acima).

## **Tipos Compostos**
*Tipos compostos* podem agrupar vários valores em um único tipo. O Rust tem dois tipos primitivos compostos.

### **Tuplas**
Uma tupla é a maneira genérica de agrupar vários tipos. As tuplas tem um tamanho e composição pré-defindos e que não podem ser alterados.

Uma tupla é criada colocando-se várias variáveis separadas por vírgulas entre parênteses.

```rust
let tupla: (i32, f64, u8, char) = (500, 6.4, 200, '河');
```

Para pegar os valores individuais de cada elemento da tupla, podemos fazer:

```rust 
let tupla = (500, 6.4, 200, '河');

let (w, x, y, z) = tupla;

println!("O primeiro elemento é {w}");
println!("O segundo elemento é {x}");
println!("O terceiro elemento é {y}");
println!("O quarto elemento é {z}");
```

Isso é uma utilização do casamento de padrão. Outra forma de acessar um elemento específico de uma tupla é fazer:

```rust
let x: (i32, f64, u8) = (500, 6.4, 1);

println!("O primeiro elemento é {x.0}");
```

Se uma expressão não retorna um valor de nenhum tipo, ela implicitamente retorna `()`, chamado *unit*.

### **Array**
Outro tipo que pode guardar uma coleção de vários valores é o *array*. Diferente da tupla, todos os elementos de um *array* devem ter o mesmo tipo.

Um *array* é criado colocando-se várias variáveis de mesmo tipo entre vírgulas entre colchetes:

```rust
let a = [1, 2, 3, 4, 5];
```

*Arrays* são úteis quando sabemos que o número de elementos não será alterado, por exemplo:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
```

Caso se deseje declarar o tipo explicitamente, é fácil fazê-lo: coloca-se entre colchetes o tipo dos elementos do *array* e a quantidade de elementos, separados por ponto-vírgula `;`.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Também podemos criar uma *array* em que todos os elementos são iguais com uma sintaxe simplificada: colocamos entre colchetes o elemento e sua multiplicidade no *array*;

```rust
let a = [3; 5]; // ⇋ let a = [3, 3, 3, 3, 3];
```

Podemos acessar elementos específicos de um *array* pelo seu índice colocando, após o nome da variável *array*, seu índice entre colchetes.

```rust
let a = [1, 2, 3, 4, 5];

let primeiro = a[0]; // primeiro = 1
let segundo = a[1]; // segundo = 2
```

Se tentarmos acessar um índice além do limites de um *array*, o código "entra em pânico", isto é, temos um erro em tempo de execução, algo raro em Rust, pois ele costuma capturar os erros em tempo de compilação.

# **Funções**
A palavra-chave `fn` é utilizada para declarar funções. Por padrão, o *snake_case* é utilizado em seus nomes. Ademais, o nome da função é seguido por parênteses, onde ficam seus parâmetros e chaves, onde fica o corpo dá função, isto é, onde o seu comportamento é implementado. Os tipos dos parâmetros de uma função devem ser explicitamente declarados, bem como o tipo da sua saída, a não ser que este seja *unit* `()` - a tupla vazia, que é equivalente ao tipo `void` de outras linguagens.

```rust
fn soma(a: i32, b: i32) -> i32 {
    return a + b;
}
```

Ao chamar uma função, escreve-se seu nome seguido de parênteses, onde ficam seus argumentos: valores que serão associados aos parâmetros da função quando da hora de sua execução.

```rust
a = soma(2, 7);
b: i32 = soma(16, 37); // tipo explicitado (opcional)

c = soma(a, b) // Aqui, os argumentos são os valores associados às variáveis a e b
```

## **Comandos e Expressões**
Na disciplina ***Linguagens de Programação***, criamos um interpretador para a `Linguagem Imperativa 2` - uma linguagem imperativa com funções. Os conceitos explicados nessa seção são melhores entendidos sob a ótica do interpretador, pois, o que é explicado aqui diz respeito exatamente à como o interpretador/compilador lê os programas em Rust. Recomendo, portanto, a leitura dos arquivos `AbsLI.hs` e `Interpreter.hs` da [Linguagem Imperativa 2](https://github.com/Henrique-zoo/Linguagens-de-Programacao/tree/main/Interpretadores/LI2) no meu repositório da matéria [Linguagens de Programação](https://github.com/Henrique-zoo/Linguagens-de-Programacao).

O corpo de uma expressão é constituído de um bloco de comandos (*statements*) com, opcionalmente, uma expressão ao final. Comandos e expressões são definidos como segue:
- **Comandos** são instruções que performam uma ação (atualizam o contexto) e não retornam um valor.
- **Expressões** são avaliadas para um valor resultante.

Comandos podem ser constituídos por expressões, por exemplo:

```rust
let y = (3 + 6) * 8;
```

A linha é um comando de atribuição, que é constituído de um identificador (`y`, o nome da variável) e uma expressão (`(3 + 6) * 8`). À variável `y` é atribuído o valor do retorno da avaliação da expressão em questão.

Note que o lado direito de um comando de atribuição não pode ser outro comando pois comandos não retornam nenhum valor e, à uma variável, deve ser atribuído um valor. A linha a seguir gera um erro de compilação em Rust:

```rust
let x = (let y = 6);
```

- **Observação:** em algumas linguagens (como C), uma atribuição retorna o valor atribuído. Nesses casos, é possível fazer o que tentamos na linha acima.

    ```c
    int x;
    int y = (x = 6); // Resultado: x = 6 e y = 6
    ```


A definição de uma função é, também, um comando. Por exemplo:

```rust
fn main() {
    let y = 6;
}
```

é um comando.

Por outro lado, são expressões:
- Um literal de qualquer tipo;
- Uma expressão matemática, que é avaliada para um literal inteiro;
- Uma série de operações sobre strings (concatenações, etc.), que é avaliada para um literal string;
- Uma série de operações sobre booleanos, que é avaliada para um literal booleano;
- A chamada de uma função;
- A chamada de um macro;
- Um bloco de novo escopo.

Por exemplo:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
}
```

Nesse caso, temos um comando de atribuição que atribui à variável `y` o  resultado da avaliação da expressão

```rust
{
    let x = 3;
    x + 1
}
```
que é um bloco com um comando e uma expressão. A avaliação desse bloco é feita da seguinte forma:
- O comando de atribuição `let x = 3;` cria uma variável `x` no escopo desse bloco e atribui a ela o valor `3`;
- A expressão `x + 1` é avaliada e retorna o valor `3 + 1 = 4`.

O valor retornado por essa expressão interna do bloco de escopo é o valor retornado pela expressão externa (o próprio bloco). 

Note que não há ponto e vírgula após a expressão `x + 1` do código anterior. Isso ocorre porque expressões em Rust não são sucedidas de ponto e vírgula. Adicionar `;` após a expressão a tranformaria em um comando, dessa forma, não haveria expressões no bloco de escopo e ele retornaria `()`.
- **Observação:** caso houvessem duas expressões no bloco, a avaliação dele retornaria o valor da avaliação da última expressão.

Esse código tem um efeito similar ao do código a seguir:

```rust
fn main() {
    let x = 3;
    let y = x + 1;
}
```
A diferença é que, no primeiro, `x` só existe no escopo do bloco.

## Funções com Valores de Saída
Como dito anteriormente, funções podem ser finalizadas por expressões. A avaliação dessa expressão no contexto do escopo da função é o resultado da avaliação da função, ou seja, sua saída. Funções finalizadas por expressões tem valores de saída.

> Na verdade, toda função tem um valor de saída, mas as que não são finalizadas por uma expressão retornam implicitamnte o valor *unit* `()`, uma tupla vazia, e o tipo da sua saída não precisa ser explicitado. Então, fingimos que elas não têm saída, mas, de fato, elas precisam ter, pois a chamada de uma função é uma expressão, e uma expressão precisa retornar um valor.

```rust
// função que retorna um valor
fn cincun() -> u8 { // "Com Saída"
    5
}

// função que retorna `()`
fn main() { //  "Sem Saída"
    let x = cincun();

    println!("O valor de x é {x}");
}
```

Note que não é necessária a palavra-chave `return` para definir a saída de uma função, diferente de outras linguagens. Podemos, porém, utilizar o `return` para terminar a execução da função antes da sua última linha ou, simplesmente, por preferência. Nesse caso, é necessário utilizar ponto e vírgula, pois o `return` é um comando, não uma expressão.

### Por que o `return` é um comando?
O `return` retorna um valor, então por que ele é um comando? É um comando pois ele não retorna um valor no contexto em que é utilizado, por exemplo, não podemos fazer

```rust
fn main() {
    let x = return 5;
}
```
Além disso, o `return` modifica o contexto do escopo em que é utilizado, dando fim a ele, seguindo, portanto, as condições para ser um comando.

# Comentários
Em Rust, comentários são feitos com `//`. Tudo que sucede esse marcador em uma linha é ignorado pelo compilador.

Para fazer comentários de múltiplas linhas, pode-se adicionar `//` em cada linha ou utilizar `/* */`

```rust
/*
Tudo que estiver aqui dentro é ignorado pelo compilador!

😍💕💕😘😊😂🤣❤️😍😍👍👌❤️ 
*/
```

# Controle de Fluxo
É a capacidade de executar um bloco de código apenas se uma condição for verdadeira ou executá-lo repetidas vezes enquanto a condição for verdadeira.

## Expressão `if`
A expressão `if` permite que alteremos o comportamento do código em função de certas condições.

A sintaxe é a seguinte: depois da palavra-chave `if`, introduzimos uma condição, seguida de um bloco de comandos chamado "braço", que fica entre chaves. Isso diz: execute esse bloco de comando apenas se essa condição for verdadeira, caso contrário, ignore essas linhas. Opcionalmente, podemos adicionar, após o bloco de comando do `if`, um `else`, também seguido de um bloco de comandos. Nesse caso, o código diz: se essa condição for verdadeira, execute esse bloco de comandos, senão, ignore-o e execute este outro bloco.

```rust
let number = 3;

if number < 5 {
    println!("A condição é verdadeira!");
} else {
    println!("A condição é falsa!");
}
```

Diferente de algumas linguagens, como C e C++, a condição de um `if` tem que ser boolena - Em C, por exemplo, só se avalia se o valor do inteiro na condição é diferente de 0 (até porque sequer existem booleanos).

### Múltiplas condições com `else if`
Podemos combinar `else` e `if` para tratar mais de dois caminhos possíveis para o programa.

```rust
let numero = 6;

if numero % 4 == 0 {
    println!("O número é divisível por 4");
} else if numero % 3 == 0 {
    println!("O número é divisível por 3");
} else if numero % 2 == 0 {
    println!("O número é divisível por 2");
} else {
    println!("O número não é divisível por 4, 3, ou 2");
}
```

Note que, se um número for divisível por 4, 3 e 2, apenas o comando `println!("O número é divisível por 4");` será executado. Em uma estrutura `if` de controle de fluxo com vários possíveis caminhos, apenas um pode ser tomado. Para exibir mais de um divisor do `numero`, faríamos:

```rust
if numero % 4 == 0 {
    println!("O número é divisível por 4");
}
if numero % 3 == 0 {
    println!("O número é divisível por 3");
}
if numero % 2 == 0 {
    println!("O número é divisível por 2");
}
if numero % 4 != 0 && numero % 3 == 0 && numero % 2 == 0 {
    println!("O número não é divisível por 4, 3, ou 2");
}
```

### Utilizando `if` em comandos de atribuição
Um `if` no Rust é uma expressão, não um comando. Dessa forma, ao invés de ocupar espaço com a longa sintaxe do `if` padrão, podemos utilizar uma simplificação dessa sintaxe para comando de atribuição.

```rust
let condicao = true;
let numero = if condicao { 5 } else { 6 };
```
**Não confunda essa simplificação do `if` com um operador ternário.** Em muitas linguagens em que o `if` é um comando (ex.: Java, C, Python etc.), criou-se uma espécie de `if` alternativo que é uma expressão: o operador ternário ou operador condicional. Em Rust, o `if` padrão já é uma expressão, logo, não é necessário o operador ternário. Em suma, não há nada novo na sintaxe acima. **Esse é um `if` normal**, simplesmente não pulamos linhas ao escrevê-lo. Poderíamos fazer

```rust
let condicao = true;

let numero = if condicao {
        5
    } else {
        6
    }
;
```
Lembre-se que uma variável só pode ter um tipo, portanto, os braços do `if` e do `else` devem ser avaliados para o mesmo tipo. Dessa forma, o código a seguir gera um erro de compilação:

```rust
fn main() {
    let condition = true;

    let numero = if condition { 5 } else { "seis" };

    println!("O valor do número é: {numero}");
}
```

## Estruturas de Repetição
O Rust tem três tipos de estruturas de repetição: `loop`, `while` e `for`.

### Repetição com `loop`
A palavra-chave `loop` faz o Rust executar um bloco de comandos até que você explicitamente o faça parar. Caso o loop não seja quebrado pelo código, podemos quebrá-lo pressionando `Ctrl + C` no terminal. A palavra-chave utilizada para quebrar um loop é `break`. Podemos, também, utilizar a palavra-chave `continue` para que os comandos após a sua utilização não sejam executados em algum caso específico.

```rust
fn main() {
    let soma = 0;
    let mut n = 10;

    loop {
        if n == 0 {
            break; // quebramos o loop quando n é igual a 0
        } else if 10 % n == 0 {
            continue; // não somamos divisores de 10
        }
        
        let soma = soma + n;
        n -= 1;
    }

    println!("O resultado da soma é {soma}")
}
```

#### Retornando valores de Loops
Podemos adicionar uma expressão após o `break` para que a expressão `loop` seja avaliada para um valor diferente de `()`. Por exemplo:

```rust
fn main() {
    let mut contador = 0;

    let resultado = loop {
        contador += 1;

        if contador == 10 {
            break contador * 2;
        }
    };

    println!("O resultado é {resultado}");
}
```

#### Rótulos de Loops para Desambiguação
Quando temos vários loops encasquetados, as palavras-chave `break` e `continue` se aplicam ao loop mais interno nesse ponto. Podemos rotular os loops para aplicá-las ao loop que quisermos:

```rust
fn main() {
    let mut contador = 0;
    'contagem_crescente: loop {
        println!("contador = {contador}");
        let mut faltando = 10;

        loop {
            println!("faltando = {faltando}");
            if faltando == 9 {
                break;
            }
            if contador == 2 {
                break 'contagem_crescente;
            }
            faltando -= 1;
        }

        contador += 1;
    }
    println!("Fim contador = {contador}");
}
```

### Repetições Condicionais com `while`
Quando queremos repetir um bloco de comandos enquanto uma condição for verdade, podemos usar a estrutura de repetição `while`. É possível implementar qualquer algoritmo que use a estrutura de repetição `while` utilizando `loop`, `if-else` e `break`, mas, como esse é um padrão muito comum, o Rust (assim como quase todas as linguagens de programação) decidiu constrí-lo embutido na linguagem.

```rust
let mut l = 0;
let mut r = 10;

while l < r {
    println!("l é menor que r");
    l += 1;
}
```

Também podemos percorrer uma coleção com `while`

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut i = 0;

    while i < 5 {
        println!("O valor é: {}", a[i]);

        i += 1;
    }
}
```
porém, essa não é a melhor solução pois:
1. Se tentarmos acessar um índice além dos limites do vetor, o programa apresentará um erro em tempo de execução;
2. Se aumentarmos o tamanho do vetor, precisaremos reestrutrar o código.

### Percorrendo Coleções com `for`
O `for` é a solução perfeita para percorrer uma coleção. Fazemos

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for elemento in a {
        println!("o valor é: {elemento}");
    }
}
```

O `for` também pode ser utilizado intercambiavelmente com o `while`. Basta utilizarmos um `Range`, que cria um vetor com todos os números naturais dentro de um intervalo fechado no início e aberto no final, e conseguimos fazer tudo com um `for`.

```rust
for numero in (1..4) { // [1, 2, 3]
    println!("{numero}");
}
```