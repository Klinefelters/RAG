use ragit::{Index, LoadMode, QueryTurn};
use std::io::Write;

async fn rag() -> Result<(), Box<dyn std::error::Error>> {
    let index = Index::load("./".to_string(), LoadMode::QuickCheck).map_err(|e| format!("Index load error: {:?}", e))?;
    let mut history = vec![];

    loop {
        let mut curr_input = String::new();
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut curr_input).unwrap();
        let response = index.query(&curr_input, history.clone()).await.map_err(|e| format!("Query error: {:?}", e))?;
        println!("{}", response.response);
        if response.response.trim() == "/q" {
            break;
        }
        history.push(QueryTurn::new(curr_input, response));
    }
    Ok(())
}

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(rag()).unwrap();
}
