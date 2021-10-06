use std::{thread, time};
use database::VSwimDB;

use crate::vatsim::Vatsim;

mod vatsim;
pub mod database;

#[tokio::main]
async fn main() {
    loop {
        let vatsim = Vatsim::new()
            .with_status_url("https://status.vatsim.net/status.json")
            .build().await
            .expect("Couldnt build Vatsim module");
        println!("Current update: {:?} Total Pilots: {:?}",
                 vatsim.data_handler.v3_data.update,
                 vatsim.data_handler.v3_data.pilots.len(),
        );

        let db = VSwimDB::new();
        db.commit_vatsim_data_general(&vatsim.data_handler).await;
        db.commit_vatsim_data_pilots(&vatsim.data_handler).await;

        let sleep_time = time::Duration::from_secs(3600);
        thread::sleep(sleep_time);
    }
}