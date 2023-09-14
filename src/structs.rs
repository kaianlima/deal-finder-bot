use std::{fmt::Display, sync::Arc};

#[derive(Clone)]
pub struct Data(pub Arc<DataInner>);

pub struct DataInner {
    pub discord_guild_id: String,
    pub ds_token: String,
    pub reqwest: reqwest::Client,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub site: String,
    pub name: String,
    pub currency: String,
    pub full_price: String,
    pub discounted_price: String,
    pub discount: String,
    pub img_url: String,
}

#[derive(Clone, Debug)]
pub struct GamesVec {
    pub games: Vec<Game>,
}

#[derive(Clone, Debug)]
pub struct GameOpt {
    pub game: Option<Game>,
}

impl Display for GameOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.game.as_ref() {
            Some(g) => {
                if g.discounted_price.is_empty() {
                    if g.site == "Nuuvem" {
                        write!(f, "Price: {}{}\nDiscount: {}", g.currency, g.full_price, g.discount)
                    } else {
                        write!(f, "Price: {}\nDiscount: {}", g.full_price, g.discount)
                    }
                } else {
                    if g.site == "Nuuvem" {
                        write!(f, "Price: {}{}\nDiscount: {}", g.currency, g.discounted_price, g.discount)
                    } else {
                        write!(f, "Price: {}\nDiscount: {}", g.discounted_price, g.discount)
                    }
                }
            }
            None => write!(f, "Not found!"),
        }
    }
}



pub type Command = poise::Command<Data, CommandError>;
pub type CommandError = Error;
pub type CommandResult<E=Error> = Result<(), E>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;