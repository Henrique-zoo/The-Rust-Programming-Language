# **The Rust Programming Language**
Esse repositório contém os códigos e anotações que criei enquanto estudava a linguagem pela sua documentação oficial [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

## **Sumário**
1. [Variáveis e Constantes](#variáveis-e-constantes)
    - [Variáveis](#variáveis)
        - [Sombreamento](#sombreamento)
    - [Constantes](#constantes)
2. [Tipos de Dados](#tipos-de-dados)

----

## **Variáveis e Constantes**

### **Variáveis**
Em Rust, variáveis são declaradas pela palavra-chave `let`. Por padrão, variáveis são imutáveis. Para fazê-las mutáveis, podemos utilizar a palavra-chave `mut`.

```rust
let x = 5; // Uma variável imutável
let mut y = 5; // Uma variável mutável

x = 6; // Erro de compilação
y = y + x; // Ignorando a última linha, tudo certo
```

#### **Sombreamento**
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

### **Constantes**
São declaradas pela palavra-chave `const`.
Assim como as variáveis imutáveis, contantes não podem mudar, a diferença é que não se pode utilizar o modificador `mut` em constantes, ou seja, constantes não são só imutáveis por padrão, elas são **sempre** imutáveis.

Na declaração de constantes, o tipo deve ser explicitamente declarado. Além disso, por padrão, os nomes de constantes em Rust são escritos em caixa alta e seguem o *snake_case*.

```rust
const TRES_HORAS_EM_SEGUNDOS: u32 = 60 * 60 * 3;
```

À constantes, só podem ser atribuidos valores constantes, não expressões que só podem ser computadas em tempo de execução. Além disso, constantes podem ser declaradas globalmente e utilizadas em todo o código.

## Tipos de Dados
Rust é uma linguagem de tipagem estática, forte mas, muitas vezes implícita. Isso quer dizer que o compilador deve saber os tipos de todas as variáveis em tempo de compilação mas, em muitos casos, consegue inferí-los.

Em Rust, existem dois subconjuntos de tipos: tipos escalares e tipos compostos. Veremos o que é cada um a seguir.

### Tipos Escalares
Um tipo escalar representa um único valor.

#### Números Inteiros
Em Rust, existem vários tipos de inteiros, que variam de acordo com o espaço de memória que eles ocupam e de eles terem ou não sinal


Lenght | Signed | Unsigned
-------|--------|---------
 8-bit |  `i8`  |   `u8`
 16-bit|  `i16` |  `u16`
 32-bit|  `i32` |  `u32`
 64-bit|  `i64` |  `u64`
128-bit| `i128` |  `u128`
  arch | `isize`| `usize`