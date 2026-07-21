use std::{
    path::{Path, PathBuf},
    sync::Arc,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::states::story::StoryMedia;
use teloxide::{
    ApiError, DownloadError, RequestError,
    net::Download,
    prelude::{Bot, Requester},
    types::{BusinessConnectionId, Seconds},
};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct PreparedStoryContent {
    path: PathBuf,
    media_type: &'static str,
}

impl Drop for PreparedStoryContent {
    fn drop(&mut self) {
        if let Err(error) = std::fs::remove_file(&self.path) {
            if error.kind() != std::io::ErrorKind::NotFound {
                log::warn!("story temp faylini o'chirib bo'lmadi: {}", error);
            }
        }
    }
}

pub async fn prepare_story_content(
    bot: &Bot,
    media: &StoryMedia,
) -> Result<PreparedStoryContent, RequestError> {
    let file_id = match media {
        StoryMedia::Photo(file_id) | StoryMedia::Video(file_id) => file_id,
    };
    let telegram_file = bot.get_file(file_id.clone()).await?;
    let extension = Path::new(&telegram_file.path)
        .extension()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or(match media {
            StoryMedia::Photo(_) => "jpg",
            StoryMedia::Video(_) => "mp4",
        });

    let counter = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "silf-story-{}-{}.{}",
        std::process::id(),
        counter,
        extension
    ));

    let mut destination = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .await
        .map_err(|error| RequestError::Io(Arc::new(error)))?;

    if let Err(error) = bot
        .download_file(&telegram_file.path, &mut destination)
        .await
    {
        drop(destination);
        let _ = fs::remove_file(&path).await;
        return Err(download_error_to_request_error(error));
    }
    destination
        .flush()
        .await
        .map_err(|error| RequestError::Io(Arc::new(error)))?;
    drop(destination);

    let media_type = match media {
        StoryMedia::Photo(_) => "photo",
        StoryMedia::Video(_) => "video",
    };

    Ok(PreparedStoryContent { path, media_type })
}

pub async fn post_story_multipart(
    bot: &Bot,
    business_connection_id: BusinessConnectionId,
    prepared: &PreparedStoryContent,
    active_period: Seconds,
    caption: Option<&str>,
) -> Result<(), RequestError> {
    let mut url = bot.api_url();
    url.path_segments_mut()
        .expect("yuklab bolmadi")
        .push(&format!("bot{}", bot.token()))
        .push("postStory");

    let content = serde_json::json!({
        "type": prepared.media_type,
        prepared.media_type: "attach://story_media",
    });
    let media_part = reqwest::multipart::Part::file(&prepared.path)
        .await
        .map_err(|error| RequestError::Io(Arc::new(error)))?;
    let mut form = reqwest::multipart::Form::new()
        .text("business_connection_id", business_connection_id.0)
        .text("content", content.to_string())
        .text("active_period", active_period.seconds().to_string())
        .part("story_media", media_part);

    if let Some(caption) = caption {
        form = form.text("caption", caption.to_owned());
    }

    let response = bot.client().post(url).multipart(form).send().await?;
    let payload: serde_json::Value = response.json().await?;

    if payload.get("ok").and_then(serde_json::Value::as_bool) != Some(true) {
        let description = payload
            .get("description")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("postStory multipart request failed")
            .to_owned();
        return Err(RequestError::Api(ApiError::Unknown(description)));
    }

    Ok(())
}

fn download_error_to_request_error(error: DownloadError) -> RequestError {
    match error {
        DownloadError::Network(error) => RequestError::Network(error),
        DownloadError::Io(error) => RequestError::Io(error),
    }
}
