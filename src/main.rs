fn main() {
    //mena zamestnancov
    const NAME: &str = "Janko";
    const NAME2: &str = "Mirko";
    const NAME3: &str = "Marian";
    const NAME4: &str = "Michal";
    println!("{}", NAME);
 //vek zamestnancov
    const JANKO: i32 = 33;
    const MIRKO: i32 = 28;
    const MARIAN: i32 = 43;
    const MICHAL: i32 = 22;

    // zoznam vekov zamestnancov
    let zoznam = [JANKO, MIRKO, MIRKO, MICHAL];

    for item in zoznam {
        println!("{}",item);
    }


}