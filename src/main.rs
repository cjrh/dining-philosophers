use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    indent: usize,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn print(&self, s: &str) {
        let indent = " ".repeat(self.indent);
        println!("{}{}: {}", indent, &self.name, s);
    }

    fn think(&self) {
        // Create an empty string with `indent` spaces
        self.print("is thinking...");
        random_sleep(1, 10);
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        let _locks = loop {
            self.print("trying to get both forks...");
            let r1 = self.left_fork.try_lock();
            if r1.is_ok() {
                self.print("got left fork!");
            }
            random_sleep(1, 1000);
            let r2 = self.right_fork.try_lock();
            if r2.is_ok() {
                self.print("got right fork!");
            }
            if r1.is_ok() && r2.is_ok() {
                self.print("got both forks!");
                break (r1.unwrap(), r2.unwrap());
            } else {
                if r1.is_ok() {
                    self.print("can't get right fork, returning left fork");
                }
                if r2.is_ok() {
                    self.print("can't get left fork, returning right fork");
                }
                // self.print("can't get both forks, waiting...");
                drop(r1);
                drop(r2);
            }
            random_sleep(100, 101);
        };
        // self.print("is waiting left fork...");
        // let _left_fork_guard = self.left_fork.lock().unwrap();
        // random_sleep(1, 1000);
        // self.print("is waiting right fork...");
        // let _right_fork_guard = self.right_fork.lock().unwrap();
        self.print("is eating...");
        random_sleep(1, 10);
        self.print("is returning forks...");
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Kant"];

fn main() {
    // Create forks
    // let forks = Arc::new(vec![Mutex::new(Fork), Mutex::new(Fork)]);
    let mut forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

    // Clone the first fork into the last position.
    forks.push(forks[0].clone());

    let (tx, rx) = mpsc::channel();
    // Create philosophers
    let philosophers: Vec<_> = PHILOSOPHERS
        .iter()
        .zip(forks.windows(2))
        .enumerate()
        .map(|(i, (name, pair))| {
            let left_fork = pair[0].clone();
            let right_fork = pair[1].clone();
            let philosopher = Philosopher {
                name: name.to_string(),
                indent: i * 30,
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
    thread::scope(|s| {
        for p in philosophers {
            s.spawn(move || {
                for _ in 0..100 {
                    // Make each philosopher think
                    p.think();
                    // Make each philosopher eat
                    p.eat();
                }
            });
        }
    });
    tx.send("Done".to_string()).unwrap();

    println!("Number of forks: {}", forks.len() - 1);

    if std::env::args().any(|arg| arg == "--thoughts") {
        // Output their thoughts
        use std::io::Write;
        loop {
            let thought = rx.recv().unwrap();
            if thought == "Done" {
                break;
            }
            std::io::stdout().write_all(thought.as_bytes()).unwrap();
        }
    }
}

// Utility function to sleep for a random amount of time
fn random_sleep(a: u64, b: u64) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(a..b);
    thread::sleep(Duration::from_millis(sleep_time));
}
