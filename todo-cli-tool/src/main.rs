use std::io;
#[derive(Debug)]
struct Task{
    title:String,
    done:bool
}


fn main(){
   let mut tasks:Vec<Task>=Vec::new();

   loop{
    let mut input=String::new();
    println!("Enter the todo thing u want to add to list ");
    io::stdin().read_line(&mut input).expect("Failed to take human input");

    let input =input.trim();
    if input=="exit"{
        break;
    }

    let mut part= input.splitn(2,' ');
    let command =part.next().unwrap();
    let args=part.next();

    match command {
        "todo"=>{
            if let Some(title)=args{
                tasks.push(Task {title:title.to_string(),done:false});
                println!("Added the task {}",title);

            }
            else{
                    println!("Please provide a task title.");
            }
        },
        "list"=>{
            for (i,task) in tasks.iter().enumerate(){
                println!("{}:{}[{}]",i+1,task.title,if task.done {"Done"}else {"TBD"});
            }
        },
        "done"=>{
        if let Some(num_str)=args{
            if let Ok(num)=num_str.parse::<usize>(){
                if num>0 &&num<=tasks.len(){
                    tasks[num-1].done=true;
                    println!("Marked task {} as done .",tasks[num-1].title);
                }
                else{
                    println!("Please provide a valid number ");
                }
            }
            else{
                println!("This cant be parsed into a number")
            }
        }
        },
        "delete"=>{
            if let Some(num_str)=args{
                if let Ok(num)=num_str.parse::<usize>(){
                    if num>0 &&num<=tasks.len(){
                        let task=tasks.remove(num-1);
                        println!("We have removed this tasks {}",task.title);
                    }
                    else{
                        println!("Error the item was not found ")
                    }
                    
                }
                else{
                    println!("Please provide a valid number.");
                }
            }
        }
        _=>{
            println!("Unknown command :{}",command);
        }
    }
   }
}