use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use downcast_rs::Downcast;

#[cfg(feature = "mustache")]
mod mustache;

pub type Properties = HashMap<&'static str, String>;

pub trait Renderable: Downcast {
    /// Register interest in specific element properties
    ///
    /// Any property names returned will be queried for
    /// there value and added to `props` before calling `render`
    fn props(&self) -> &[&'static str] {
        &[]
    }

    /// Render the component to a string
    ///
    /// `props` contains key-value pairs for any keys
    /// that were returned when calling `props`
    fn render(&self, props: Properties) -> String;
}

impl_downcast!(Renderable);

impl<T> Renderable for T
    where T: ::std::fmt::Display + 'static
{
    fn props(&self) -> &[&'static str] {
        &[]
    }

    fn render<'doc>(&self, props: Properties) -> String {
        self.to_string()
    }
}


/// Component for templating
#[derive(Debug)]
pub struct Component<D, T> {
    pub data: D,
    pub template: T,
    pub props: Vec<&'static str>,
}

impl<D, T> Deref for Component<D, T> {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<D, T> DerefMut for Component<D, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
