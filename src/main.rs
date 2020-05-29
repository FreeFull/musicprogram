mod engine;

fn main() {
    let engine = engine::start().unwrap();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
