pub mod general;
pub mod activities;
pub mod banking;
pub mod helpers;

use general::*;
use activities::*;
use banking::*;
use serenity::framework::standard::macros::group;

#[group]
#[commands(
    slots
    // ,card/coin
    // ,curse
    
    // ,play
    // ,playlist
)]
pub struct Activities;

#[group]
#[commands(
    features
    ,toggle
    ,test
)]
pub struct General;

#[group]
#[commands(
    wallet
    ,gain
    ,transfer
)]
pub struct Banking;