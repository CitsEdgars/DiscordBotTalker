pub mod general;
pub mod activities;
pub mod helpers;

use general::*;
use activities::*;
use serenity::framework::standard::macros::group;

#[group]
#[commands(
    slots
    // ,card/coin
    // ,curse
    
    // ,play
    // ,playlist
    
    // ,wallet
    // ,transfer
    // ,steal
)]
pub struct Activities;

#[group]
#[commands(
    features
    ,toggle
    ,test
)]
pub struct General;
