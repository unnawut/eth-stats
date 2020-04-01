extern crate reqwest;
extern crate web3;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use web3::futures::Future;
use web3::types::BlockId;

const DEFAULT_NUM_BLOCKS: usize = 5;

#[derive(Deserialize, Debug)]
struct BlockNumber {
    jsonrpc: String,
    id: String,
    result: String
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let num_blocks = match env::var("NUM_BLOCKS") {
        Ok(val) => val.parse().unwrap(),
        Err(_) => DEFAULT_NUM_BLOCKS,
    };

    let mut block_timestamps: Vec<usize> = vec![];
    let mut blocktimes: Vec<usize> = vec![];

    let request_url = match env::var("ETH_CLIENT_URL") {
        Ok(val) => val,
        Err(_) => "http://localhost:8545".to_string(),
    };

    let (_eloop, transport) = web3::transports::Http::new(&request_url).unwrap();
    let web3 = web3::Web3::new(transport);

    let latest_blocknum = web3.eth().block_number().wait().unwrap();
    println!("Latest height: {:?}\n", latest_blocknum);

    for n in 0..num_blocks {
        let blocknum = latest_blocknum - n;
        let block_data = web3.eth().block(BlockId::from(blocknum)).wait().unwrap();
        let timestamp = block_data.unwrap().timestamp.as_usize();

        block_timestamps.push(timestamp);

        println!("Block #{:?} has timestamp {:?}", blocknum, timestamp);
    }

    for n in 0..num_blocks - 1 {
        let later_block = block_timestamps[n];
        let earlier_block = block_timestamps[n + 1];

        blocktimes.push(later_block - earlier_block);
    }

    println!("\nBlock times: {:?}\n", blocktimes);

    let total_blocktime = blocktimes.iter().sum::<usize>();
    println!("Max blocktime: {:?}s", blocktimes.iter().max().unwrap());
    println!("Min blocktime: {:?}s", blocktimes.iter().min().unwrap());
    println!("Avg blocktime: {:?}s", total_blocktime as f32 / blocktimes.len() as f32);
    println!("Total blocktime: {:?}s ({:?}mins)", total_blocktime, total_blocktime as f32 / 60.0);

    Ok(())
}

