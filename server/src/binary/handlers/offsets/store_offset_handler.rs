use crate::binary::sender::Sender;
use anyhow::Result;
use sdk::error::Error;
use sdk::offsets::store_offset::StoreOffset;
use std::sync::Arc;
use streaming::system::System;
use tokio::sync::RwLock;
use tracing::trace;

pub async fn handle(
    command: StoreOffset,
    sender: &mut dyn Sender,
    system: Arc<RwLock<System>>,
) -> Result<(), Error> {
    trace!("{}", command);
    let system = system.read().await;
    system
        .get_stream(command.stream_id)?
        .get_topic(command.topic_id)?
        .store_offset(command.consumer_id, command.partition_id, command.offset)
        .await?;

    sender.send_empty_ok_response().await?;
    Ok(())
}
