//! Shared UI crate for the web and desktop packages.

pub mod client;
pub mod server;

pub use client::{
    App, AppErrorFallback, AppLanguage, DeveloperTools, Footer, Home, Main, Page, Route, Theme,
    Token, TokenList, TokenListItem, TokenLoadRequest, TopNavigation,
};
