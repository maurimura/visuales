use std::{
    fs::{read_dir, File},
    path::Path,
};

use nannou::{
    image::{
        gif::{GifEncoder, Repeat},
        open,
    },
    window, App,
};

pub fn save_frame(app: &App, window: window::Id, workspace: &str, title: &str) {
    if let Some(window) = app.window(window) {
        let path = &format!(
            "./{}/images/{}-{:0>3}.png",
            workspace,
            title,
            app.elapsed_frames()
        );
        window.capture_frame(Path::new(path));
    }
}

pub fn create_gif(workspace: &str, title: &str) {
    let dir = read_dir(&Path::new(&format!("{}/images", workspace))).unwrap();
    let frames: Vec<(nannou::image::Frame, String)> = dir
        .filter_map(|file| {
            let file_name = String::from(file.unwrap().file_name().to_str().unwrap());

            if file_name.starts_with(&format!("{}-", title)) {
                let image_buffer = open(Path::new(&format!("{}/images/{}", workspace, file_name)))
                    .unwrap()
                    // .resize(800, 800, nannou::image::imageops::FilterType::Triangle)
                    .to_rgba8();

                let frame = nannou::image::Frame::new(image_buffer);
                Some((frame, file_name))
            } else {
                None
            }
        })
        .collect();
    let file_out = File::create(&format!("{}/images/{}.gif", workspace, title)).unwrap();
    let mut encoder = GifEncoder::new(file_out);
    encoder.set_repeat(Repeat::Infinite).unwrap();
    encoder
        .encode_frames(
            frames
                .iter()
                .map(move |(frame, _)| frame.clone())
                .collect::<Vec<nannou::image::Frame>>(),
        )
        .unwrap();
}
