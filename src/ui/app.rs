use dioxus::prelude::*;

use crate::network::NetworkMode;

/// Main application component
pub fn App(cx: Scope) -> Element {
    let network_mode = use_state(cx, || None::<NetworkMode>);
    let setup_complete = use_state(cx, || false);

    cx.render(rsx! {
        style { include_str!("./styles.css") }
        div {
            class: "app-container",
            
            if network_mode.is_none() && !**setup_complete {
                rsx! { SetupScreen { on_select: move |mode| {
                    network_mode.set(Some(mode));
                    setup_complete.set(true);
                }}}
            } else {
                rsx! { ChatScreen { mode: *network_mode.get().unwrap_or(&NetworkMode::Standard) }}
            }
        }
    })
}

/// Setup screen for first-time mode selection
fn SetupScreen<'a>(cx: Scope<'a>, on_select: EventHandler<'a, NetworkMode>) -> Element {
    cx.render(rsx! {
        div {
            class: "setup-screen",
            
            h1 { "Welcome to PlaneSight Chat" }
            
            p {
                class: "subtitle",
                "Secure P2P chat using the infinite monkey principle"
            }
            
            div {
                class: "mode-selection",
                
                h2 { "Choose your network mode:" }
                
                div {
                    class: "mode-card",
                    onclick: move |_| on_select.call(NetworkMode::Standard),
                    
                    h3 { "🌐 Standard Mode" }
                    p { "Uses Nostr for peer discovery and signaling" }
                    ul {
                        li { "Fast peer discovery" }
                        li { "Public relay network" }
                        li { "End-to-end encrypted messages" }
                    }
                }
                
                div {
                    class: "mode-card",
                    onclick: move |_| on_select.call(NetworkMode::Hidden),
                    
                    h3 { "🔒 Hidden Mode" }
                    p { "Uses Tor for complete anonymity" }
                    ul {
                        li { "Maximum privacy" }
                        li { "Anonymous connections" }
                        li { "Slower but more secure" }
                    }
                    span {
                        class: "note",
                        "Note: Requires Tor feature to be compiled"
                    }
                }
            }
        }
    })
}

/// Main chat screen
fn ChatScreen(cx: Scope, mode: NetworkMode) -> Element {
    let messages = use_state(cx, Vec::<String>::new);
    let input_text = use_state(cx, String::new);
    let peer_count = use_state(cx, || 0usize);

    cx.render(rsx! {
        div {
            class: "chat-screen",
            
            // Header
            div {
                class: "chat-header",
                h1 { "PlaneSight Chat" }
                div {
                    class: "status",
                    span {
                        class: "mode-badge",
                        match mode {
                            NetworkMode::Standard => "🌐 Standard",
                            NetworkMode::Hidden => "🔒 Hidden",
                        }
                    }
                    span {
                        class: "peer-count",
                        "Peers: {peer_count}"
                    }
                }
            }
            
            // Messages area
            div {
                class: "messages-container",
                for (idx, msg) in messages.iter().enumerate() {
                    div {
                        key: "{idx}",
                        class: "message",
                        "{msg}"
                    }
                }
            }
            
            // Input area
            div {
                class: "input-container",
                input {
                    class: "message-input",
                    r#type: "text",
                    placeholder: "Type your message...",
                    value: "{input_text}",
                    oninput: move |evt| input_text.set(evt.value.clone()),
                    onkeypress: move |evt| {
                        if evt.key() == "Enter" && !input_text.is_empty() {
                            let msg = format!("You: {}", input_text.get());
                            messages.modify(|msgs| {
                                let mut new_msgs = msgs.clone();
                                new_msgs.push(msg);
                                new_msgs
                            });
                            input_text.set(String::new());
                        }
                    }
                }
                button {
                    class: "send-button",
                    onclick: move |_| {
                        if !input_text.is_empty() {
                            let msg = format!("You: {}", input_text.get());
                            messages.modify(|msgs| {
                                let mut new_msgs = msgs.clone();
                                new_msgs.push(msg);
                                new_msgs
                            });
                            input_text.set(String::new());
                        }
                    },
                    "Send"
                }
            }
            
            // Info footer
            div {
                class: "info-footer",
                p {
                    "🐒 Messages are transmitted as coordinates in infinite text space"
                }
                p {
                    class: "small",
                    "Only seed and coordinates are exchanged - actual text is derived mathematically"
                }
            }
        }
    })
}
