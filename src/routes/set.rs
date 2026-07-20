use teloxide::prelude::*;
use teloxide::dispatching::UpdateHandler;
use crate::handlers::start::{Command, start};

pub fn setup_router() -> UpdateHandler<teloxide::RequestError> {
    Update::filter_message()
        .filter_command::<Command>()
        .endpoint(start)
}