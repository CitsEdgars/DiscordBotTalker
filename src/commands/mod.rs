pub mod general;

use general::*;
use serenity::framework::standard::macros::group;

#[group]
#[commands(features, toggle, slots)]
pub struct General;
