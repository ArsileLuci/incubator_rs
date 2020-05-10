use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::thread::Builder;
struct NotSendAndNotSync(Rc<u8>);
struct OnlySend(RefCell<u8>);
struct SendAndSync(Arc<Mutex<u8>>);
struct OnlySync<'a>(MutexGuard<'a, u8>);
fn main() {
    let nsyncansend = NotSendAndNotSync(Rc::new(1));
    let onlysend = OnlySend(RefCell::new(1));
    let sendandsync = SendAndSync(Arc::new(Mutex::new(1)));
    let mut mutex = Mutex::new(1);
    let onlysync: OnlySync = OnlySync(mutex.lock().unwrap());
    let builder = thread::Builder::new();
    let t1 = builder
        .spawn(|| {
            //Uncommenting line bellow makes program uncompilable because of Not Sync -> Rc is !Sync
            //let sync_check = &nsyncansend;
            //Uncommenting line bellow makes program uncompilable because of Not Send -> Rc is !Send
            //let send_check = nsyncansend;
            //Uncommenting line bellow makes program uncompilable because of Not Sync -> RefCell is !Sync
            //let sync_check = &onlysend;
            //Uncommenting line bellow won't make program uncompilable because of Send -> RefCell is Send
            //let send_check = onlysend;
            //Uncommenting line bellow won't make program uncompilable because of Sync -> Arc is Sync
            //let sync_check = &sendandsync;
            //Uncommenting line bellow won't make program uncompilable because of Send -> Arc is Send
            //let send_check = sendandsync;
            //let sync_check = &onlysync;
            //Uncommenting line bellow won't make program uncompilable because of Send -> MutexGuard<'_, T> is !Send
            //let send_check = onlysync;
        })
        .unwrap();

    t1.join().unwrap();
}
