use std::io;
use std::collections::HashMap;

fn main() {

    let mut tasks: HashMap<u32, String> = HashMap::new();
    let mut id_counter = 1;
    
    //Create a loop to keep the program running
    loop {

        println!("1.Create a new task/n 2. View all tasks/n 3. Exit");
        let mut choice= String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Enter todo:");
                let mut todo = String::new();
                io::stdin().read_line(&mut todo).unwrap();
                tasks.insert(id_counter, todo.trim().to_string());
                id_counter += 1;


            }
            "2" => {
                for (id, todo) in &tasks {
                    println!("{}: {}",id,todo)
                }
            }
            "3" => break,
            _=> println!("invalid Choice")
           
        }
        

    }
}
