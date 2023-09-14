use scraper::Selector;

use crate::structs::{Game, GameOpt, GamesVec};

pub fn make_selector(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

pub fn get_element_text(select: &scraper::element_ref::Select) -> String {
    let mut elements: Vec<String> = Vec::new();
    for e in select.clone() {
        let element = e.text().collect::<Vec<_>>().join(" ").trim().replace('\n', " ");
        elements.push(element);
    }
    elements.into_iter().collect::<String>()
}

pub fn get_currency(cell: &scraper::element_ref::Select) -> String {
    let monetary_text = get_element_text(cell);
    let split_text: Vec<&str> = monetary_text.split(char::is_numeric).collect();
    let currency = split_text.first().unwrap().to_string();

    currency
}

pub fn get_attr_src_text(select: &mut scraper::element_ref::Select) -> String {
    let element = select.next();
    let mut src: String = String::new();
    if let Some(e) = element {
        if let Some(str) = e.value().attr("src") {
            src = str.to_string();
        }
    }
    src
}

pub fn search_in(games_searched: &GamesVec, text: &str) -> GameOpt {
    let index = games_searched.games.iter().position(|game| {
        game.name.to_lowercase() == text.to_lowercase()
    });

    let game_matched: Option<Game> = match index {
        Some(i) => games_searched.games.get(i).cloned(),
        None => None,
    };

    GameOpt { game: game_matched }
}