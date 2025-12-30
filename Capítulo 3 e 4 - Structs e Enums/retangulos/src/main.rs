#[derive(Debug)] // print para debug!
struct Retangulo {
    largura: u32,
    altura: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Retangulo {
        largura: dbg!(30 * scale),
        altura: 50,
    };

    let area = area(&rect1);

    println!("O retângulo {rect1:#?} tem área {area} m²");
}

fn area(retangulo: &Retangulo) -> u32 {
    retangulo.largura * retangulo.altura
}
