use rusoto_core::{Region, RusotoError};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemOutput, PutItemInput, WriteRequest, PutRequest, BatchWriteItemInput};
use crate::vatsim::vatsim_data_model::{GeneralData, PilotData};
use std::collections::HashMap;
use crate::vatsim::vatsim_api_data::VatsimDataHandler;

pub struct VSwimDB {
    dynamo_db_batch_limit: usize,
    client: DynamoDbClient,
}

impl VSwimDB {
    pub fn new() -> Self {
        Self{
            dynamo_db_batch_limit: 25,
            client: DynamoDbClient::new(Region::UsEast2),
        }
    }

    async fn put_item(&self, input: PutItemInput) -> PutItemOutput {
        self.client.put_item(input).await.expect("Problem putting item")
    }

    pub async fn commit_vatsim_data_general(&self, vatsim_data: &VatsimDataHandler) -> PutItemOutput {
        let item = vatsim_data.v3_data.general.clone().to_dynamo_item().expect("Couldnt convert struct to dynamo item");
        let input = GeneralData::generate_put_item(item);
        self.put_item(input).await
    }

    pub async fn commit_vatsim_data_pilots(&self, vatsim_data: &VatsimDataHandler) {
        let batches: Vec<Vec<PilotData>> = self.batch_data(vatsim_data.v3_data.pilots.clone());
        for batch in batches {
            let mut write_requests: Vec<WriteRequest> = Vec::new();
            for pilot in batch {
                let put_request = PutRequest {
                    item: serde_dynamo::to_item(pilot).expect("Couldnt serialize pilot for batch write")
                };
                let write_request = WriteRequest {
                    delete_request: None,
                    put_request: Some(put_request)
                };
                write_requests.push(write_request);
            }
            let mut request: HashMap<String, Vec<WriteRequest>> = HashMap::new();
            request.insert("vatsim_data_pilots".to_string(), write_requests);
            let pilot_input = BatchWriteItemInput {
                request_items: request.clone(),
                return_consumed_capacity: None,
                return_item_collection_metrics: None
            };

            let res = self.client.batch_write_item(pilot_input).await;
            match res {
                Ok(ref output) => {
                    if let Some(unprocessed_items) = &output.unprocessed_items {
                        if unprocessed_items.len() > 0 {
                            println!("Unprocessed Items: {:?}", unprocessed_items);
                        }
                    }
                },
                Err(error) => {
                    match error {
                        RusotoError::Service(s_error) => {
                            println!("Service error {:?}", s_error);
                        }
                        _ => {println!("Some other error {:?}", error)}
                    }
                }

            }

        }
    }

    fn batch_data<T: Clone>(&self, data: Vec<T>) -> Vec<Vec<T>> {
        let rows = data.len()/self.dynamo_db_batch_limit;

        let mut batches: Vec<Vec<T>> = Vec::new();

        for row in 0..rows {
            let mut batch: Vec<T> = Vec::new();
            for col in 0..self.dynamo_db_batch_limit {
                batch.push(data[(row*self.dynamo_db_batch_limit)+col].clone());
            }
            batches.push(batch);
        }

        batches
    }
}