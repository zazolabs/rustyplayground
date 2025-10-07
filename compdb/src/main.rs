use std::io;
// use std::cmp::Ordering;
use std::collections::HashMap;

fn handle_add(db: &mut HashMap::<String, Vec<String>>, name: &str, deptname: &str) {
    db.entry(deptname.to_string())
        .or_insert(Vec::<String>::new())
        .push(name.to_string());
}

fn comp_directory(db: &HashMap::<String, Vec<String>>) {
    println!("Company Directory List:");
    for (dept, v) in db {
        println!("{dept} Department List:");
        let mut vs = v.to_vec();
        vs.sort();
        for e in vs.iter() {
            println!("{e}");
        }
    }
}

fn list_by_dept(db: &HashMap::<String, Vec<String>>, deptname: &str) {
    let dept = db.get(deptname);
    match dept {
        Some(list) => {
            println!("{deptname} Department List:");
            for e in list {
                println!("{e}");
            }
        },
        None => println!("Invalid Department {deptname}"),
    }
}

fn print_prompt() {
    let main_prompt = "Commands:
        Add <Name> to <Department>
        List <Department>
        Directory
        quit
        ";
    println!("{main_prompt}");
}

fn main() {
    println!("Hello, world!");
    let mut compdb = HashMap::<String,Vec<String>>::new();

    loop {
        let mut command = String::new();

        print_prompt();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        let mut split:Vec<&str>  = command.trim().split(' ').collect();
        println!("split: {:?}", split);

        match split[0] {
             "Add" => handle_add(&mut compdb, split[1], split[3]),
             "List" => list_by_dept(&compdb, split[1]),
             "Dir" => comp_directory(&compdb),
             "quit" => break,
             _=> print_prompt()
        }
        println!("{:?}", compdb);
        // // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => { println!("Invalid number"); continue }
        // };
    }
        println!("{:?}", compdb);

}
