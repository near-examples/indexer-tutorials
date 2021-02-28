fn main() {
    let args: Vec<String> = std::env::args().collect();
    let home_dir = std::path::PathBuf::from(near_indexer::get_default_home());

    let command = args.get(1)
        .map(|arg| arg.as_str())
        .expect("You need to provide a command: `init` or `run` as arg");

    match command {
        "init" => {
            let config_args = near_indexer::InitConfigArgs {
                chain_id: Some("localnet".to_string()),
                account_id: None,
                test_seed: None,
                num_shards: 1,
                fast: false,
                genesis: None,
                download: true,
                download_genesis_url: None,
            };
            near_indexer::indexer_init_configs(&home_dir, config_args);
        },
        "run" => {
            let indexer_config = near_indexer::IndexerConfig {
        home_dir: std::path::PathBuf::from(near_indexer::get_default_home()),
        sync_mode: near_indexer::SyncModeEnum::FromInterruption,
        await_for_node_synced: near_indexer::AwaitForNodeSyncedEnum::WaitForFullSync,
    };

    actix::System::builder()
        .stop_on_panic(true)
        .run(move || {
            let indexer = near_indexer::Indexer::new(indexer_config);
            let stream = indexer.streamer();
            actix::spawn(listen_blocks(stream));
        })
        .unwrap();
        }
        _ => panic!("You have to pass `init` or `run` arg"),
    }
}


async fn listen_blocks(mut stream: tokio::sync::mpsc::Receiver<near_indexer::StreamerMessage>) {
    while let Some(streamer_message) = stream.recv().await {
        println!("{}", serde_json::to_value(streamer_message).unwrap());
    }
}
