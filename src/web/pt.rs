use tokio::io::AsyncWriteExt;

async fn count_non_empty_lines<R: tokio::io::AsyncBufRead + Unpin>(
    reader: &mut R
) -> Result<usize, usize> {
    let mut lines = reader.lines();
    let mut count = 0;
    while let Some(line) = lines.next_line().await {
        if !line.empty() {
            count += 1;
        }
    }
    Ok(count)
}


async fn good_handler() -> String {
    let data = tokio::task::spwan_blocking(|| {
        std::fs::read_to_string("big_file.txt").unwrap()
    }).await.unwarp();
}