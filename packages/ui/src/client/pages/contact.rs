use dioxus::prelude::*;
use dioxus_i18n::t;

const CONTACT_FORM_URL: &str = "https://www.samuelasherrivello.com/contact-form/";
const LINKEDIN_URL: &str = "https://SamuelAsherRivello.com/LinkedIn/";
const PROFILE_IMAGE: Asset = asset!("/assets/images/profile/samuel-asher-rivello.png");

#[component]
pub fn Contact() -> Element {
    rsx! {
        main { class: "page-content contact-page",
            section { class: "contact-page__hero",
                div { class: "contact-page__intro",
                    h1 { {t!("contact-title")} }
                    p { {t!("contact-intro")} }

                    section { class: "contact-actions", "aria-label": t!("contact-links"),
                        a {
                            class: "contact-action contact-action--primary",
                            href: CONTACT_FORM_URL,
                            target: "_blank",
                            rel: "noreferrer noopener",
                            span { class: "contact-action__label", {t!("contact-form")} }
                            span { class: "contact-action__detail", {t!("send-direct-message")} }
                        }
                        a {
                            class: "contact-action",
                            href: LINKEDIN_URL,
                            target: "_blank",
                            rel: "noreferrer noopener",
                            span { class: "contact-action__label", {t!("linkedin")} }
                            span { class: "contact-action__detail", {t!("connect-professionally")} }
                        }
                    }
                }

                img {
                    class: "contact-page__portrait",
                    src: PROFILE_IMAGE,
                    width: "330",
                    height: "413",
                    alt: t!("profile-alt"),
                }
            }
        }
    }
}
