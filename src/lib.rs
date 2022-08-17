//! A bingo library.

mod caller;
mod card;

pub use crate::{
    caller::{
        ArrayCaller,
        Caller,
    },
    card::Card,
};
