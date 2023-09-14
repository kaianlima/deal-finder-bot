use lazy_static::lazy_static;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::info;
use url::form_urlencoded::byte_serialize;

use crate::Context;
use crate::funcs::{get_attr_src_text, get_currency, get_element_text, make_selector, search_in};
use crate::structs::{Command, CommandResult, Error, Game, GameOpt, GamesVec};

lazy_static! {
    static ref STEAM_RESULTS_SELECTOR: Selector = make_selector("div[id='search_resultsRows']");
    static ref STEAM_GAME_SELECTOR: Selector = make_selector("a.search_result_row");
    static ref STEAM_GAME_TITLE_SELECTOR: Selector = make_selector("span.title");
    static ref STEAM_GAME_FULL_PRICE_SELECTOR: Selector = make_selector("div.discount_original_price");
    static ref STEAM_GAME_DISCOUNTED_PRICE_SELECTOR: Selector = make_selector("div.discount_final_price");
    static ref STEAM_GAME_DISCOUNT_SELECTOR: Selector = make_selector("div.discount_pct");
    static ref STEAM_GAME_IMG_URL_SELECTOR: Selector = make_selector("div.search_capsule > img");

    static ref EPIC_RESULTS_SELECTOR: Selector = make_selector("main section ul");
    static ref EPIC_GAME_SELECTOR: Selector = make_selector("li > div > div > a > div > div");
    static ref EPIC_GAME_TITLE_SELECTOR: Selector = make_selector("div:nth-child(2) > div:nth-child(2) > div > div");
    static ref EPIC_GAME_FULL_PRICE_SELECTOR: Selector = make_selector("div:nth-child(2) > div:nth-child(3) > div > div:nth-child(2) > div > div:first-child > span > div");
    static ref EPIC_GAME_DISCOUNTED_PRICE_SELECTOR: Selector = make_selector("div:nth-child(2) > div:nth-child(3) > div > div:nth-child(2) > div > div:nth-child(2) > span");
    static ref EPIC_GAME_DISCOUNT_SELECTOR: Selector = make_selector("div:nth-child(2) > div:nth-child(3) > div > div:first-child > span > div");
    static ref EPIC_GAME_IMG_URL_SELECTOR: Selector = make_selector("div:first-child > div > div > div > div > img");

    static ref NUUVEM_RESULTS_SELECTOR: Selector = make_selector("div.products-items");
    static ref NUUVEM_GAME_SELECTOR: Selector = make_selector("div.product-card--grid a.product-card--wrapper");
    static ref NUUVEM_GAME_TITLE_SELECTOR: Selector = make_selector("h3.product-title");
    static ref NUUVEM_GAME_CURRENCY_SELECTOR: Selector = make_selector("sup.currency-symbol");
    static ref NUUVEM_GAME_PRICE_INTEGER_SELECTOR: Selector = make_selector("span.integer");
    static ref NUUVEM_GAME_PRICE_DECIMAL_SELECTOR: Selector = make_selector("span.decimal");
    static ref NUUVEM_GAME_DISCOUNT_SELECTOR: Selector = make_selector("span.product-price--discount");
    static ref NUUVEM_GAME_IMG_URL_SELECTOR: Selector = make_selector("div.product-img > img");
}

pub async fn get_game_steam(
    client: Client,
    game: String,
) -> Result<Vec<Game>, Error> {
	// API endpoint var
    const STEAM_URL: &str = "https://store.steampowered.com/search/?term=";

    let game_param_encoded: String = byte_serialize(game.as_bytes()).collect();
    let url: String = format!("{}{}", &STEAM_URL, &game_param_encoded);

    info!("STEAM_URL call: {:#?}", url);

    let response_str: String = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    // let browser = Browser::default().unwrap();
    // let response_str = browser.new_tab()?
    // .navigate_to(&url)?
    // .wait_for_element_with_custom_timeout("div[id='search_resultsRows']", Duration::from_secs(10))?
    // .get_content()?;
    //let document = Html::parse_document(&inner_text_content.to_string());

    let document = Html::parse_document(&response_str);
    //info!("DOCUMENT: {:#?}", document);

    let mut game_list: Vec<Game> = Vec::new(); 

    let main_rows_result = document.select(&STEAM_RESULTS_SELECTOR);
    for main_rows in main_rows_result {
        let game_rows = main_rows.select(&STEAM_GAME_SELECTOR);
        for game_row in game_rows {
            let game_name = get_element_text(&game_row.select(&STEAM_GAME_TITLE_SELECTOR));
            let game_full_price = get_element_text(&game_row.select(&STEAM_GAME_FULL_PRICE_SELECTOR));
            let game_discounted_price = get_element_text(&game_row.select(&STEAM_GAME_DISCOUNTED_PRICE_SELECTOR));
            let game_discount = get_element_text(&game_row.select(&STEAM_GAME_DISCOUNT_SELECTOR));
            let game_currency = get_currency(&game_row.select(&STEAM_GAME_DISCOUNTED_PRICE_SELECTOR));
            let game_img_url = get_attr_src_text(&mut game_row.select(&STEAM_GAME_IMG_URL_SELECTOR));

            if !game_discounted_price.is_empty() || !game_full_price.is_empty() {
                let game: Game = Game {
                    site: "Steam".to_string(),
                    name: game_name,
                    currency: game_currency,
                    full_price: game_full_price,
                    discounted_price: game_discounted_price,
                    discount: if game_discount.is_empty() { "0%".to_string() } else { game_discount },
                    img_url: game_img_url,
                };
                game_list.push(game);
            }
        }
    }
    
    info!("Steam search found: {:#?}", &game_list.len());

    Ok(game_list)
}

pub async fn get_game_epic(
    client: Client,
    game: String,
) -> Result<Vec<Game>, Error> {
	// API endpoint var
    const EPIC_URL: &str = "https://store.epicgames.com/pt-BR/browse?q=";
    const PARAMS: &str = "&sortBy=relevancy&sortDir=DESC&count=40";

    let game_param_encoded: String = byte_serialize(game.as_bytes()).collect();
    let url: String = format!("{}{}{}", &EPIC_URL, &game_param_encoded, &PARAMS);

    info!("EPIC_URL call: {:#?}", url);

    let response_str: String = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    // let browser = Browser::default().unwrap();
    // let response_str = browser.new_tab()?
    // .navigate_to(&url)?
    // .wait_for_element_with_custom_timeout("div[id='search_resultsRows']", Duration::from_secs(10))?
    // .get_content()?;
    // let document = Html::parse_document(&response_str.to_string());

    //info!("document: {:#?}", document);

    let document = Html::parse_document(&response_str);

    let mut game_list: Vec<Game> = Vec::new(); 

    let main_rows_result = document.select(&EPIC_RESULTS_SELECTOR);
    //info!("EPIC_RESULTS: {:#?}", &main_rows_result.next().unwrap());
    for main_rows in main_rows_result {
        let game_rows = main_rows.select(&EPIC_GAME_SELECTOR);
        //info!("GAME_ROWS: {:#?}", &game_rows.next().unwrap());
        for game_row in game_rows {
            let game_name = get_element_text(&game_row.select(&EPIC_GAME_TITLE_SELECTOR));
            let game_full_price = get_element_text(&game_row.select(&EPIC_GAME_FULL_PRICE_SELECTOR));
            let game_discounted_price = get_element_text(&game_row.select(&EPIC_GAME_DISCOUNTED_PRICE_SELECTOR));
            let game_discount = get_element_text(&game_row.select(&EPIC_GAME_DISCOUNT_SELECTOR));
            let game_currency = get_currency(&game_row.select(&EPIC_GAME_DISCOUNTED_PRICE_SELECTOR));
            let game_img_url = get_attr_src_text(&mut game_row.select(&EPIC_GAME_IMG_URL_SELECTOR));

            if !game_discounted_price.is_empty() || !game_full_price.is_empty() {
                let game: Game = Game {
                    site: "Epic Games".to_string(),
                    name: game_name,
                    currency: game_currency,
                    full_price: game_full_price,
                    discounted_price: game_discounted_price,
                    discount: if game_discount.is_empty() { "0%".to_string() } else { game_discount },
                    img_url: game_img_url,
                };
                game_list.push(game);
            }
        }
    }
    
    info!("Epic Games search found: {:#?}", &game_list.len());

    Ok(game_list)
}

pub async fn get_game_nuuvem(
    client: Client,
    game: String,
) -> Result<Vec<Game>, Error> {
	// API endpoint var
    const NUUVEM_URL: &str = "https://www.nuuvem.com/br-pt/catalog/page/1/search/";

    let game_param_encoded: String = byte_serialize(game.as_bytes()).collect();
    let url: String = format!("{}{}", &NUUVEM_URL, &game_param_encoded);

    info!("NUUVEM_URL call: {:#?}", url);

    let response_str: String = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    // let browser = Browser::default().unwrap();
    // let response_str = browser.new_tab()?
    // .navigate_to(&url)?
    // .wait_for_element_with_custom_timeout("div[id='search_resultsRows']", Duration::from_secs(10))?
    // .get_content()?;
    //let document = Html::parse_document(&inner_text_content.to_string());

    let document = Html::parse_document(&response_str);
    //info!("DOCUMENT: {:#?}", document);

    let mut game_list: Vec<Game> = Vec::new(); 

    let main_rows_result = document.select(&NUUVEM_RESULTS_SELECTOR);
    //info!("main_rows_result: {:#?}", main_rows_result);
    for main_rows in main_rows_result {
        //info!("main_rows: {:#?}", main_rows);
        let game_rows = main_rows.select(&NUUVEM_GAME_SELECTOR);
        for game_row in game_rows {
            //info!("game_row: {:#?}", game_row);
            let game_name = get_element_text(&game_row.select(&NUUVEM_GAME_TITLE_SELECTOR));
            let game_price = get_element_text(&game_row.select(&NUUVEM_GAME_PRICE_INTEGER_SELECTOR)) + &get_element_text(&game_row.select(&NUUVEM_GAME_PRICE_DECIMAL_SELECTOR));
            let game_discount = get_element_text(&game_row.select(&NUUVEM_GAME_DISCOUNT_SELECTOR));
            let game_currency = get_currency(&game_row.select(&NUUVEM_GAME_CURRENCY_SELECTOR));
            let game_img_url = get_attr_src_text(&mut game_row.select(&NUUVEM_GAME_IMG_URL_SELECTOR));

            if !game_price.is_empty() {
                let game: Game = Game {
                    site: "Nuuvem".to_string(),
                    name: game_name,
                    currency: game_currency,
                    full_price: if game_discount.is_empty() { "R$0".to_string() } else { game_price.clone() },
                    discounted_price: if game_discount.is_empty() { "R$0".to_string() } else { game_price.clone() },
                    discount: if game_discount.is_empty() { "0%".to_string() } else { game_discount },
                    img_url: game_img_url,
                };
                game_list.push(game);
            }
        }
    }
    
    info!("Nuuvem search found: {:#?}", &game_list.len());

    Ok(game_list)
}

#[poise::command(prefix_command, slash_command, reuse_response, track_edits)]
pub async fn deal(
    ctx: Context<'_>, 
    game: String, 
) -> CommandResult {
    let start = Instant::now();
    info!("Commands parameters: {{Game: {:#?}}}", &game);

    let sites: Vec<&str> = vec!("Steam", "Epic Games", "Nuuvem");
    let client: Client = ctx.data().0.reqwest.clone();
    let mut site_games: HashMap<&str, GameOpt> = HashMap::new();

    // Get games option
    let games_steam: GamesVec = GamesVec {
        games: get_game_steam(client.clone(), game.clone()).await?,
    };
    // let game_epic_opt: GamesVec = GamesVec {
    //     games: get_game_epic(client.clone(), game.clone()).await?,
    // };
    let games_nuuvem: GamesVec = GamesVec {
        games: get_game_nuuvem(client.clone(), game.clone()).await?,
    };

    // Set game name for the first found in steam, or then nuuvem
    let mut game_name: String = game.clone();
    let mut img_url: String = String::new();
    if !games_steam.games.is_empty() {
        game_name = games_steam.games.first().unwrap().name.clone();
        img_url = games_steam.games.first().unwrap().img_url.clone()
    } else if !games_nuuvem.games.is_empty() {
        game_name = games_nuuvem.games.first().unwrap().name.clone();
        img_url = games_nuuvem.games.first().unwrap().img_url.clone();
    }

    let game_steam_opt: GameOpt = search_in(&games_steam, &game_name);
    let game_nuuvem_opt: GameOpt = search_in(&games_nuuvem, &game_name);
    site_games.insert(sites[0], game_steam_opt);
    site_games.insert(sites[2], game_nuuvem_opt);

    // Build response fields
    let mut fields_vec: Vec<(String, GameOpt, bool)> = Vec::default();
    for (site, game_opt) in site_games {
        fields_vec.push((site.to_string(), game_opt, true));
    }
    fields_vec.sort_by(|a, b| b.0.cmp(&a.0));

    ctx.send(|builder| {
        builder
        .content("").embed(|e| {
            e.title(&game_name)
            .image(img_url)
            .fields(fields_vec)
        })
    })
    .await?;

    let duration: Duration = start.elapsed();
    info!("Time elapsed in deal command is: {:?}", duration);

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [deal()]
}