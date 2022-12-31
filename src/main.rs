mod operations;
use solana_sdk::signer::Signer;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
const URL: &str = "https://api.devnet.solana.com";

fn main() {
    let rpc_client = RpcClient::new(URL);
    let sender_key = operations::new_keypair();
    let receiver_key = Keypair::from_bytes(&[78,191,0,2,163,138,129,56,151,229,72,236,119,17,18,47,52,252,251,220,75,238,231,245,170,123,230,224,166,3,26,7,15,185,50,207,55,49,241,241,101,60,84,179,213,56,221,0,117,76,36,141,63,207,196,223,87,126,116,38,80,199,161,13]).unwrap();

    if let Ok(_) = operations::airdrop(&rpc_client, &sender_key.pubkey(), 1.0) {
        if let Ok(balance) = operations::check_balance(&rpc_client, &sender_key.pubkey()) {
            println!("Sender balance now: {:?}", balance);
        }

        let transfer_amount = 0.5;

        match operations::transfer(&rpc_client, &sender_key, &receiver_key.pubkey(), transfer_amount) {
            Ok(_) => { 
                if let Ok(balance) = operations::check_balance(&rpc_client, &sender_key.pubkey()) {
                    println!("Sender balance after transaction: {:?}", balance);
                }
                if let Ok(balance) = operations::check_balance(&rpc_client, &receiver_key.pubkey()) {
                    println!("Receiver balance after transaction: {:?}", balance);
                }
            },
            Err(err) => println!("Error: {:?}", err),
        }
    } else {
        println!("Failed");
    }
}
