// use crate::aws::{queue::ingress::DdexIngestionQueue, storage::ingress::DdexIngestionStorage};
// use blob_codec::BlobEstimator;
// use log_macros::log_warn;
// use std::{
//     path::Path,
//     sync::{Arc, Mutex},
// };

// pub struct InputFolderBuilder {
//     pub ddex_ingestion_queue: Arc<DdexIngestionQueue>,
//     pub ddex_ingestion_storage: Arc<Mutex<DdexIngestionStorage>>,
//     pub blob_estimator: BlobEstimator,
// }

// impl InputFolderBuilder {
//     pub fn build(
//         ddex_ingestion_queue: &Arc<DdexIngestionQueue>,
//         ddex_ingestion_storage: &Arc<Mutex<DdexIngestionStorage>>,
//     ) -> Self {
//         Self {
//             ddex_ingestion_queue: ddex_ingestion_queue.clone(),
//             ddex_ingestion_storage: ddex_ingestion_storage.clone(),
//             blob_estimator: BlobEstimator::default(),
//         }
//     }
//     pub async fn build_input_folder(&self) -> anyhow::Result<()> {
//         let mut ddex_ingestion_storage = self
//             .ddex_ingestion_storage
//             .lock()
//             .expect("Failed to lock on ddex_ingestion_storage");
//         ddex_ingestion_storage.clear_input_folder()?;

//         let mut added_counter = 0;

//         loop {
//             match self.ddex_ingestion_queue.reserve_message_folder().await? {
//                 Some(s3_message_folder) => {
//                     let local_message_folder = ddex_ingestion_storage
//                         .sync_message_folder(&s3_message_folder)
//                         .await?;
//                     match self
//                         .blob_estimator
//                         .estimate_and_check(Path::new(&ddex_ingestion_storage.input_files_dir))
//                     {
//                         Ok(_) => {
//                             ddex_ingestion_storage
//                                 .local_to_s3_folder_mapping
//                                 .insert(local_message_folder.clone(), s3_message_folder.clone());
//                             added_counter += 1;
//                             ddex_ingestion_storage
//                                 .s3_message_folders
//                                 .push(s3_message_folder)
//                         }
//                         Err(err) => {
//                             log_warn!(err);
//                             std::fs::remove_dir_all(Path::new(&local_message_folder))?;
//                             let mut status_value = String::new();
//                             // just one message exceed the limit - set that one message as rejected
//                             if added_counter == 0 {
//                                 status_value =
//                                     self.ddex_ingestion_queue.rejected_status_value.clone();
//                             } else {
//                                 status_value =
//                                     self.ddex_ingestion_queue.unprocessed_status_value.clone();
//                             }
//                             self.ddex_ingestion_queue
//                                 .set_single_message_folder_status(s3_message_folder, status_value)
//                                 .await?;
//                             break;
//                         }
//                     }
//                 }
//                 None => break,
//             }
//         }
//         Ok(())
//     }
// }
