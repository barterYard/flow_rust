use cadence_json::ValueOwned;
use flow::access::{GetTransactionRequest, PingRequest, SendTransactionRequest};
use flow::proto::{
    access::{
        access_api_client::AccessApiClient, BlockResponse, GetBlockByHeightRequest,
        GetCollectionByIdRequest, GetLatestBlockRequest, GetTransactionsByBlockIdRequest,
    },
    execution::GetTransactionResultRequest,
};
use std::str;

#[tokio::main]
async fn main() {
    let mut client = AccessApiClient::mainnet().await.expect("error");
    // let trans = Transaction {
    //     arguments: todo!(),
    //     authorizers: todo!(),
    //     envelope_signatures: todo!(),
    //     payload_signatures: todo!(),
    //     gas_limit: todo!(),
    //     payer: todo!(),
    //     reference_block_id: todo!(),
    //     script: todo!(),
    //     proposal_key: todo!(),
    // };
    // client
    //     .send_transaction(SendTransactionRequest {
    //         transaction: Some(trans),
    //     })
    //     .await;
    let _ = client.ping(PingRequest {}).await;
    loop {
        let r = match client
            .get_latest_block(GetLatestBlockRequest {
                is_sealed: true,
                full_block_response: true,
            })
            .await
        {
            Ok(x) => x.into_inner(),
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        };
        let block = r.block.unwrap();

        println!(
            "block height: {}, id: {:?}",
            block.height,
            hex::encode(block.id.clone())
        );

        let k = block.id.clone();
        for collection in block.collection_guarantees {
            let col = match client
                .get_collection_by_id(GetCollectionByIdRequest {
                    id: collection.collection_id,
                })
                .await
            {
                Ok(x) => x.into_inner(),
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            };

            for tr_id in col.collection.clone().unwrap().clone().transaction_ids {
                let tran = match client
                    .get_transaction_result(GetTransactionRequest {
                        id: tr_id,
                        block_id: k.clone(),
                        collection_id: col.collection.clone().unwrap().id.clone(),
                    })
                    .await
                {
                    Ok(x) => x.into_inner(),
                    Err(e) => {
                        println!("{:?}", e);
                        continue;
                    }
                };
                for event in tran.events {
                    if let ValueOwned::Event(event_payload) =
                        serde_json::from_slice(&event.payload).unwrap()
                    {
                        println!("{:?}", event_payload.id);
                    }
                }
            }
        }
    }
}
