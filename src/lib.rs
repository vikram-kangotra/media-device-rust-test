use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {

    let window = web_sys::window()
        .expect("no global `window` exists");
    let navigator = window.navigator();

    let media_devices = navigator.media_devices()
        .expect("no media devices");

    let mut constraint = web_sys::MediaStreamConstraints::new();
    constraint.video(&JsValue::TRUE);
    constraint.audio(&JsValue::FALSE);

    let user_media = JsFuture::from(media_devices
                        .get_user_media_with_constraints(&constraint)?);

    let media_stream = user_media.await?
        .dyn_into::<web_sys::MediaStream>()?;

    let document = window.document()
        .expect("should have a document on window");
    let body = document.body()
        .expect("document should have a body");

    let video = document.create_element("video")?;
    video.set_attribute("autoplay", "true")?;
    video.set_attribute("playsinline", "true")?;

    let video_element = video.dyn_into::<web_sys::HtmlVideoElement>()?;
    video_element.set_src_object(Some(&media_stream));

    body.append_child(&video_element)?;

    Ok(())
}
