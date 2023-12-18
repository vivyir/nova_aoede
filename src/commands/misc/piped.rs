use poise::command;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use reqwest::Error as ReqwestError;

use crate::{
    types::{
        Context,
        MaybeError,
    },
    utils::cut_excess,
};

#[command(
    prefix_command,
    slash_command,
    track_edits,
    broadcast_typing,
    category = "Miscellaneous"
)]
pub async fn piped(
    ctx: Context<'_>, 
    #[description = "The command to get help about. Leave blank if you want a list of all \
                     commands."]
    query: Option<String>,
) -> MaybeError {
    match query {
        Some(q) => {
            let request_url = format!(
                "https://api.piped.yt/streams/{q}"
            );
            println!("{request_url}");
            let response = reqwest::get(&request_url).await?;
            let pipedstream: PipedStream = response.json().await?;
            ctx.say(format!("query: {q}")).await?;
            ctx.say(format!("{:#?}", pipedstream.audio_streams[0])).await?;

            let guild_id = match ctx.guild_id() {
                Some(v) => v,
                None => {
                    ctx.say(format!("This message isn't in a guild.")).await?;
                    return Ok(());
                }
            };

            Ok(())
        },
        None => {
            ctx.say("You didn't provide a fucking link.").await?;
            Ok(())
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipedStream {
    pub title: String,
    pub description: String,
    pub upload_date: String,
    pub uploader: String,
    pub uploader_url: String,
    pub uploader_avatar: String,
    pub thumbnail_url: String,
    pub hls: Option<String>,
    pub dash: Option<String>,
    pub lbry_id: Option<String>,
    pub category: String,
    pub uploader_verified: bool,
    pub duration: i64,
    pub views: i64,
    pub likes: i64,
    pub dislikes: i64,
    pub uploader_subscriber_count: i64,
    pub audio_streams: Vec<AudioStream>,
    pub video_streams: Vec<VideoStream>,
    pub related_streams: Vec<RelatedStream>,
    pub subtitles: Vec<Subtitle>,
    pub livestream: bool,
    pub proxy_url: String,
    pub chapters: Vec<Value>,
    pub preview_frames: Vec<PreviewFrame>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioStream {
    pub url: String,
    pub format: String,
    pub quality: String,
    pub mime_type: String,
    pub codec: String,
    pub audio_track_id: Option<i64>,
    pub audio_track_name: Option<String>,
    pub audio_track_type: Option<String>,
    pub audio_track_locale: Option<String>,
    pub video_only: bool,
    pub itag: i64,
    pub bitrate: i64,
    pub init_start: i64,
    pub init_end: i64,
    pub index_start: i64,
    pub index_end: i64,
    pub width: i64,
    pub height: i64,
    pub fps: i64,
    pub content_length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoStream {
    pub url: String,
    pub format: String,
    pub quality: String,
    pub mime_type: String,
    pub codec: Option<String>,
    pub audio_track_id: Option<i64>,
    pub audio_track_name: Option<String>,
    pub audio_track_type: Option<String>,
    pub audio_track_locale: Option<String>,
    pub video_only: bool,
    pub itag: i64,
    pub bitrate: i64,
    pub init_start: i64,
    pub init_end: i64,
    pub index_start: i64,
    pub index_end: i64,
    pub width: i64,
    pub height: i64,
    pub fps: i64,
    pub content_length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedStream {
    pub url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub thumbnail: String,
    pub uploader_name: String,
    pub uploader_url: String,
    pub uploader_avatar: String,
    pub uploaded_date: String,
    pub short_description: Option<String>,
    pub duration: i64,
    pub views: i64,
    pub uploaded: i64,
    pub uploader_verified: bool,
    pub is_short: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtitle {
    pub url: String,
    pub mime_type: String,
    pub name: String,
    pub code: String,
    pub auto_generated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewFrame {
    pub urls: Vec<String>,
    pub frame_width: i64,
    pub frame_height: i64,
    pub total_count: i64,
    pub duration_per_frame: i64,
    pub frames_per_page_x: i64,
    pub frames_per_page_y: i64,
}
