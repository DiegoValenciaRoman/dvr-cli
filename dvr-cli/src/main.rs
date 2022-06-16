use std::env;
use std::io;
struct Task{
    name:String,
    description:String,
    date:String,
    is_done:bool
}
struct Proyect{
    tasks:Vec<Task>,
    name:String,
    tags:Vec<String>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let _proyects: Vec<Proyect> = load_proyects(); 
    execute_command(String::from(&args[1]));
}

fn execute_command(command:String){
    //todo: change allowed_commands to string literals
    let allowed_commands: Vec<String> = Vec::from([String::from("list"),String::from("create")]);
    if allowed_commands.contains(&command){
        match command.as_str() {
            "list" => list_projects(),
            "create" => create_proyect(),
            _ => println!("Command not found")
        }
    }else{
        println!("Command not found");
    }

}
fn create_proyect(){
    //create project with input from the user
    println!("create proyect");
    //read input from user in terminal
    let mut s=String::new();
    print!("Please enter some text: ");
    io::stdin().read_line(&mut s).expect("Did not enter a correct string");

}
fn load_proyects()->Vec<Proyect>{
    //load proyects from file
    println!("load proyects");
    let mut proyects:Vec<Proyect> = Vec::new();
    proyects.push(Proyect{
        tasks:Vec::new(),
        name:String::from("proyect1"),
        tags:Vec::new()
    });
    proyects.push(Proyect{
        tasks:Vec::new(),
        name:String::from("proyect2"),
        tags:Vec::new()
    });
    return proyects;
}
fn list_projects(){
    let demo:Proyect = Proyect{
        tasks:Vec::new(),
        name:String::from("Demo"),
        tags:Vec::new()

    };
    let projects = Vec::from([demo]);
    //iterate and print all projects
    for project in projects{
        println!("{}",project.name);
    }
}