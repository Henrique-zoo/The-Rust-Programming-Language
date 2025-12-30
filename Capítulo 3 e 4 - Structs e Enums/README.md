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

Uma escolha de design da linguagem rust é, diferente de muitas linguagens, não ter um tipo `null`. Apesar do conceito que o `null` representa se, de fato, útil na programação, sua implementação é motivo de muitos problemas - algo que o próprio criador desse conceito admite. 

`Option` é uma enum definida na *standard library*. Ela é exatamente igual ao `Maybe` do haskell.

A enum `Option` serve para representar casos em que um valor pode ser algo ou nada. Por exemplo: se uma função retorna o primeiro elemento de uma lista, ela pode (i) retornar `None` se a lista for vazia ou (ii) retornar `Some(valor)`, em que `valor` é o primeiro elemento da lista.
