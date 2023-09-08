use flow_rs::access::{GetTransactionRequest, PingRequest, SendTransactionRequest};
use flow_rs::cadence_json::ValueOwned;
use flow_rs::proto::{
    access::{
        access_api_client::AccessApiClient, BlockResponse, GetBlockByHeightRequest,
        GetCollectionByIdRequest, GetLatestBlockRequest, GetTransactionsByBlockIdRequest,
    },
    entities::Event,
    execution::GetTransactionResultRequest,
};
use flow_rs::Channel;
use flow_rs::FlowNetwork;
use std::str;

#[tokio::main]
async fn main() {
    let mut client = AccessApiClient::<Channel>::get_client(
        "http://access-001.mainnet1.nodes.onflow.org:9000".to_string(),
    )
    .await
    .unwrap();

    let mut latest_block_height = 7652359;
    loop {
        let r = match client
            .get_block_by_height(GetBlockByHeightRequest {
                height: latest_block_height,
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
        let block = r.clone().block.unwrap();

        latest_block_height += 1;

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
                    let event_p = event.parse_payload().unwrap();
                    println!("{:?}", event_p.id);
                }
            }
        }
    }
}
