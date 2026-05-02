use dioxus::prelude::*;

const VIDEO_CSS: Asset = asset!("/assets/styling/video.css");

#[component]
pub fn Video() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: VIDEO_CSS }

        main { class: "video-page",
            div { class: "video-page__frame",
                iframe {
                    class: "video-page__embed",
                    src: "https://www.youtube.com/embed/AYzdduCllVw",
                    title: "YouTube video player",
                    allow: "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share",
                    allowfullscreen: "true",
                }
            }
        }
    }
}
