use crate::binary::mapper;
use crate::binary::sender::Sender;
use anyhow::Result;
use sdk::error::Error;
use sdk::offsets::get_offset::GetOffset;
use std::sync::Arc;
use streaming::system::System;
use tokio::sync::RwLock;
use tracing::trace;

pub async fn handle(
    command: GetOffset,
    sender: &mut dyn Sender,
    system: Arc<RwLock<System>>,
) -> Result<(), Error> {
    trace!("{}", command);
    let system = system.read().await;
    let offset = system
        .get_stream(command.stream_id)?
        .get_topic(command.topic_id)?
        .get_offset(command.consumer_id, command.partition_id)
        .await?;
    let offset = mapper::map_offset(command.consumer_id, offset);
    sender.send_ok_response(&offset).await?;
    Ok(())
}
