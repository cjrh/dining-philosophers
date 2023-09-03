use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        println!("{} is thinking...", &self.name);
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        println!("{} is grabbing forks...", &self.name);
        self.left_fork.lock().unwrap();
        self.right_fork.lock().unwrap();
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    // Create forks
    // let forks = Arc::new(vec![Mutex::new(Fork), Mutex::new(Fork)]);
    let forks = (1..PHILOSOPHERS.len() + 2).map(|_| Arc::new(Mutex::new(Fork))).collect::<Vec<_>>();

    let (tx, rx) = mpsc::channel();
    // Create philosophers
    let philosophers: Vec<_> = PHILOSOPHERS
        .iter()
        .zip(forks.windows(2))
        .map(|(name, pair)| {
            let left_fork = pair[0].clone();
            let right_fork = pair[1].clone();
            let philosopher = Philosopher {
                name: name.to_string(),
                left_fork,
                right_fork,
                thoughts: tx.clone(),
            };
            // thread::spawn(move || {
            //     philosopher.think();
            //     philosopher.eat();
            // });
            philosopher
        })
        .collect();

    // Make each of them think and eat 100 times
    for _ in 0..10 {
        for p in philosophers.iter() {
            p.think();
            p.eat();
        }
        // Make each philosopher think

        // Make each philosopher eat
    }
    tx.send("Done".to_string()).unwrap();

    // Output their thoughts
    loop {
        let thought = rx.recv().unwrap();
        if thought == "Done" {
            break;
        }
        println!("{}", rx.recv().unwrap());
    }
}
