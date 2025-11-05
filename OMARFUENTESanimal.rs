trait Emitesonido{
    fn emit_sonifo(&self)
    {
        println!("Hace sonido");
    }

}

struct Grillo;
struct Ballena;
struct Gato;

impl Emitesonido for Grillo{

}

impl Emitesonido for Ballena{

}

impl Emitesonido for Gato{
    fn emit_sonifo(&self)
    {
        println!("Miauuuuu");
    }

}

fn main(){

    let ga= Gato;
    let ba= Ballena;
    let gri= Grillo;


    print!("El gato hace "); ga.emit_sonifo();
    print!("La ballena "); ba.emit_sonifo();
    print!("El grillo "); gri.emit_sonifo();
}