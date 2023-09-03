dining-philosophers
===================

I played around with the "Dining Philosophers" concurrency exercise.

There are two key observations:
1. A race is unlikely until delay is added between a philosopher attempting to grab the left and the right fork. 
Once a delay is added, deadlocks are easy to reproduce. 
This makes it much easier to test different solutions.
2. The essence of the problem is that a philosopher requires _both forks_ to eat, but the forks are independently shared by others.

Based on the insight above, the basic solution I came up with is that if a philosopher cannot successfully pick up both forks at the same time, they should put back any forks picked up and wait a bit to try again later. This is the basic idea, extracted from the code:

```rust
    fn eat(&self) {
        let _locks = loop {
            self.print("trying to get both forks...");
            let r1 = self.left_fork.try_lock();
            if r1.is_ok() {
                self.print("got left fork!");
            }
            random_sleep(1, 100);
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
            random_sleep(1, 1000);
        };

        // Below is the naive code which deadlocks. The deadlock is that 
        // all philosophers end up waiting for the right fork which never
        // arrives. You can make the deadlock more or less likely by changing
        // the delay time between taking the two forks.
        //
        // self.print("is waiting left fork...");
        // let _left_fork_guard = self.left_fork.lock().unwrap();
        // random_sleep(1, 1000);
        // self.print("is waiting right fork...");
        // let _right_fork_guard = self.right_fork.lock().unwrap();

        self.print("is eating...");
        random_sleep(10, 20);
        self.print("is returning forks...");
    }
}
```

This solution does resolve the deadlock but introduces latency into the system because of the wait delay for the philosopher to "try again".  In the future I want to experiment with holding onto one of the two forks for a short period of time, in case the other shows up during that window and then the philosopher can immediately begin. And if the timeout is reached, the picked-up fork goes back to the table with a wait, as above. The main difference is that there will be some opportunity to eat faster, if we can wait while holding one of the locks. Unfortunately, as far as I can see, `std::sync::Mutex` doesn't have a way to add a timeout for the `lock()` call. I see that the `parking_log` crate has a `try_lock_for` method that allows a timeout to be provided. I would like to try that.
