trait HacerSonido {
    fn hacer_sonido(&self);
}

struct Grillo;
struct Perro;
struct Ballena;
struct Gato;

impl HacerSonido for Perro {
    fn hacer_sonido(&self) {
        println!("Guau Guau");
    }
}

impl HacerSonido for Gato {
    fn hacer_sonido(&self) {
        println!("Miau Miau");
    }
}

impl HacerSonido for Ballena {
    fn hacer_sonido(&self) {
        println!("Glub Glub");
    }
}

impl HacerSonido for Grillo {
    fn hacer_sonido(&self) {
        println!("Cri Cri");
    }
}

fn main() {
    let mi_perro = Perro;
    let mi_gato = Gato;
    let mi_ballena = Ballena;
    let mi_grillo = Grillo;

    mi_perro.hacer_sonido();
    mi_gato.hacer_sonido();
    mi_ballena.hacer_sonido();
    mi_grillo.hacer_sonido();
}
