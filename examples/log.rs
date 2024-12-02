use engine::Logger; 

fn main() {
    Logger::Info!("Hello world");
    Logger::Warn!("This is a warning");
    Logger::Error!("This is an error");
}
