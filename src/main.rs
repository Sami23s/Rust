use std::io;
fn main() {
    Login();
    Enabling();
    let mut skuska = String::new();
    println!("Zadaj cislo interakcie: ");
    println!("1 - Dátum narodenia");
    println!("2 - Výpis pozície");
    println!("3 - Možnosť 3");


    io::stdin().read_line(&mut skuska).expect("Nepodarilo sa načítať riadok");
    let c: i32 = skuska.trim().parse().expect("Nepodarilo sa previesť na číslo");
    match c{

        1=>vypisZamest(),
        _=>println!("Only one  digit allowed"),
    }

    fn Login(){
        let mut line = String::new();
        println!("Enter your name :");
        let b1 = std::io::stdin().read_line(&mut line);
        println!("Hello , {}", line);

    }


    fn Enabling(){
        let enabled = true;
        let result = if enabled { "Account has been Enabled" } else { "Account Disabled" };

        println!("{}",result);}

    fn vypisZamest(){
        let mut input = String::new();
        println!("Zadaj meno zamestnanca:" );
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        let hladaj = input.trim();




        if hladaj== "Janko"{
            println!("Jankov vek je: {}",JANKO)
        }
        else {
            if  hladaj=="Mirko"{
                println!("Mirkov vek je: {}",MIRKO) }
            else {
                if  hladaj=="Marian" {println!("Marianov vek je: {}",MARIAN)
            }
            else { if hladaj=="Michal" {println!("Michalov vek je: {}",MICHAL)
            } }}
        }
        //mena zamestnancov
        const NAME: &str = "Janko";
        const NAME2: &str = "Mirko";
        const NAME3: &str = "Marian";
        const NAME4: &str = "Michal";

        let zoznam = [NAME, NAME2, NAME3, NAME4];
        //vek zamestnancov
        const JANKO: i32 = 33;
        const MIRKO: i32 = 28;
        const MARIAN: i32 = 49;
        const MICHAL: i32 = 22;

        // zoznam vekov zamestnancov
        let zoznam2 = [JANKO, MIRKO, MARIAN, MICHAL];

        //  for item2 in zoznam2 {
       //     println!("{} is {} years old", item, item2);
      //  }


    }}



