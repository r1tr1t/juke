use std::usize;

/// Core UI functionality and components
use leptos::prelude::*;
use leptos::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use leptos_router::path;
use logging::log;
use serde::Deserialize;
use web_sys::console;
use web_sys::js_sys::Math::log;

// mount application to body of index.html
fn main() {
    console_error_panic_hook::set_once();
    mount::mount_to_body(App);
}

// primary app component
#[component]
fn App() -> impl IntoView {
    // define playlists from playlists.json
    let playlists: Vec<Playlist> =
        serde_json::from_str(include_str!("playlists.json")).expect("failed to parse json");
    // add index as playlist id and map playlists to signals
    let playlists: Vec<_> = playlists
        .into_iter()
        .enumerate()
        .map(|(index, playlist)| (index, signal(playlist)))
        .collect();
    provide_context(playlists);
    // define sample feed
    let mut feed: Vec<FeedEvent> = Vec::new();
    feed.push(FeedEvent::new(
        FeedAction::FriendRequest,
        "Garrett Jones".to_owned(),
        FeedTarget::None,
    ));
    feed.push(FeedEvent::new(
        FeedAction::Comment("Try clicking the like button on a playlist!".to_owned()),
        "Garrett Jones".to_owned(),
        FeedTarget::Album("Sample Feed Album".to_owned()),
    ));
    provide_context(signal(feed));
    let base = document()
        .base_uri()
        .expect("Expected baseURI")
        .expect("Expected baseURI pt2");
    view! {
        <Router>
            <header id="top">
                <Top />
            </header>
            <main>
                <nav id="left" class="panel">
                    <Left />
                </nav>
                <section id="content" class="panel">
                    <Routes fallback=|| "Not found.">
                        // router paths
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/playlists/:id") view=Playlist />
                    </Routes>
                </section>
                <section id="right" class="panel">
                    <Right />
                </section>
            </main>
        </Router>
    }
}

// Homepage component
#[component]
fn Home() -> impl IntoView {
    view! {
        <h2 id="title">Welcome to jukeboxd!</h2>
        <hr />
        <p class="big-p">
            "This project represents a very barebones wireframe of an idea that I had almost a year ago and have always wanted to create. For a long time, I've felt like music streaming services (Spotify in particular) were failing to capitalize on the potential to implement some really cool social elements into their client. Both Spotify and Apple Music have a feature that allows you to find other users, follow them and save their playlists, but don't go much further than that. I wanted to create an application that expands on that capability by:"
        </p>
        <ul class="big-p">
            <li>
                "Creating a toggle that allows other users to publicly like or comment on both users playlists and artist albums and collections"
            </li>
            <li>
                "Giving users a feed to see activity from friends (new playlists, likes on playlists, comments on playlists, albums or collections, etc.)"
            </li>
            <li>
                "A direct messaging service built in to allow for easier sharing of music and podcasts"
            </li>
            <li>
                "The ability to collaborate on playlists with and seamlessly share music and/or podcasts with users on other platforms"
            </li>
        </ul>
        <p class="big-p">
            "Additionally, I wanted to build the entire app using as much Rust as possible. Rust is by far my favorite language, and I honestly believe that it will eventually become the future of programming, primarily due to the way that the compiler forces you to write better code from start to finish, making many lazy programming habits impossible. As Rust can be compiled to WASM, this means that even application that run in the browser (including Electron apps and all variants), can be written almost entirely in Rust, resulting in a safer, faster application than one written in JavaScript. After browsing through and trying a few different Frontend Rust frameworks, I settled on Leptos due to it's mind-boggling speed, reactive signal system and the fact that it uses proc macros instead of requiring ridiculous amounts of boilerplate, like many other Rust frameworks do."
        </p>
        <p class="big-p">
            "If that sounds like a daunting task, you'd be absolutely correct. In fact, that's the main reason that I've been putting off working on this idea for so long. My programming background is primarily backend and data analytics, so the learning curve for Leptos was incredibly steep for me. As a result, this project is merely a wireframe of what the final product might eventually look like, as I haven't even gotten around to writing most of the business logic yet. In the end, it should be able to run a fully featured Spotify or Apple Music natively on any platform that comes with a web browser."
        </p>
        <p class="big-p">
            "For now, poke around with the playlists on the left side, and watch in awe as the feed on the right updates in real time as there are absolutely zero API requests being made :D. This application is running in Client-Side Rendering mode, meaning all DOM manipulations are being controlled by WASM service workers running in your browser!"
        </p>
    }
}

// parameters for playlist struct
#[derive(Params, PartialEq)]
struct PlaylistParams {
    id: usize,
}

// playlist viewer component (will eventually be used to view playlists, albums and other collections)
#[component]
fn Playlist() -> impl IntoView {
    // get parameter from url
    let params = use_params::<PlaylistParams>();
    // derived signal tracking parameter changes
    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .map(|params| params.id)
            .unwrap_or(usize::MAX)
    };
    // get playlists from context
    let playlists: Vec<(usize, (ReadSignal<Playlist>, WriteSignal<Playlist>))> =
        use_context().expect("Playlist content expected");
    // setup signals for like button
    let (like, set_like) = signal(false);
    let (style, set_style) = signal("like");
    // get feed from context
    let (feed, set_feed): (ReadSignal<Vec<FeedEvent>>, WriteSignal<Vec<FeedEvent>>) =
        use_context().expect("Feed context expected");
    // render the current playlist
    let playlist = move || {
        // get current playlist
        let current = playlists[id()].1 .0.get();
        let title = current.title.clone();
        let author = current.author.clone();
        // toggle like button
        let toggle_like = move |_| {
            // set_like(true);
            if like() {
                set_like(false);
                set_style("like");
            } else {
                set_like(true);
                set_style("like_filled");
                // update the feed when the like button is clicked
                set_feed.write().push(FeedEvent::new(
                    FeedAction::Like,
                    "You".to_owned(),
                    FeedTarget::Playlist {
                        title: title.clone(),
                        author: author.clone(),
                    },
                ));
            }
        };
        view! {
            <div class="playlist-info-wrapper">
                <h2 id="title">{current.title}</h2>
                <div id="author">Playlist by {current.author}</div>
                <div id="description">{current.description}</div>
                <button
                    class="like"
                    on:click=toggle_like
                    style:background-image=format!("url('/images/{}.svg')", style())
                ></button>
            </div>
            <hr />
            <ol id="playlist-songs">
                {current
                    .songs
                    .into_iter()
                    .enumerate()
                    .map(|(index, song)| {
                        view! {
                            <li class="song">
                                <div id="enumerate">{index + 1}</div>
                                <div id="song-title">{song.title}</div>
                                <div id="author">{song.author}</div>
                                <div id="release">{song.release}</div>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        }
    };
    playlist
}

// top bar component
#[component]
fn Top() -> impl IntoView {
    view! {
        <a href="/" class="logo"></a>
        <h1 id="title">
            <a href="/">jukeboxd</a>
        </h1>
    }
}

// left panel component
#[component]
fn Left() -> impl IntoView {
    view! {
        <h2>Library</h2>
        <hr />
        <Library />
    }
}

// library component, used in left panel
#[component]
fn Library() -> impl IntoView {
    let playlists: Vec<(usize, (ReadSignal<Playlist>, WriteSignal<Playlist>))> =
        use_context().expect("Playlist context expected");
    // map playlists to html elements
    let library_item = move |(index, (playlist, set_playlist)): (
        usize,
        (ReadSignal<Playlist>, WriteSignal<Playlist>),
    )| {
        // clone value out of read signal
        let playlist = playlist();
        view! {
            <li class="playlist">
                <a href=format!("/playlists/{}", index) id="title">
                    {playlist.title}
                </a>
                <div id="author">Author: {playlist.author}</div>
                <div id="length">Tracks: {playlist.songs.len()}</div>
            </li>
        }
    };
    // render all playlists
    view! { <ul id="library-content">{playlists.into_iter().map(library_item).collect_view()}</ul> }
}

// right side panel component
#[component]
fn Right() -> impl IntoView {
    let (feed, set_feed): (ReadSignal<Vec<FeedEvent>>, WriteSignal<Vec<FeedEvent>>) =
        use_context().expect("Feed context expected");
    let feed_target = |t: FeedTarget| match t {
        FeedTarget::Playlist { title, author } => format!("playlist {} by {}", title, author),
        FeedTarget::Album(s) => format!("album {}", s),
        FeedTarget::None => String::new(),
    };
    view! {
        <h2>Feed</h2>
        <hr />
        {move || {
            feed()
                .into_iter()
                .map(|f| {
                    view! {
                        <div class="feed-event">
                            {match f.action {
                                FeedAction::Like => {
                                    view! {
                                        <span class="actor">{f.actor}</span>
                                        <span class="action">" liked "</span>
                                        <span class="target">{feed_target(f.target)}</span>
                                    }
                                        .into_any()
                                }
                                FeedAction::Comment(c) => {
                                    view! {
                                        <span class="actor">{f.actor}</span>
                                        <span class="action">" commented on "</span>
                                        <span class="target">
                                            {format!("{}:", feed_target(f.target))}
                                        </span>
                                        <div class="comment">{c}</div>
                                    }
                                        .into_any()
                                }
                                FeedAction::FriendRequest => {
                                    view! {
                                        <span class="actor">{f.actor}</span>
                                        <span class="action">" sent you a friend request!"</span>
                                    }
                                        .into_any()
                                }
                            }}
                        </div>
                    }
                })
                .collect_view()
        }}
    }
}

// playlist struct
#[derive(Deserialize, Debug, Clone)]
struct Playlist {
    title: String,
    author: String,
    description: String,
    songs: Vec<Song>,
    likes: usize,
}

// song struct
#[derive(Deserialize, Debug, Clone)]
struct Song {
    title: String,
    author: String,
    album: String,
    release: String,
}

// feed event struct
#[derive(Clone)]
struct FeedEvent {
    action: FeedAction,
    actor: String,
    target: FeedTarget,
}

// constructor for feed event
impl FeedEvent {
    fn new(action: FeedAction, actor: String, target: FeedTarget) -> FeedEvent {
        FeedEvent {
            action,
            actor,
            target,
        }
    }
}

// feed action enum
#[derive(Clone)]
enum FeedAction {
    Like,
    Comment(String),
    FriendRequest,
}

// feed target enum
#[derive(Clone)]
enum FeedTarget {
    Playlist { title: String, author: String },
    Album(String),
    None,
}
