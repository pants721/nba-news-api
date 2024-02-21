use futures::future::join_all;
use itertools::Itertools;
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client,
};
use scraper::{Html, Selector};
use serde::Serialize;
use std::error;
use utoipa::ToSchema;

use super::article::Article;

#[derive(Clone, Serialize, ToSchema)]
pub struct Site {
    pub name: String,
    pub url: String,
    pub base_url: String,
    #[serde(skip)]
    link_selector: Selector,
    #[serde(skip)]
    title_selector: Selector,
    #[serde(skip)]
    subtitle_selector: Selector,
    #[serde(skip)]
    author_selector: Selector,
    #[serde(skip)]
    date_selector: Selector,
    #[serde(skip)]
    headers: HeaderMap,
}

impl Site {
    pub async fn get_top_links(
        &self,
        client: Client,
    ) -> Result<Vec<String>, Box<dyn error::Error>> {
        let body = client
            .get(&self.url)
            .headers(self.headers.clone())
            .send()
            .await?
            .text()
            .await?;
        let doc = Html::parse_document(&body);

        let links = doc.select(&self.link_selector);

        Ok(links
            .filter_map(|a_tag| a_tag.value().attr("href"))
            .map(|link| self.base_url.clone() + link)
            .collect_vec())
    }

    pub async fn get_top_articles(
        &self,
        client: Client,
    ) -> Result<Vec<Article>, Box<dyn error::Error>> {
        Ok(join_all(
            self.get_top_links(client.clone())
                .await?
                .iter()
                .map(|link| self.parse_article(link.to_string(), client.clone())),
        )
        .await
        .into_iter()
        .filter_map(|x| x.ok())
        .collect_vec())
    }

    async fn parse_article(
        &self,
        url: String,
        client: Client,
    ) -> Result<Article, Box<dyn error::Error>> {
        let body = client
            .get(&url)
            .header("User-Agent", "")
            .send()
            .await?
            .text()
            .await?;
        Ok(self.parse_article_text(body, url).await?)
    }

    async fn parse_article_text(
        &self,
        body: String,
        url: String,
    ) -> Result<Article, Box<dyn error::Error>> {

        let doc = Html::parse_document(&body);

        // TODO: Extract this into a function
        let title = match doc.select(&self.title_selector).next() {
            Some(r) => r.inner_html(),
            None => "".to_string(),
        };
        let subtitle = match doc.select(&self.subtitle_selector).next() {
            Some(r) => r.inner_html(),
            None => "".to_string(),
        };
        let author = match doc.select(&self.author_selector).next() {
            Some(r) => r.inner_html(),
            None => "".to_string(),
        };
        let date = match doc.select(&self.date_selector).next() {
            Some(r) => r.inner_html(),
            None => "".to_string(),
        };

        Ok(Article {
            title,
            subtitle,
            author,
            date,
            url,
            source: self.name.clone(),
        })
    }
}

lazy_static! {
    static ref ESPN: Site = Site {
        name: "espn".to_string(),
        url: "https://espn.com/nba".to_string(),
        base_url: "https://espn.com".to_string(),
        link_selector: Selector::parse("section[class*=col-three] ul[class*='headlineStack'] > li > a").unwrap(),
        title_selector: Selector::parse("header[class=article-header] > h1").unwrap(),
        subtitle_selector: Selector::parse("none").unwrap(), // TODO: Figure out how to handle
        // a site not having an article attribute
        author_selector: Selector::parse("div:not([class=author-img])[class*=author]").unwrap(),
        date_selector: Selector::parse("div[class=article-meta] span[class*=timestamp]").unwrap(),
        headers: HeaderMap::new(),
    };

    static ref NBA: Site = Site {
        name: "nba".to_string(),
        url: "https://nba.com/news/category/top-stories".to_string(),
        base_url: "https://www.nba.com".to_string(),
        link_selector: Selector::parse("article[class*='Article'] > a").unwrap(),
        title_selector: Selector::parse("h1[class*=ahTitle]").unwrap(),
        subtitle_selector: Selector::parse("p[class*=ahSubtitle]").unwrap(),
        author_selector: Selector::parse("p[class*=authorName]").unwrap(),
        date_selector: Selector::parse("time[class*=ahDate]").unwrap(),
        headers: HeaderMap::from_iter(vec![
            (USER_AGENT, "".parse().unwrap())
        ].into_iter()),
    };
}

pub fn get_all() -> Vec<&'static Site> {
    vec![&ESPN, &NBA]
}

mod tests {
    use super::*;
    use scraper::Html;

    fn site_mock() -> Site {
        Site {
            name: "".to_string(),
            url: "".to_string(),
            base_url: "".to_string(),
            link_selector: Selector::parse("none").unwrap(),
            title_selector: Selector::parse("h1[class*=title]").unwrap(),
            subtitle_selector: Selector::parse("p[class=subtitle]").unwrap(),
            author_selector: Selector::parse("p[class*=author]").unwrap(),
            date_selector: Selector::parse("time").unwrap(),
            headers: HeaderMap::new(),
        }
    }

    fn article_expected() -> Article {
        Article { 
            title: "title 1".to_string(), 
            subtitle: "subtitle 1".to_string(), 
            author: "lucas".to_string(), 
            date: "2:00PM".to_string(), 
            url: "".to_string(), 
            source: "".to_string() 
        }
    }

    fn article_mock_html() -> String {
        Html::parse_fragment(r#"<!DOCTYPE html> <html> <head> <title>Page Title</title> </head> <body> <h1 class="asdasdasdasd_title_____">title 1</h1> <p class="subtitle">subtitle 1</p> <p class="thisistheauthor">lucas</p> <time>2:00PM</time> <p class="content"> blagh blahg</p> OTHER STUFF WOW LOOKS AT THIS INTERESTING STUFF </body> </html>"#).html()
    }

    #[actix_rt::test]
    async fn parse_article() {
        let actual_article = Site::parse_article_text(&site_mock(), article_mock_html(), "".to_string()).await.unwrap();
        
        assert_eq!(actual_article, article_expected());
    }

}
