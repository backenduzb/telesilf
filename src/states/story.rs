use teloxide::types::FileId;
use teloxide::types::{
    InputFile, InputStoryContent, InputStoryContentPhoto, InputStoryContentVideo, Seconds,
};

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

    pub fn to_input_story_content(&self) -> Option<InputStoryContent> {
        match &self.media {
            Some(StoryMedia::Photo(file_id)) => {
                Some(InputStoryContent::Photo(InputStoryContentPhoto {
                    photo: InputFile::file_id(file_id.clone()),
                }))
            }
            Some(StoryMedia::Video(file_id)) => {
                Some(InputStoryContent::Video(InputStoryContentVideo {
                    video: InputFile::file_id(file_id.clone()),
                    duration: None,
                    cover_frame_timestamp: None,
                    is_animation: None,
                }))
            }
            None => None,
        }
    }

    pub fn active_period(&self) -> Seconds {
        Seconds::from_seconds(24 * 60 * 60)
    }
}
