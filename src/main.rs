extern crate reqwest;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::io::Read;
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};

mod model;

fn main() {
    let mut page_responses: Vec<model::Response> = Vec::new();

    let mut anime_new = Vec::new(); // Will be added to DB
    let mut anime_unreferenced = Vec::new(); // don't have MAL ID

    // GraphQL request settings
    let page = 1;
    let per_page = 50; // max
    let is_adult = false;
    let sort = "ID_DESC";

    let client = reqwest::Client::new();
    let mut params = HashMap::new();
    params.insert("query", "query ($page: Int = 1, $id: Int, $type: MediaType, $isAdult: Boolean = false, $search: String, $format: MediaFormat, $status: MediaStatus, $countryOfOrigin: CountryCode, $source: MediaSource, $season: MediaSeason, $year: String, $onList: Boolean, $yearLesser: FuzzyDateInt, $yearGreater: FuzzyDateInt, $licensedBy: [String], $includedGenres: [String], $excludedGenres: [String], $includedTags: [String], $excludedTags: [String], $sort: [MediaSort] = [ID_DESC]) { Page(page: $page, perPage: 50) { pageInfo { total perPage currentPage lastPage hasNextPage } ANIME: media(id: $id, type: $type, season: $season, format: $format, status: $status, countryOfOrigin: $countryOfOrigin, source: $source, search: $search, onList: $onList, startDate_like: $year, startDate_lesser: $yearLesser, startDate_greater: $yearGreater, licensedBy_in: $licensedBy, genre_in: $includedGenres, genre_not_in: $excludedGenres, tag_in: $includedTags, tag_not_in: $excludedTags, sort: $sort, isAdult: $isAdult) { id, idMal, title { romaji english native }, externalLinks { id, site, url } } } }");

    // 1. loop pages if necessery
    // 2. idMal may be null, so check for that and list it in seperate file
    // 3x. serialize 
    // 4. check against local db
    // 5. merge into local db if new // before looping page, every page has new process 1-4

    let mut response = client.post("https://graphql.anilist.co")
        .json(&params)
        .send()
        .expect("Failed to send request");
    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Failed to read response");
    println!("{}", &buf);

    let decoded: model::Response = json::decode(&buf).unwrap();

    let mut anime_list = decoded.data.Page.ANIME;

    for anime in anime_list.iter() {
        // idMal may be null, check for and add to `anime_unreferenced`
        if anime.idMal == None {
            anime_unreferenced.push(anime);
        }

        if anime.idMal != None {
            anime_new.push(anime);
        }
    }

    println!("{:?}", anime_unreferenced);

}
