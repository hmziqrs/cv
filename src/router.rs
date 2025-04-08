use dioxus::prelude::*;

use crate::screens::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    HomeScreen {},
}
