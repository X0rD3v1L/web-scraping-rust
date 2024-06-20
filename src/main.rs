use scraper::{Html, Selector};
use plotly::common::Title;
use plotly::{Plot, Bar,Layout};

fn retrieve_html() -> String {
    let response = reqwest::blocking::get("https://results.eci.gov.in/AcResultGenJune2024/candidateswise-S0123.htm").unwrap().text().unwrap();
    return response;
}

fn clean_data(raw_data : Vec<&str>) -> Vec<String> {
    raw_data
    .into_iter()
    .filter(|s| !s.trim().is_empty())
    .map(|s| s.trim().to_string())
    .collect()
}
fn basic_bar_chart(each_cand_name: Vec<String>, each_cand_votes: Vec<i32>) {
    let t = Bar::new(each_cand_name, each_cand_votes);
    let mut plot = Plot::new();
    plot.add_trace(t);

    let layout = Layout::new()
    .title(Title::new("Candidate Votes"))
    .width(1400)
    .height(700);

    plot.set_layout(layout);
    plot.show();
}
fn main() {
    let response: String = retrieve_html();
    let doc_body = Html::parse_document(&response);

    let all_cand_infos = Selector::parse(".cand-box").unwrap();

    let mut all_cleaned_data = Vec::new();

    for each_info in doc_body.select(&all_cand_infos) {
        let cand_info = each_info.text().collect::<Vec<_>>();

        all_cleaned_data.push(clean_data(cand_info));
    }

    let mut each_cand_votes = Vec::new();
    let mut each_cand_name = Vec::new();

    for each_cand_info in &all_cleaned_data {
        
        if let Some(vote_str) = each_cand_info.get(1) {
            if let Ok(vote) = vote_str.parse::<i32>() {
                each_cand_votes.push(vote);
            }
        }

        if let Some(name) = each_cand_info.get(3) {
            each_cand_name.push(name.to_string());

        }

    }

    basic_bar_chart(each_cand_name, each_cand_votes);
}
