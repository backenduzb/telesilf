#[derive(Debug, Clone, Default)]
pub enum State {
    #[default]
    Idle,

    WaitingStoryMedia,
    WaitingStoryCaption,
    WaitingStoryConfirm,
}
