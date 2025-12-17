use std::{collections::HashMap, vec, io};

type Matrix = HashMap<usize, HashMap<usize, usize>>;

fn multiplicacao_de_matrizes(p: &Vec<usize>) -> (Matrix, Matrix) {
    let n = p.len() - 1;
    let mut m: Matrix = HashMap::new();
    let mut s: Matrix = HashMap::new();

    for l in 2..=n { // l é a quantidade de matrizes que multiplicamos em uma associação
        for i in 1..=n-l+1 { // i vai de 1 até a posição inicial da última associação de tamanho l possível
            let j = i + l - 1;
            for k in i..=j { // k vai da posição inicial da associação (i) até a final(i+l-1), somando o "peso" das subassociações que formam a associação atual e comparando com o menor peso já computado dessa associação
                let q = m.get(&i)
                    .and_then(|x| x.get(&k))
                    .unwrap_or(&0)
                    + m.get(&(k+1))
                    .and_then(|x| x.get(&j))
                    .unwrap_or(&0)
                    + p[i-1] * p[k] * p[j];
                let &menor = m.get(&i)
                    .and_then(|x| x.get(&j))
                    .unwrap_or(&0);
                if q < menor || menor == 0 { // Se q for menor do que o menor peso dessa associação já computado, q é o menor peso dessa associação até a atual iteração
                    m.entry(i)
                        .or_insert_with(HashMap::new)
                        .insert(j, q);
                    s.entry(i)
                        .or_insert_with(HashMap::new)
                        .insert(j, k);
                }
            }
        }
    }
    (m, s)
}

fn encontra_associacoes(saida: &mut [String], s: &Matrix, l:usize, r: usize) {
    if r - l >= 2 {
        let &sep_i = s.get(&l)
            .and_then(|x| x.get(&r))
            .unwrap();

        if r - sep_i > 1 && sep_i - l > 1 { 
            saida[2*sep_i] = ")".to_owned() + &saida[2*sep_i] + "(";
            saida[2*(l-1)] += "(";
            saida[2*r] = ")".to_owned() + &saida[2*r];
        } else if  r - sep_i > 1 {
            saida[2*sep_i] += "(";
            saida[2*r] = ")".to_owned() + &saida[2*r];
        } else if sep_i > l {
            saida[2*(l-1)] += "(";
            saida[2*sep_i] = ")".to_owned() + &saida[2*sep_i];
        }
        encontra_associacoes(saida, s, l, sep_i);
        encontra_associacoes(saida, s, sep_i+1, r);
    }
}

fn format_index(x: usize) -> String {
    let subscritos: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];
    x.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .map(|d| subscritos[d])
        .collect()
}

fn main() {
    println!("Digite as dimensões das matrizes (separe por vírgulas)");
    let mut p = String::new();

    io::stdin()
        .read_line(&mut p)
        .expect("Falha ao ler a linha");

    let p:Vec<usize> = p.split(",")
        .map(|s| s.trim()) // Remove espaços e \n
        .map(|s| s.parse::<usize>()
        .expect("Valor inválido, digite números inteiros"))
        .collect();

    let (m,s) = multiplicacao_de_matrizes(&p);

    let n = p.len() - 1;
    let mut saida: Vec<String> = vec!(String::from(""));

    for i in 0..n {
        saida.push(format!("A{}ₓ{}", format_index(p[i]), format_index(p[i+1])));
        if i != n-1 { saida.push(String::from("∙")) } else { saida.push(String::from(" ")) }
    }

    encontra_associacoes(&mut saida, &s, 1, n);
    
    println!("A menor quantidade de multiplicações possível é {}.", m.get(&1).and_then(|x| x.get(&n)).unwrap());
    println!("Ela ocorre quando multiplicamos as matrizes de acordo com a expressão a seguir:");
    for termo in &saida {
        print!("{termo}");
    }
    println!();

    /*for (i, inner_map) in &m {
        for (j, value) in inner_map {
            println!("m[{}][{}] = {} ", i, j, value);
        }
    }
    println!();
    for (i, inner_map) in &s {
        for (j, value) in inner_map {
            println!("s[{}][{}] = {} ", i, j, value);
        }
    }*/
}