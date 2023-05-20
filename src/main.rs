use std::io;
fn main() {
    Enabling();
    Login();

    vypisZamest();

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
        let mut start = String::new();
        let start = start.trim_end();
        println!("Enter name of employee :");
        let b1 = std::io::stdin().read_line(& start);

        if start == "Janko"{
            println!("Jankov vek je: {}",JANKO)
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
        const MARIAN: i32 = 43;
        const MICHAL: i32 = 22;

        // zoznam vekov zamestnancov
        let zoznam2 = [JANKO, MIRKO, MARIAN, MICHAL];

        //  for item2 in zoznam2 {
       //     println!("{} is {} years old", item, item2);
      //  }


    }
}


