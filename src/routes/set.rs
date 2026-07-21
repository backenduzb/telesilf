use teloxide::dispatching::UpdateHandler;
use teloxide::prelude::*;

use crate::handlers::{
    business::connection::save_business_connection_from_connection,
    business::deleted::deleted_business_messages,
    business::start::business_start,
    messages::message_handler,
    start::{Command as StartCommand, start},
};

pub fn setup_router() -> UpdateHandler<teloxide::RequestError> {
    dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<StartCommand>()
                .endpoint(start),
        )
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_business_message().endpoint(business_start))
        .branch(
            Update::filter_business_connection().endpoint(save_business_connection_from_connection),
        )
        .branch(Update::filter_deleted_business_messages().endpoint(deleted_business_messages))
}
