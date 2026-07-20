use teloxide::dispatching::UpdateHandler;
use teloxide::prelude::*;

use crate::handlers::business::test;
use crate::handlers::start::{start, Command as StartCommand};

pub fn setup_router() -> UpdateHandler<teloxide::RequestError> {
    dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<StartCommand>()
                .endpoint(start),
        )
        .branch(
            Update::filter_business_message()
                .endpoint(test),
        )
}