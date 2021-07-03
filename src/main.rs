use actix::prelude::*;

// ---

struct Task {
    task_id: String,
}

impl Task {
    fn new(task_id: impl Into<String>) -> Self {
        Task {
            task_id: task_id.into(),
        }
    }
}

// ---

/// A WorkerActor represents the whole worker, which can be executing any number of tasks
struct WorkerActor {
    running_tasks: usize,
}

impl Actor for WorkerActor {
    type Context = Context<Self>;
}

impl WorkerActor {
    fn new() -> Self {
        WorkerActor { running_tasks: 0 }
    }
}

impl Handler<StartTaskMsg> for WorkerActor {
    type Result = ();

    fn handle(&mut self, msg: StartTaskMsg, _ctx: &mut Context<Self>) -> Self::Result {
        self.running_tasks += 1;

        let _task_addr = TaskActor::new(msg.0).start();

        // TODO: how to wait for these to stop?

        ()
    }
}

// ---

/// A TaskActor represents the execution of a single task
struct TaskActor {
    task: Task,
}

impl Actor for TaskActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Starting task {}", self.task.task_id);

        // TODO: how to sleep?
    }
}

impl TaskActor {
    fn new(task: Task) -> Self {
        Self { task }
    }
}

// ---

struct StartTaskMsg(Task);

impl Message for StartTaskMsg {
    type Result = ();
}

// ---

#[actix_rt::main]
async fn main() {
    // start new actor
    let addr = WorkerActor::new().start();

    // send message and get future for result
    let task = Task::new("task1");
    let res = addr.send(StartTaskMsg(task)).await;

    // handle() returns tokio handle
    res.unwrap();
    println!("DONE");

    // stop system and exit
    System::current().stop();
}
