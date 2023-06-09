#![warn(missing_docs)]
#![deny(missing_copy_implementations)]
#![doc = include_str!("../readme.md")]

mod dense_map;
mod direction;
mod joint;
mod path_finder;

pub use crate::{
    dense_map::{
        iters::{DiamondPoints, GetTaxicabPoints, GetTaxicabPointsAround, MutGetTaxicabPoints},
        TaxicabMap,
    },
    direction::Direction,
    joint::Joint,
    path_finder::PathFinder,
};
