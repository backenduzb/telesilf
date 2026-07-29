#[derive(Debug, Clone, Default)]
pub enum State {
    #[default]
    Idle,

    WaitingStoryMedia,
    WaitingStoryAction,
    WaitingStoryCaptionDecision,
    WaitingStoryCaption,
    WaitingStoryConfirm,
}
