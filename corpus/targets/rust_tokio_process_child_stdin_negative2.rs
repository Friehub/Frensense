use tokio::process::Command;

#[tokio::main]
async fn main() {
    let output = Command::new("cat")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .arg("-e")
        .spawn()
        .unwrap()
        // SAFE: `wait_with_output` handles stdin lifecycle automatically.
        .wait_with_output()
        .await
        .unwrap();

    println!("exit: {}", output.status);
}
