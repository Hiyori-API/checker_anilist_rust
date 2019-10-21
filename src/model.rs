#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct PageInfo {
    pub total: u32,
    pub perPage: u32,
    pub currentPage: u32,
    pub lastPage: u32,
    pub hasNextPage: bool
}

#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct PageItem {
    pub pageInfo: PageInfo,
    pub ANIME: Vec<Anime>
}

#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Page {
    pub Page: PageItem
}

#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Response {
    pub data: Page
}

#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct AnimeTitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub native: Option<String>
}

#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct External {
    pub id: Option<u32>,
    pub site: Option<String>,
    pub url: Option<String>
}

#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Anime { 
    pub id: u32,
    pub idMal: Option<u32>,
    pub title: AnimeTitle,
    pub externalLinks: Vec<External>
}