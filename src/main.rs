use clap::{Parser, ValueEnum};
use reqwest::{Error, Client};
use serde::{Serialize, Deserialize};
use serde_json::json;

use std::fs::File;
use std::io::prelude::*;

#[derive(Parser)]
#[command(name = "txview")]
#[command(version = "0.1.0")]
#[command(author = "Phantola")]
#[command(about = "txv is simple CLI tool to view transaction details\n\n
If you want to use this tool for ethereum compatible chain (except oasys),\n
you should have infura api key, and save a key in $HOME/.config/txview/config (file)\n
You can get it from <https://infura.io/>")]
struct Cli {
    /// Select the chain to search
    #[arg(value_enum)]
    chain_name: SupportChain,

    /// The transaction hash to search. It should be a 0x-prefixed hex string
    tx_hash : String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum SupportChain {
    /// Ethereum main-net
    EthMainnet,

    /// Ethereum test-net
    EthGoerli,

    /// Ethereum test-net
    EthSepolia,

    /// Linea main-net
    LineaMainnet,

    /// Linea test-net
    LineaGoerli,

    /// Oasys sandverse test-net
    OasSandverse,
}

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
// enum RepresentOption {
//     /// Show all information
//     All,

//     /// Show only block information
//     Block,

//     /// Show only transaction information
//     Transaction,

//     /// Show only log information
//     Log,
// }


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();

    println!("===========================================================================");
    println!("Requested Transaction Information\n");
    println!("Chain  : {:?}\nTxHash : {}", args.chain_name, args.tx_hash);
    println!("===========================================================================");

    

    match args.chain_name {
        SupportChain::EthMainnet | 
        SupportChain::EthGoerli | 
        SupportChain::EthSepolia | 
        SupportChain::LineaMainnet | 
        SupportChain::LineaGoerli => {
            let homedir = std::env::var("HOME");
            let home_dir_value= match homedir {
                Ok(homedir) => homedir,
                Err(e) => {println!("Error: {}", e); return Ok(());}
            };

            let file_path = format!("{}/.config/txview/config", home_dir_value);
            let mut fd = File::open(&file_path).expect("Unable to open config file"); 
            let mut infura_api_key = String::new();
            fd.read_to_string(&mut infura_api_key).expect("Unable to read config file");

            let request_url = get_request_url(args.chain_name, infura_api_key);

            let rpc_response = get_transaction_info_infura(&request_url, &args.tx_hash).await;
            let rpc_response_raw;

            let parsed_rpc_response: RpcResponse = match rpc_response {
                Ok(rpc_response) => {
                    rpc_response_raw = rpc_response.clone();
                    serde_json::from_str::<RpcResponse>(&rpc_response).unwrap()
                }
                Err(e) => {
                    println!("Error: {}", e);
                    return Ok(());
                }
            };

            print_transaction_info_infura(parsed_rpc_response, rpc_response_raw);
        },
        SupportChain::OasSandverse => {
            let request_url = get_request_url(args.chain_name, String::new());
            let rpc_response = get_transaction_info_oasys(&request_url, &args.tx_hash).await;
            let rpc_response_raw;

            let parsed_rpc_response: OasysRpcResponse = match rpc_response {
                Ok(rpc_response) => {
                    rpc_response_raw = rpc_response.clone();
                    serde_json::from_str::<OasysRpcResponse>(&rpc_response).unwrap()
                }
                Err(e) => {
                    println!("Error: {}", e);
                    return Ok(());
                }
            };

            print_transaction_info_oasys(parsed_rpc_response, rpc_response_raw);
        },
    }
    

    Ok(())
}

fn get_request_url(chain_name : SupportChain, api_key : String) -> String {
    match chain_name {
        SupportChain::EthMainnet => {
            return format!("https://mainnet.infura.io/v3/{}", api_key);
        },
        SupportChain::EthGoerli => {
            return format!("https://goerli.infura.io/v3/{}", api_key);
        },
        SupportChain::EthSepolia => {
            return format!("https://sepolia.infura.io/v3/{}", api_key);
        },
        SupportChain::LineaMainnet => {

            return format!("https://linea-mainnet.infura.io/v3/{}", api_key);
        },
        SupportChain::LineaGoerli => {
            return format!("https://linea-goerli.infura.io/v3/{}", api_key);
        },
        SupportChain::OasSandverse=> {
            return String::from("https://explorer.sandverse.oasys.games/api");
        },
    }
}

// Using Infura
#[derive(Debug, Serialize, Deserialize)]
struct JSONRpcBody {
    jsonrpc : String,
    method : String,
    params : (String),
    id : usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Log {
    address: String,
    blockHash: String,
    blockNumber: String,
    data: String,
    logIndex: String,
    removed: bool,
    topics: Vec<String>,
    transactionHash: String,
    transactionIndex: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    blockHash: String,
    blockNumber: String,
    contractAddress: Option<String>,
    cumulativeGasUsed: String,
    effectiveGasPrice: String,
    from: String,
    gasUsed: String,
    logs: Vec<Log>,
    logsBloom: String,
    status: String,
    to: String,
    transactionHash: String,
    transactionIndex: String,
    r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RpcResponse {
    jsonrpc: String,
    id: u32,
    result: Transaction,
}

async fn get_transaction_info_infura(url : &String, tx_hash : &str) -> Result<String, Error> {

    let client = Client::new();

    let request_body = json!({
        "jsonrpc": "2.0",
        "method": "eth_getTransactionReceipt",
        "params": [tx_hash],
        "id": 1
    });


    let request_body_string = serde_json::to_string(&request_body).unwrap();

    let res = client
    .post(url)
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .body(request_body_string)
    .send()
    .await?;

    Ok(res.text().await?)
}

fn print_transaction_info_infura(response : RpcResponse, raw_response : String) {

    println!("\n");
    println!("===========================  Block Information  ===========================");

    let block_hash = &response.result.blockHash[..];
    println!("Block hash : {:?}", block_hash.replace("\"", ""));

    let block_number = &response.result.blockNumber[2..];

    if let Ok(block_number_int) = u64::from_str_radix(block_number, 16) {
        println!("Block Number (Hex/Dec) : {:?} / {:?}", block_number, block_number_int);
    } else {
        println!("Block Number (Hex) : {:?}", block_number);
    }

    println!();
    println!("========================  Transaction Information  ========================");

    let status = &response.result.status[..];
    if status == "0x1" {
        println!("Status : Success ({})", status);
    } else {
        println!("Status : Fail ({})", status);
    }


    let transaction_hash = &response.result.transactionHash[..];
    println!("Transaction hash : {:?}", transaction_hash.replace("\"", ""));
    println!("Transaction Index : {:?}", response.result.transactionIndex);

    let from_address = &response.result.from[..];
    println!("From address : {:?}", from_address.replace("\"", ""));
    let to_address = &response.result.to[..];
    println!("To address : {:?}", to_address.replace("\"", ""));

    let gas_used = &response.result.gasUsed[2..];
    if let Ok(gas_used_int) = u64::from_str_radix(gas_used, 16) {
        println!("Gas Used (Hex/Dec) : {:?} / {:?}", gas_used, gas_used_int);
    } else {
        println!("Gas Used (Hex) : {:?}", gas_used);
    }

    let cumulative_gas_used = &response.result.cumulativeGasUsed[2..];
    if let Ok(cumulative_gas_used_int) = u64::from_str_radix(cumulative_gas_used, 16) {
        println!("Cumulative Gas Used (Hex/Dec) : {:?} / {:?}", cumulative_gas_used, cumulative_gas_used_int);
    } else {
        println!("Cumulative Gas Used (Hex) : {:?}", cumulative_gas_used);
    }

    let effective_gas_price = &response.result.effectiveGasPrice[2..];

    if let Ok(effective_gas_price_int) = u64::from_str_radix(effective_gas_price, 16) {
        println!("Effective Gas Price (Hex/Dec) : {:?} / {:?}", effective_gas_price, effective_gas_price_int);
    } else {
        println!("Effective Gas Price (Hex) : {:?}", effective_gas_price);
    }

    // contract address
    if let Some(contract_address) = &response.result.contractAddress {
        println!("Contract Address : {:?}", contract_address.replace("\"", ""));
    }

    println!();
    println!("============================  Log Information  ============================\n");

    let mut log_index = 0;
    for log in response.result.logs {
        println!("Log Index : {:?}", log_index);
        println!("Address : {:?}", &log.address[..]);
        println!("Block Hash : {:?}", &log.blockHash[..]);
        println!("Block Number : {:?}", &log.blockNumber[..]);
        println!("Data : {:?}", &log.data[..]);
        println!("Log Index : {:?}", &log.logIndex[..]);
        println!("Removed : {:?}", &log.removed);
        println!("Topics : ");
        // iterate topic with index
        for index
            in 0..log.topics.len() {
            println!("\tTopic {} : {:?}", index, &log.topics[index]);
        }

        println!();
        println!("Transaction Hash : {:?}", &log.transactionHash[..]);
        println!("Transaction Index : {:?}", &log.transactionIndex[..]);
        println!("\n");
        log_index += 1;
    }

    println!("logsBloom : {:?}", &response.result.logsBloom[..]);

    println!();
    println!("==============================  Raw Response  ==============================");
    println!("{:?}", raw_response); 
}


// Using Oasis
#[derive(Debug, Serialize, Deserialize)]
struct OasysLog {
    address : String,
    data : String,
    topics: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OasysTransaction {
    revertReason : String,
    blockNumber : String,
    confirmations : String,
    from : String,
    gasLimit : String,
    gasPrice : String,
    gasUsed : String,
    hash : String,
    input : String,
    logs : Vec<OasysLog>,
    success : bool,
    timeStamp : String,
    to : String,
    value : String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OasysRpcResponse {
    result : OasysTransaction,
    status : String,
}

async fn get_transaction_info_oasys(url : &String, tx_hash : &str) -> Result<String, Error> {

    let client = Client::new();
    let res = client
    .get(url)
    .query(&[("module", "transaction"), ("action", "gettxinfo"), ("txhash", tx_hash)])
    .send()
    .await?;

    Ok(res.text().await?)
}

fn print_transaction_info_oasys(response : OasysRpcResponse, raw_response : String) {
    
        println!("\n");
        println!("===========================  Block Information  ===========================");
    
        let block_number = &response.result.blockNumber[..];
        println!("Block Number : {:?}", block_number);
    
        println!();
        println!("========================  Transaction Information  ========================");

        let status = &response.result.success;
        if status == &true {
            println!("Status : Success ({})", status);
        } else {
            println!("Status : Fail ({})", status);
        }
    
        let transaction_hash = &response.result.hash[..];
        println!("Transaction hash : {:?}", transaction_hash.replace("\"", ""));
        println!("Transaction Index : {:?}", response.result.blockNumber);
    
        let from_address = &response.result.from[..];
        println!("From address : {:?}", from_address.replace("\"", ""));
        let to_address = &response.result.to[..];
        println!("To address : {:?}", to_address.replace("\"", ""));
    
        let gas_used = &response.result.gasUsed[..];
        if let Ok(gas_used_int) = gas_used.parse::<u64>() {
            let hex_string = format!("{:X}", gas_used_int);
            println!("Gas Used (Hex/Dec) : {:?} / {:?}", hex_string, gas_used);
        } else {
            println!("Gas Used (Hex) : {:?}", gas_used);
        }
    
        let gas_limit = &response.result.gasLimit[..];
        if let Ok(gas_limit_int) = gas_limit.parse::<u64>() {
            let hex_string = format!("{:X}", gas_limit_int);
            println!("Gas Limit (Hex/Dec) : {:?} / {:?}", hex_string, gas_limit);
        } else {
            println!("Gas Limit (Hex) : {:?}", gas_limit);
        } 
    
        let gas_price = &response.result.gasPrice[..];
        if let Ok(gas_price_int) = gas_price.parse::<u64>() {
            let hex_string = format!("{:X}", gas_price_int);
            println!("Gas Price (Hex/Dec) : {:?} / {:?}", hex_string, gas_price);
        } else {
            println!("Gas Price (Hex) : {:?}", gas_price);
        }
    
        let value = &response.result.value[..];
        if let Ok(value_int) = value.parse::<u64>() {
            let hex_string = format!("{:X}", value_int);
            println!("Value (Hex/Dec) : {:?} / {:?}", hex_string, value);
        } else {
            println!("Value (Hex) : {:?}", value);
        } 
    
        println!();
        println!("============================  Log Information  ============================\n");
        for index in 0..response.result.logs.len() {
            println!("Log {}", index);
            let log = &response.result.logs[index];

            println!("Address : {:?}", &log.address[..]);
            println!("Data : {:?}", &log.data[..]);
            println!("Topics : ");
            // iterate topic with index
            for index
                in 0..log.topics.len() {
                    match &log.topics[index] {
                        Some(topic) => println!("\tTopic {} : {:?}", index, topic),
                        None => println!("\tTopic {} : None", index),
                    }
            }
            println!();
        } 

        println!();
        println!("==============================  Raw Response  ==============================");
        println!("{:?}", raw_response);
}