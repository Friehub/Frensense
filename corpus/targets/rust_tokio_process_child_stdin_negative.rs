use tokio::process::Command;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let mut child = Command::new("cat")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(b"hello\n").await.unwrap();
    // SAFE: Explicitly shutting down stdin sends EOF so the child can exit.
    stdin.shutdown().await.unwrap();

    let status = child.wait().await.unwrap();
    println!("exit: {status}");
}
