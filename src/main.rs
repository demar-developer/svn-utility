mod libs;

fn main() {
    let version = libs::version();
    println!("SVN version: {}", version);
}