use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
struct User {
    id: u32,
    timestamp: i64,
}

fn main() {
    let user: Arc<RwLock<Vec<User>>> = Arc::new(RwLock::new(Vec::new()));

    let is_stop = Arc::new(RwLock::new(false));

    let is_stop_clone1 = Arc::clone(&is_stop);
    let is_stop_clone2 = Arc::clone(&is_stop);
    let is_stop_clone3 = Arc::clone(&is_stop);
    let is_stop_clone4 = Arc::clone(&is_stop);
    let is_stop_clone5 = Arc::clone(&is_stop);

    let user_thread1 = Arc::clone(&user);
    let user_thread2 = Arc::clone(&user);
    let user_thread3 = Arc::clone(&user);
    let user_thread4 = Arc::clone(&user);
    let user_thread5 = Arc::clone(&user);

    let mut userId = 0;

    let t1 = thread::spawn(move || {
        while !*is_stop_clone1.read().unwrap() {
            {
                userId += 1;
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;

                let mut uservec = user_thread1.write().unwrap();
                let tuser = User {
                    id: userId,
                    timestamp,
                };
                println!("(Thread1) Create User = {:?}", tuser);
                uservec.push(tuser);
            }

            thread::sleep(Duration::from_millis(500));
        }
    });

    let t2 = thread::spawn(move || {
        while !*is_stop_clone2.read().unwrap() {
            {
                let uservec = user_thread2.read().unwrap();
                println!("(Thread2) User Length =  {}", uservec.len());
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    let t3 = thread::spawn(move || {
        while !*is_stop_clone3.read().unwrap() {
            {
                let mut uservec = user_thread3.write().unwrap();
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;

                uservec.retain(|user| now - user.timestamp <= 2);
            }
            println!("(Thread3) Clean Data Before 2 Secound Created Here Last");
            thread::sleep(Duration::from_secs(3));
        }
    });

    let t4 = thread::spawn(move || {
        while !*is_stop_clone4.read().unwrap() {
            {
                let mut uservec = user_thread4.read().unwrap();
                println!("(Thread4) Full User Vec = {:?}", *uservec);
            }
            thread::sleep(Duration::from_secs(5));
        }
    });

    let t5 = thread::spawn(move || {
        while !*is_stop_clone5.read().unwrap() {
            {
                let mut uservec = user_thread5.read().unwrap();

                if uservec.len() > 7 {
                    let mut stop = is_stop_clone5.write().unwrap();
                    *stop = true;
                    println!("(Thread5) STOP THE PROGRAM");
                }
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();
    t5.join().unwrap();
}
