use std::{error};
use futures::future::join_all;
use lazy_static::lazy_static;
use reqwest::Client;
use scraper::{Selector, Html};
use itertools::Itertools;
use serde::{Serialize};
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
}

impl Site {
    pub async fn get_top_links(&self, client: Client) -> Result<Vec<String>, Box<dyn error::Error>> {
        let body = client
            .get(&self.url)
            .header("User-Agent", "")
            .send()
            .await?
            .text().await?;
        let doc = Html::parse_document(&body);

        let links = doc.select(&self.link_selector);

        Ok(
            links
                .filter_map(|a_tag| a_tag.value().attr("href"))
                .map(|link| self.base_url.clone() + link)
                .collect_vec()
        )
    }

    pub async fn get_top_articles(&self, client: Client) -> Result<Vec<Article>, Box<dyn error::Error>> {
        Ok(join_all(self.get_top_links(client.clone())
            .await?.iter()
            .map(|link| async {
                self.parse_article(link.to_string(), client.clone()).await.unwrap()
            })
            .collect_vec())
            .await)
    }

    async fn parse_article(&self, url: String, client: Client) -> Result<Article, Box<dyn error::Error>> {
        let body = client
            .get(&url)
            .header("User-Agent", "")
            .send()
            .await?
            .text().await?;

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
    };
}

pub fn get_all() -> Vec<&'static Site> {
        vec![
            &ESPN,
            &NBA,
        ]
}
