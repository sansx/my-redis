use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut buffer = File::create("foo1.txt").await?;

    buffer.write_all(b"some bytes").await?;
    Ok(())
}