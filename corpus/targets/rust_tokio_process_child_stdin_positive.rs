// [frensense]
// observation: `tokio::process::ChildStdin` is dropped without being explicitly awaited or closed. Dropping `ChildStdin` closes the pipe, but the child process may block waiting for input on stdin if it expects more data.
// impact: The child process hangs indefinitely waiting for stdin, never producing output or exiting. This leaks OS processes and can accumulate zombie/orphaned processes over time, exhausting system process tables.
// improvement: Explicitly `drop` or `shutdown` the child's stdin when done, and use `wait()` or `wait_with_output()` to reap the child.

use tokio::process::Command;

#[tokio::main]
async fn main() {
    let mut child = Command::new("cat")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .spawn()
        .unwrap();

    let stdin = child.stdin.take().unwrap();
    // Write some data but never close stdin
    stdin.try_write(b"hello\n").ok();

    // Child may hang waiting for EOF on stdin
    let status = child.wait().await.unwrap();
    println!("exit: {status}");
}
