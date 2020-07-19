use tokio::process::Command;
use std::borrow::Borrow;

pub async fn create_thumbnail(video_path: String, thumbnail_path: String) -> Result<(), Box<dyn std::error::Error>> {
    //println!("IN create_thumbnail with: {}, {}", video_path, thumbnail_path);
    let args = format!(" -i {} -ss 00:00:00 -vframes 1 {} -y", video_path, thumbnail_path);
    let child = Command::new("ffmpeg")
        .arg("-i")
        .arg(video_path)
        .arg("-ss")
        .arg("00:00:00")
        .arg("-vframes")
        .arg("1")
        .arg(thumbnail_path)
        .arg("-y")
        .spawn();
    let future = child.expect("failed to spawn");
    let status = future.await?;
    //println!("status: {}", status );
    Ok(())

}