use std::io::stdin;
mod todo_list;

fn main() {
    println!("Welcome to TODO List:");
    let mut list = todo_list::TodoList::new();
    let mut error_msg = String::new();

    loop {
        print!("\x1B[2J\x1B[1;1H");
        if !error_msg.is_empty() {
            println!("{}\n", error_msg)
        }

        println!("{}\n\n", list);
        println!("Choose an option:");
        println!("[1] Add item to list");
        println!("[2] Check/Uncheck item");
        println!("[3] Remove item");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                let mut name = String::new();
                let mut desc = String::new();
                println!("Enter the name of the item: ");
                stdin().read_line(&mut name).unwrap();
                name = name.trim().to_string();
                println!("Enter its description: ");
                stdin().read_line(&mut desc).unwrap();
                desc = desc.trim().to_string();
                list.add_item(todo_list::TodoItem::new(name, desc, false));
            }
            "2" => {
                let mut name = String::new();
                println!("Enter the name of the item: ");
                stdin().read_line(&mut name).unwrap();
                name = name.trim().to_string();
                list.toggle_completed(name);
            }
            "3" => {
                let mut name = String::new();
                println!("Enter the name of the item: ");
                stdin().read_line(&mut name).unwrap();
                name = name.trim().to_string();
                list.remove_item(name);
            }
            _ => {
                error_msg = String::from("Please enter a valid choice.");
            }
        }
    }
}
