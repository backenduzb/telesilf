use teloxide::types::FileId;
use teloxide::types::Seconds;

#[derive(Debug, Clone)]
pub enum StoryMedia {
    Photo(FileId),
    Video(FileId),
}

#[derive(Debug, Clone, Default)]
pub struct StoryDraft {
    pub media: Option<StoryMedia>,
    pub caption: Option<String>,
}

impl StoryDraft {
    pub fn clear(&mut self) {
        self.media = None;
        self.caption = None;
    }

    pub fn is_ready(&self) -> bool {
        self.media.is_some()
    }

    pub fn active_period(&self) -> Seconds {
        Seconds::from_seconds(24 * 60 * 60)
    }
}
