#![doc = include_str!("../README.md")]
#![deny(
    missing_docs,
    rustdoc::broken_intra_doc_links,
    elided_lifetimes_in_paths
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod client;
mod dht;
mod error;
mod magnet;
mod torrent;
mod tracker;

pub use magnet::Magnet;