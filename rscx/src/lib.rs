pub mod attributes;
pub mod props;
pub use attributes::*;

#[cfg(feature = "axum")]
pub mod axum;
pub mod context;
pub mod format_wrapper;
pub mod render;

pub extern crate rscx_macros;
use std::future::Future;

pub use rscx_macros::*;

pub extern crate typed_builder;

pub extern crate html_escape;
pub use format_wrapper::FormatWrapper;

use async_trait::async_trait;
use futures::future::join_all;

pub trait CollectFragment<I>
where
    I: Iterator,
    Vec<String>: FromIterator<<I as Iterator>::Item>,
{
    fn collect_fragment(self) -> String;
}

impl<I> CollectFragment<I> for I
where
    I: Iterator,
    Vec<String>: FromIterator<<I as Iterator>::Item>,
{
    fn collect_fragment(self) -> String {
        self.collect::<Vec<String>>().join("")
    }
}

#[async_trait]
pub trait CollectFragmentAsync<I, Fut>
where
    I: Iterator,
    Vec<Fut>: FromIterator<<I as Iterator>::Item>,
    Fut: Future<Output = String>,
{
    async fn collect_fragment_async(self) -> String;
}

#[async_trait]
impl<I, Fut> CollectFragmentAsync<I, Fut> for I
where
    I: Iterator + Send,
    Vec<Fut>: FromIterator<<I as Iterator>::Item>,
    Fut: Future<Output = String> + Send,
{
    async fn collect_fragment_async(self) -> String {
        join_all(self.collect::<Vec<_>>()).await.join("")
    }
}

pub trait MapFragmentExt: Iterator {
    fn map_fragment<F, B>(self, f: F) -> String
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
        B: ToString;
}

impl<T> MapFragmentExt for T
where
    T: Iterator,
{
    fn map_fragment<F, B>(self, f: F) -> String
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
        B: ToString,
    {
        self.map(f).map(|b| b.to_string()).collect::<Vec<_>>().join("")
    }
}
