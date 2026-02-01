# **Sumário**
1. [Structs](#structs)
    1. [Tuple Structs](#tuple-structs)
    2. [Unit-Like Structs](#unit-like-structs)
    3. [Ownership de Structs](#ownership-de-structs)
2. [Enums](#enums)
    1. [A Enum Option](#a-enum-option)
    2. [Controle de Fluxo com o `match`](#controle-de-fluxo-com-o-match)
    3. [Controle de Fluxo Conciso com `if let`](#controle-de-fluxo-conciso-com-if-let)
    4. [Controle de Fluxo com `let else`](#controle-de-fluxo-com-let--else)
3. [Métodos](#métodos)
    1. [O Operador `->`](#o-operador--)
    2. [Funções Associadas](#funções-associadas)
    3. [Múltiplos Blocos `impl`](#múltiplos-blocos-impl)
----
# Structs
Structs, assim como tuplas, combinam múltiplos dados relacionados e esses dados podem ser de diferentes tipos. Diferente de tuplas, em structs nomeamos cada dado que os compõem, de forma que sabemos sempre seu significado. 

Formalmente, tanto structs quanto tuplas são **Tipos Produto**, uma classe de **Tipos Algébricos** (ADTs). Esses conceitos são melhor explicados no repositório [Linguagens de Programação](https://github.com/Henrique-zoo/Linguagens-de-Programacao).

Para definir uma struct em rust, nós inserimos a palavra chave `struct` e, em seguida, o nome que daremos a ela, que deve representar o significado dos dados que ela agrupa. Então, entre chaves, definimos os nomes e tipos de cada "pedaço" de dado da struct, esses "pedaços" são chamados de **campos**.

```rust
struct Usuario {
    ativo: bool,
    nome_de_usuario: String,
    email: String,
}
```

Para utilizar uma struct que definimos, nós a instanciamos, especificando valores concretos para os campos que definimos. A instanciação é feita da seguinte forma: inserimos o nome da struct e, entre chaves, os nomes e valores de cada campo. A ordem em que os campos são especificados não precisa ser a mesma da definição da struct.

```rust
fn main() {
    let usuario1 = Usuario {
        ativo: true,
        email: String::from("algumemail@exemplo.com"),
        nome_de_usuario: String::from("algumnome123"),
    };
}
```

Para acessar os campos da struct, utilizamos *dot notation*. Por exemplo, para acessar o email do usuário definido acima, usamos `usuario1.email`. Se a variável do tipo da struct for mutável, podemos utilizar *dot notation* para, com uma atribuição, alterar o valor de um campo.

```rust
usuario1.email = String::from("outroemail@exemplo.com");
```

Note que a instancia inteira precisa ser mutável; não é possível marcar apenas alguns campos como mutáveis.

Ao invés de criar uma instância assim, nós poderíamos pensar em criar uma função que recebe como parâmetros os valores que queremos atribuir aos campos e retorna a struct construída.

```rust
fn construir_usuario(email: String, nome_de_usuario: String) -> Usuario {
    Usuario {
        ativo: true,
        nome_de_usuario: nome_de_usuario,
        email: email,
    }
}
```

#### *Field init shorthand syntax*
Faz sentido que os parâmetros da função tenham o mesmo nome dos campos da struct, mas ter que repetir o texto pode ser um pouco tedioso. Como é muito commum que os parâmetros e campos tenham o mesmo nome, o rust simplifica a sintaxe para esse caso: podemos usar a *field init shorthand syntax*, que nos permite omitir o campo quando o nome é o mesmo.

```rust
fn construir_usuario(email: String, nome_de_usuario: String) -> Usuario {
    Usuario {
        ativo: true,
        nome_de_usuario,
        email,
    }
}
```

#### *Struct update syntax*
É muito comum termos que criar uma instância de uma struct que compartilha a maior parte de valores dos campos de outra instância. Nesse caso, teríamos que fazer:

```rust
fn main() {
    let usuario2 = Usuario {
        ativo: usuario1.ativo,
        nome_de_usuario: usuario1.nome_de_usuario,
        email: String::from("outroemail@exemplo.com"),
    };
}
```

que é muito verboso. Ao invés disso, o rust oferece uma sintaxe mais enxuta:

```rust
fn main() {
    let usuario2 = Usuario {
        email: String::from("outroemail@exemplo.com"),
        ..usuario1
    };
}
```

Os dois códigos acima tem exatamente o mesmo efeito. Em ambos, o valor no campo `nome_de_usuario` foi movido para o `usuario2` e o `usuario1` não pode mais ser usado depois disso (`usuario1.ativo` não foi movido porque o tipo `bool` implementa `Copy`). Se tivessemos definido `usuario2` com novos valores de `email` e `nome_de_usuario`, `usuario1` ainda seria válido.

## Tuple Structs
O rust também suporta um tipo de struct que parace tupla, chamado *tuple structs*. Nessas structs, os campos não são nomeados; eleas são basicamente tuplas com nome.

```rust
struct Cor(i32, i32, i32);
struct Ponto(i32, i32, i32);

fn main() {
    let preto = Color(0, 0, 0);
    let origem = Ponto(0, 0, 0);
}
```
Aqui, `Cor` e `Ponto` não são apenas os nomes dos tipos criados, são **construtores-função**, isto é, essa sintaxe, em nível de compilação, não vai apenas gerar um tipo, vai gerar também uma função. Simbolicamente, podemos representar essa função como segue:

```rust
fn Ponto(x: i32, y: i32, z: i32) -> Ponto = {
    Ponto {
        0: x,
        1: y,
        2: z,
    }
}
```
ou seja, em baixo nível, instanciar uma *tuple struct* realiza uma chamada de função. No caso de structs comuns, nenhuma função é chamada, o compilador só aloca o espaço de memória para o tipo, inicializa cada campo e move os valores para os seus campos.

Isso implica que, utilizando *tuple structs*, podemos fazer, por exemplo,

```rust 
let f: fn(i32, i32, i32) -> Ponto = Ponto
```
o que mostra que, além do tipo, a função `Ponto` de fato é criada.

Voltando ao exemplo, note que, mesmo que `preto` e `origem` tenham a mesma quantidade de campos de mesmos tipos, eles ainda são de tipos diferentes. Assim como em tuplas, nesse tipo de struct os dados podem ser acessados por `<nome>.` seguido do índice do campo. Diferente de tuplas, para desestrurar tuple structs, é necessário especificar o nome da struct.

```rust
struct Ponto(i32, i32, i32);

fn main() {
    let tupla_origem = (0, 0, 0);
    let struct_origem = Ponto(0, 0, 0);

    let (x, y, z) = tupla_origem;
    let Ponto(x, y, z) = struct_origem;
}
```

## Unit-Like Structs
Também é possível definir structs sem nenhum campo. Essas são chamadas de *unit-Like structs* por se comportarem como o tipo unit `()`. Essas structs são úteis quando queremos implementar um trait em um tipo mas esse tipo não precisa armazenar nenhum dado.

```rust
struct SempreIgual;

fn main() {
    let subject = SempreIgual;
}
```

Imagine que depois implementaremos um comportamento para esse tipo tal que qualquer instância de `SempreIgual` é sempre igual a qualquer instância de qualquer outro tipo; nós não precisariamos de qualquer dado para isso.

## Ownership de Structs
Note que, na definição da Struct `Usuario`, nós usamos o tipo `String`, não o `&str`, de literais ou fatias de string. De fato, structs podem conter dados sobre os quais ela não tem posse (referências), mas fazer isso requer o uso de *lifetimes*, um recurso do rust que ainda não foi discutido. *Lifetimes* garantem que o dado referenciado é valido enquanto a struct que o contém for. Se tentarmos armazenar uma referência numa strucr sem considerações sobre *lifetime* obteremos um erro em tempo de compilação.

```rust
struct Usuario {
    ativo: bool,
    nome_de_usuario: &str,
    email: &str,
}

fn main() {
    let ususario1 = Usuario {
        ativo: true,
        nome_de_usuario: "algumnome",
        email: "algumemail@exemplo.com",
    };
}
```

O erro é o seguinte:

```cmd
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:4:12
  |
4 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 |     username: &str,
4 ~     email: &'a str,
  |

For more information about this error, try `rustc --explain E0106`.
error: could not compile `structs` (bin "structs") due to 2 previous errors
```

# Enums
Os enums, assim como as structs, permitem combinar outros tipos e construir um tipo mais complexo. A diferença entre os dois é que, enquanto nas structs o tipo criado deve assumir um valor para cada tipo que a compõe, nos enums ele assume o valor de um dos tipos que o compõe.

Formalmente, enums são a forma como o rust implementa a outra classe de tipos algébricos, os tipos soma (também explicados no repositório [Linguagens de Programação](https://github.com/Henrique-zoo/Linguagens-de-Programacao)).

Para definir uma enum, inserimos a palavra chave `enum`, o nome que daremos a ela e, em seguida, entre chaves, as suas variantes.

```rust
enum Forma {
    Retangulo,
    Triangulo,
    Circulo,
}
```
Note que `Retangulo`, `Triangulo` e `Circulo` parecem bastante *unit-like structs*. No exemplo acima, apenas conseguiríamos distinguir a forma com que estamos tratando, mas não calcular área ou perímetro dessas formas, pois não há nenhum valor associado a elas. Para ter acesso a esses valores, ao invés de *unit-like structs*, podemos definir as variantes de forma similar a como definimos *tuple structs*, por exemplo:

```rust
enum Forma {
    Retangulo(u64, u64),
    Triangulo(u64, u64),
    Circulo(u64),
}
```

Ou até de forma similar a como definimos structs nomeadas:

```rust
enum Forma {
    Retangulo{
        base: u64,
        altura: u64,
    },
    Triangulo{
        base: u64,
        altura: u64,
    },
    Circulo{
        raio: u64,
    },
}
```

A sintaxe das variantes é muito parecida com a de structs. Por que eu me refiro a elas apenas como similares? Apesar da semelhança no código em alto nível, há uma grande diferença em nível de compilação. Lembre-se das *tuple structs*: ao declará-las, o que o compilador faz? Ele cria um construtor-função e um tipo; com as structs nomeadas é similar: o compilador cria o tipo, mas não cria o contrutor-função. No caso das variantes de uma enum o tipo não é criado, apenas o construtor-função. De fato o **único** tipo criado na definição de uma enum é o que ela define - as variantes não criam tipos nomeáveis, apenas injetores (construtores) que retornam o tipo da enum. Dessa forma, se lá tínhamos, para uma *tuple struct* `A(...)`, uma função `A: fn(...) -> A`, aqui temos, para uma `enum E { A(...) }`, uma função `E::A: fn(...) -> E`, em que `E::A` representa que a função `A` só existe no *namespace* do tipo `E`.

Para instanciar enums, chamamos o construtor da variante, claro, no namespace da enum.

```rust
let quatro = IpEnd::V4(String::from("127.0.0.1"));
let seis = IpEnd::V6(String::from("::1"));
```

Dessa forma, se uma função tem como parâmetro algo do tipo da enum, podemos chamá-la com argumentos de todas as variantes dessa enum:

```rust
fn rota(endereco_ip: IpEnd) {}

fn main() {
    rota(IpEnd::V4(String::from("127.0.0.1")));
    rota(IpEnd::V6(String::from("::1")));
}
```

Essa é a grande vantagem de usar enums. De fato, se não fosse essa propriedade, não haveria vantagem de enums em relação a simplesmente definir structs para cada variante da enum e fazer alguma "gambiarra" para conseguir aplicar uma função a todos esses tipos.

## A Enum `Option`
Uma escolha de design da linguagem rust é, diferente de muitas linguagens, não ter um tipo `null`. Apesar do conceito que o `null` representa ser, de fato, útil na programação, sua implementação é motivo de muitos problemas - algo que o próprio criador desse conceito admite.

De fato, em linguagens que têm o tipo `null`, é muito comum que o programador não verifique, se uma variável assume esse valor antes de realizar uma série de operações (que não devem ser realizadas com um null); isso leva a erros em tempo de execução. Com a enum `Option`, isso é impossível: o tipo `Option<T>` é diferente do tipo `T`, não é possível manipular um como se manipula o outro. Dessa forma, o que se tornaria um erro em tempo de execução é trazido para o tempo de compilação, pois o rust te obriga a garantir que um valor é válido antes de manipulá-lo. Por exemplo, o código a seguir não funciona

```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let soma = x + y; // você não pode somar um i8 e um Option!
```

`Option` é uma enum definida na *standard library*. Ela é exatamente igual ao `Maybe` do haskell.

A enum `Option` serve para representar casos em que um valor pode ser algo ou nada. Por exemplo: se uma função retorna o primeiro elemento de uma lista, ela pode (i) retornar `None` se a lista for vazia ou (ii) retornar `Some(valor)`, em que `valor` é o primeiro elemento da lista.

## Controle de Fluxo com o `match`
O `match` é uma ferramenta extremamente poderosa de controle de fluxo. Com ele, você pode comparar um valor com uma série de padrões e executar trechos de código com base no padrão que "combina" com o valor. Um padrões são bem flexíveis: vão de variantes de uma enum até literais. O `match` é poderoso por ser extremamente expressivo e garantir que todos os possíveis casos são tratados.

A expressão `match` compara sequencialmente o valor com os padrõe. O primeiro que "encaixa" é aquele cujo código correspondente será executado.

O match é usado da seguinte forma: insere-se a palavra-chave `match`, que é seguida de uma expressão. Então, entre chaves, são declarados, separados por vírgulas, os "braços" do `match` (match arms) da seguinte forma: `<padrão> => <expressão>`. Por exemplo,
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
A expressão que segue o `match` pode ser de qualquer tipo. O resultado da avaliação dessa expressão é o que será comparado com os padrões. As expressões de cada braço devem ser todas do mesmo tipo, senão o compilador não conseguiria fazer posteriores verificações de tipo com a variável à qual foi atribuída a expressão `match`.

Note que o fato de um padrão estar ligado a uma expressão não é restritivo: um bloco de código é uma expressão e uma expressão pode sempre ser do tipo unit `()`, ou seja, efetivamente, o `match` pode ser utilizado tanto como expressão (em atribuições, por exemplo) quanto como comando (associando-se aos braços expressões do tipo unit focadas em efeito colateral).

Se a expressão de uma braço for de múltiplas linhas, então ela é um bloco e deve ficar entre chaves.
```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Outra utilidade do `match` é "tirar" um valor de dentro de uma enum. Isto é, se estamos fazendo um `match` sobre uma enum em que as variantes guardam valores, é possível atribuir esses valores a variáveis no escopo do braço do `match` e utilizá-los. 

```rust
enum Forma2D {
    Retangulo(f64, f64),
    Triangulo(f64, f64),
    Circulo(f64)
}

fn area(forma: &Forma2D) -> f64 {
    match forma {
        Forma2D::Retangulo(l, h) => l*h,
        Forma2D::Triangulo(b, h) => b*h/2.0,
        Forma2D::Circulo(r) => 3.14*r*r
    }
}
```

Como já foi mencionado, o `match` é exaustivo, i.e., ele deve cobrir todas as possibilidades, caso contrário, haverá erro de compilação. Nós podemos, porém, definir um comportamento padrão com um padrão *catch-all*; basicamente, ao invés de utilizar um padrão restritivo (como a variante de uma enum o um literal) num braço do `match`, simplesmente utilizamos uma variável, que pode ser qualquer coisa:
```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
```
Note que esse *catch-all* deve estar no final do `match`, senão os outros casos seriam inatingíveis.

Uma sintaxe especial para evitar *warning* de varível não utilizada é, ao invés de uma variável normal, utilizar o *placeholder* `_`. Isso indica ao Rust que o valor não será utilizado.

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
```

## Controle de Fluxo Conciso com `if let`
O `if let` é uma maneira menos verbosa de lidar com valores que encaixam em um padrão (ignorando outros possíveis padrões).

Considere um caso em, caso o valor de uma variável de tipo `Option<u8>` seja um `Some`, queiramos executar uma série de comandos e, caso seja `None`, não queiramos fazer nada. Com a construção `match`, precisariamos fazer:

```rust
match var {
    Some(val) => {
        ...
    },
    None => ()
}
```

A exaustividade do `match` se torna um estorvo nesse caso; não estamos fazendo uma atribuição, logo, não precisamos garantir que uma variável (à qual seria atribuído esse `match`) seja sempre válida. Esse caso é um exemplo que justifica a concisa expressão `if let`. Um código equivalente ao do exemplo acima é:

```rust
if let Some(val) = var {
    ...
}
```
A sintaxe do `if let` separa um padrão e uma expressão por um sinal de igual `=`. Pode

Apesar do exemplo tratar de um uso do `if let` como comando, ele também é uma expressão. Isso significa que ele precisa retornar algo sempre, inclusive quando o padrão não encaixa; nesse caso, ele retorna *unit* (`()`) por padrão. Note que, para o uso como comando, isso não é um problema, mas e numa atribuição? Se atribuímos à uma variável o `if let`, o tipo retornado caso o padrão encaixe será diferente do tipo *unit*, o que gerará um erro de compilação - assim, como todos os braços do `match` devem ser avaliados para o mesmo tipo, todos os caminhos com `if let` também devem. A solução para isso é bastante intuitiva: o `else`. Quando queremos utilizar o `if let` como expressão (em uma atribuição, por exemplo) basta definirmos um valor a ser retornado no caso padrão que tenha o mesmo tipo dos outros casos; isso é feito via `else`. Considere o caso em que o status de um usuário pode ser `Ativo`, `Inativo` ou `Suspenso`; queremos fazer uma função que verifique o acesso:
```rust
enum Status {
    Ativo {
        usuario: String,
        desde: String
    },
    Inativo,
    Suspenso(String),
}

fn verificar_acesso(status: &Status) -> String {
    if let Status::Ativo { usuario, desde } = status {
        format!("✅ Usuário '{}' ativo desde {}", usuario, desde)
    } else {
        "❌ Acesso negado".to_string()
    }
}
```
Note que essa sintaxe só é realmente conveniente em casos em que há um comportamento padrão e poucos casos fogem a ele. Se estivermos manipulando uma enum com muitas variantes tais que cada um induz um comportamento diferente, é melhor usar o `match`.

## Controle de Fluxo com `let ... else`
Ok, que essa é uma sintaxe bem estranha. Suponha que queiramos um código que compare uma variável com um determinado padrão e que execute um bloco de código se e o padrão encaixar. Esse é exatamente o mesmo caso de uso do `if let`, mas suponha, adicionalmente, que o bloco de código a ser executado seja bastante verboso. Veja o exemplo:
```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
```
Esse código é difícil de acompanhar, é fácil se perder nos blocos encadeados. Uma solução para deixar o código mais limpo é o `let ... else`. Com ele, podemos colocar o caso em que padrão não encaixa, que tende a ter o código mais simples, dentro do bloco, tratando o caso em que o padrão encaixa por fora e evitando o encadeamento de muitas estruturas.
```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        // Se o padrão não encaixa
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

# Métodos
Métodos em Rust são semelhantes a funções: são declarados com a palavra-chave `fn`, têm um nome e contêm código executável. A diferença principal é que métodos são definidos no contexto de um tipo específico (struct, enum ou trait object). Isso é uma prática comum em linguagens que suportam programação orientada a objetos, onde tipos podem ter funções associadas. Esse conceito é comum na programação orientada a objetos, onde as classes podem ser munidas de métodos - funções associadas a elas que só existem no seu contexto.

Ora, se elas são como funções, por que se preocupar em usá-las? Não poderíamos simplesmente utilizar as funções? A reposta está na organização semântica do código: é mais coerente que funções que só fazem sentido em um contexto específico só existam nele. Por exemplo, a função `area` claramente só faz sentido no contexto da enum `Forma2D`; não há razão para deixá-la acessível em todo o código. Portanto, faz sentido fazer:
```rust
enum Forma2D {
    Retangulo(f64, f64),
    Triangulo(f64, f64),
    Circulo(f64)
}

impl Forma2D {
    fn area(&self) -> f64 {
        match self {
            Forma2D::Retangulo(l, h) => l * h,
            Forma2D::Triangulo(b, h) => b * h / 2.0,
            Forma2D::Circulo(r) => 3.14 * r * r
        }
    }
}
```
Agora, a função `area` só pode ser chamada no contexto apropriado:
```rust
fn main() {
    let ret = Forma2D::Retangulo(30, 50);
    let area = ret.area();
}
```
Como pode ser visto no exemplo, para definir um método, nós começamos um bloco `impl` para o tipo em questão (tudo dentro desse bloco será associado a esse tipo) e definimos uma função dentro dele. Note que o parâmetro virou `&self` (pode ser `self` ou `&mut self`). Esse é um parâmetro especial: seu tipo é automaticamente inferido como da struct/enum do bloco `impl`, além disso, a forma como o argumento associado a esse parâmetro é passado pode ser diferente: ao invés de colocá-lo entre os parênteses que seguem o nome da função, ele antecede o nome da função e é seguido por um ponto - essa é a chamada *method sintax*. Métodos também podem ser chamados com sintaxe de função:
```rust
fn main() {
    let ret = Forma2D::Retangulo(30, 50);
    let area = Forma2D::area(&ret);
}
```
O parâmetro `&self` merece um pouco mais de atenção. Na verdade, ele é uma abreviação de `self: &Self`, em que `Self` é um "apelido" (*alias*), dentro do escopo do `impl`, para o tipo para o qual definimos o `impl`. Como todo método tem que ter um parâmetro `self` de tipo `Self`, o Rust nos deixa abreviar isso com apenas `self` (com os modificadores convenientes `&` ou `&mut `). Como já foi adiantado, o `self` pode ser modificado, como qualquer outro parâmetro, para ser recebido como referência mutável ou imutável. 

Os métodos tem mais uma vantagem além de organização semântica: o bloco `impl` encapsula todas as valências de um tipo, facilitando o uso desse tipo para outros usuários.

É comum criar métodos com o mesmo nome de campos de um tipo que apenas retornem o valor desses campos. Esses métodos são chamados *getters* (talvez você conheça isso de POO). O Rust, diferente de outras linguagens, não implementa esses métodos automaticamente. Os *getters* são úteis quando, na API pública de um tipo, queremos fazer com que determinados atributos sejam apenas para a leitura, pois podemos fazer dos atributos privados e dos métodos *getters* públicos; isso será melhor discutido depois.

## O Operador `->`
Em C e C++, há dois operadores para chamar métodos e atributos de structs (ou classes, em C++): o `.`, que é usado quando você quer chamar um método ou atributo diretamente de um objeto e o `->`, que serve para chamar o método/atributo em um ponteiro para um objeto. O operador `->` serve, basicamente, para fazer a desreferência e chamar o método (pois os métodos recebem, obrigatoriamente, um objeto *self*, não um ponteiro). Ou seja, nessas linguagens, `objeto->metodo` é o mesmo que `(*objeto).metodo`.

O Rust tem referências, correto? Então não seria necessário um operador análogo para chamar os métodos nelas? O Rust optou por outra solução (uma mais simples): esse é um dos poucos casos em que o Rust faz, automaticamente o referenciamento ou desreferenciamento. Isso quer dizer que você simplesmente utiliza `objeto.metodo` e o Rust automaticamente adiciona `&`, `&mut` ou `*` para que `objeto` fique de acordo com a assinatura do método (isto é, com como o método utiliza `self`)

## Funções associadas
Todas as funções definidas dentro de um bloco `impl` são chamadas funções associadas. Se uma função associada não tem `self` como parâmetro, ou seja, não precisa de uma instância do tipo para funcionar, então ela não é um método.

Um caso típico de função associada que não é um método é uma função construtora, i.e., uma fução que cria uma instância de um tipo. Normalmente, essas funções recebem o nome `new`, mas esse nome não é reservado - nós poderíamos chamar uma função construtora de `cleiton`. Por exemplo, podemos fazer uma função associada `quadrado` para construir um retângulo com largura e altura iguais:
```rust
impl Forma2D {
    fn quadrado(size: f64) -> Self {
        Self::Retangulo(size, size)
    }
}
```
Para chamar uma função dessas, fazemos:
```rust
fn main() {
    let square = Forma2D::square(3);
}
```
Note que não é necessária uma instância do tipo para chamar uma função desse tipo. Essas funções são análogas a métodos estáticos em Java ou C++, por exemplo, que também podem ser chamadas sem um objeto.

## Múltiplos Blocos `impl`
Pode-se definir a quantidade de blocos `impl` que se quiser para um único tipo. Via de Regra, é melhor usar só um, mas há casos em que é útil utilizar mais de um.