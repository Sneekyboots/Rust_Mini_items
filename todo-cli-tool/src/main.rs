use clap::{Parser, Subcommand};
use std::fs;
use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize,Debug)]
struct Task{
    title:String,
    done:bool,
}

#[derive(Parser)]
#[command(name="todo-cli-tool")]
#[command(about="Todo list",long_about=None)]
struct Cli {
    #[command(subcommand)]
    command:Commands,
}

#[derive(Subcommand)]
enum Commands{
    List,
    Add{title:String},
    Done{index:usize},
    Delete{index:usize},
}

fn main (){
    let cli=Cli::parse;
    let mut tasks=load_tasks();
    match &cli().command{
        Commands::Add{title}=>{
            tasks.push(Task{title:title.clone(),done:false});
            save_tasks(&tasks);
            println!("Added:{}",title);
        }
        Commands::List=>{
            for (i,task) in tasks.iter().enumerate(){
                println!("{}:{}[{}]",i+1,task.title,if task.done{"Done"} else {"TBD"});

            }
        }

        Commands::Done{index}=>{
            if  *index>0 && *index<=tasks.len(){
                tasks[*index-1].done=true;
                save_tasks(&tasks);
                 println!("Marked task {} as done.", index);
            } else {
                println!("Invalid task number.");
            }
            
        }
        Commands::Delete { index } => {
            if *index > 0 && *index <= tasks.len() {
                let removed = tasks.remove(*index - 1);
                save_tasks(&tasks);
                println!("Deleted: {}", removed.title);
            } else {
                println!("Invalid task number.");
            }
        }
    }
}
fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string(tasks).unwrap();
    fs::write("tasks.json", data).unwrap();
}

fn load_tasks() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string("tasks.json") {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}