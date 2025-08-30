
#[derive (Debug)]
struct Task{
    title:String,
    done:bool
}

fn main() {
   

let mut tasks=Vec::new();
tasks.push(Task{
    title:String::from("THIS IS MY FIRST TASK"),
    done:false
});
tasks.push(Task{
    title:String::from("THIS IS MY SECOND TASK"),
    done:true
});


for (i,task) in tasks.iter().enumerate(){
    println!("{}:{} [{}]",i+1,task.title,if task.done {"done"} else {"TBD"})
}
}
