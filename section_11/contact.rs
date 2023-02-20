use std::collections::HashMap;
use std::io;


fn main() {
    let mut vec_name: Vec<String> = Vec::new();
    let mut vec_phone: Vec<String> = Vec::new();

    save_contact(&mut vec_name, &mut vec_phone);

    let contacts: HashMap<&String, &String> = vec_name.iter().zip(vec_phone.iter()).collect();

    search_contact(contacts);
}

fn save_contact(vec_name: &mut Vec<String>, vec_phone: &mut Vec<String>) {
    let mut name = String::new();
    let mut phone = String::new();
    let mut c: u32 = 0;
    let mut n = String::new(); 

    println!("How many contacts you have?");
    io::stdin().read_line(&mut n).expect("Failed"); 
    let n: u32 = n.trim().parse().expect("Failed");

    while c < n {
        name.clear();
        phone.clear();
        println!("Enter contact name: ");
        io::stdin().read_line(&mut name).expect("Failed");
        let name: String = name.trim().parse().expect("Failed");

        println!("Enter contact phone: ");
        io::stdin().read_line(&mut phone).expect("Failed") ;
        let phone: String = phone.trim().parse().expect("Failed");

        vec_name.push(name);
        vec_phone.push(phone);
        c += 1;
    }
}

fn search_contact(contacts: HashMap<&String, &String>) {
    let mut search = String::new();
    println!("Enter name to search: ");
    io::stdin().read_line(&mut search).expect("Failed");
    let search: String = search.trim().parse().expect("Failed");

    for (key, value) in contacts {
        if *key == search {
            println!("Contact: {} {}", key, value);
        }
    }
}
