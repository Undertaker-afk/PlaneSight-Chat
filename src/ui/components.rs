// Component definitions for reusable UI elements

use dioxus::prelude::*;

/// Message component
pub fn Message<'a>(
    cx: Scope<'a>,
    sender: &'a str,
    content: &'a str,
    timestamp: u64,
) -> Element {
    cx.render(rsx! {
        div {
            class: "message-bubble",
            div {
                class: "message-header",
                span { class: "sender", "{sender}" }
                span { class: "timestamp", "{timestamp}" }
            }
            div {
                class: "message-content",
                "{content}"
            }
        }
    })
}

/// Peer list component
pub fn PeerList<'a>(cx: Scope<'a>, peers: &'a Vec<String>) -> Element {
    cx.render(rsx! {
        div {
            class: "peer-list",
            h3 { "Connected Peers" }
            ul {
                for peer in peers.iter() {
                    li {
                        key: "{peer}",
                        "{peer}"
                    }
                }
            }
        }
    })
}
