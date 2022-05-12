use std::{env, vec};

use reqwest;
use soup::prelude::*;

/// # Bing Search
///
/// ## Arguments
/// 
/// - arg1 - query
/// - arg2 - max results
/// - arg3 - cvid
/// 
/// [search keywords](https://support.microsoft.com/en-gb/topic/advanced-search-keywords-ea595928-5d63-4a0b-9c6b-0b769865e78a)
/// 
/// [advanced search options](https://support.microsoft.com/en-gb/topic/advanced-search-options-b92e25f1-0085-4271-bdf9-14aaea720930)
#[tokio::main]
async fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    for s in args.iter() {
        if s == &String::from("--help") ||
        s == &String::from("-h") {
            println!("Bing Search by Neosb\n");
            println!("arg#1 - query\narg#2 - max results\narg#3 - cvid");
            return Ok(());
        }
    }

    let mut query = &String::from("lulzsec");
    if args.len() >= 2 {
        query = &args[1];
    }

    let mut max_results = 10;
    if args.len() >= 3 {
        let max_res = &args[2];
        let parsed = max_res.parse().unwrap_or(10);
        max_results = parsed.clone();
    }

    let mut cvid = &String::from("4D2EA03FB1D5439C994D1F5C7D902272");
    if args.len() >= 4 {
        cvid = &args[3];
    }

    if args.len() > 4 {
        return Err("Too much arguments!".to_string());
    }
    
    let mut query_num = 1;
    loop {
        let test = query_num - 1;
        if test > max_results {
            break
        }
        let user_agent = "{'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.95 Safari/537.36'}";
        let client = reqwest::Client::builder()
            .user_agent(user_agent)
            .build();
        let query = format!("https://www.bing.com/search?q={}&qs=n&form=QBRE&sp=-1&sc=8-9&sk=&cvid={}&setlang=en&first={}", query, cvid, query_num);
        let c = client.expect("Internal error, cal 911!");
        let resp = c.get(query).send().await.unwrap().text().await;
        let soup = Soup::new(resp.unwrap().as_str());
        let ol_opt = soup.tag("ol").find();
        let ol = ol_opt.expect("No results returned!");
        let li = ol.tag("li").find_all();
        let mut tmp_s_v: Vec<String> = vec![];
        li.for_each(|l| {
            let children = l.children();
            for (_, child) in children.enumerate() {
                let h2 = child.tag("h2").find_all();
                for (_, h) in h2.enumerate() {
                    let ha = h.tag("a").find();
                    match ha {
                        Some(ha) => {
                            let href = ha.get("href");
                            match href {
                                Some(href) => {
                                    if href.get(0..4) != Some(&"http".to_string()) {
                                        continue;
                                    }
                                    if href.get(0..28) == Some(&"https://www.bing.com/aclick?".to_string()) {
                                        continue;
                                    }

                                    let mut tmp_s_h = format!("{} - {}\n", href, h.text());
                                    let div_out = child.tag("div").find_all();
                                    let mut tmp_s_p: String = String::from("");
                                    let mut last_p: String = String::from("");
                                    for (_, div) in div_out.enumerate() {
                                        let p = div.tag("p").find();
                                        match p {
                                            Some(p) => {
                                                let p_t = p.text();
                                                if last_p == p_t {
                                                    continue
                                                }
                                                last_p = p.text();
                                                tmp_s_p = format!("{}\n{}\n", tmp_s_p, p_t)
                                            }
                                            _ => {}
                                        }
                                    }
                                    tmp_s_h = format!("{}{}", tmp_s_h, tmp_s_p);
                                    let mut tmp: Vec<String> = vec![tmp_s_h];
                                    tmp_s_v.append(&mut tmp);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        });
        tmp_s_v.dedup();
        for s in tmp_s_v.iter() {
            print!("{}", s);
        }
        query_num += 10;
    }
    return Ok(());
}
