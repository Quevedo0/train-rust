trait Sonido {
    fn hacer_sonido(&self);
}

struct Gato;
struct Grillo;
struct Ballena;

impl Sonido for Gato {
    fn hacer_sonido(&self) {
        let sonido = format!("El Gato dice: {}", "Miau");
        println!("{}", sonido);
    }
}

impl Sonido for Grillo {
    fn hacer_sonido(&self) {
        let sonido = format!("El Grillo dice: {}", "Cri Cri");
        println!("{}", sonido);
    }
}

impl Sonido for Ballena {
    fn hacer_sonido(&self) {
        let sonido = format!("La Ballena dice: {}", "Glub Glub");
        println!("{}", sonido);
    }
}

fn reproducir_sonido<T: Sonido>(animal: T) {
    animal.hacer_sonido();
}

fn main(){
    let gato = Gato;
    let grillo = Grillo;
    let ballena = Ballena;

    println!("{}", format!("Sonido grillo"));
    reproducir_sonido(grillo);

    println!("{}", format!("Sonido gato"));
    reproducir_sonido(gato);

    println!("{}", format!("Sonido ballena"));
    reproducir_sonido(ballena);
}
