use std::str::FromStr;

use gloo_net::http::Request;

use crate::{
    exercise::{HanziPair, Tone},
    utils::{get_pronounced_pinyin, get_tones_from_pinyin},
};

pub async fn fetch_hanzi_pairs() -> Vec<HanziPair> {
    let mut hanzi_pairs: Vec<HanziPair> = vec![];
    let resp = Request::get(
        "https://raw.githubusercontent.com/matturche/Shuangshuang/refs/heads/main/data/hanzi_pairs.txt"
    )
    .send()
    .await.expect("Failed send request for hanzi pairs");
    let text = resp.text().await.expect("Failed to get text from response");
    let lines: Vec<String> = text.lines().map(str::to_owned).collect();
    for line in lines.iter() {
        let splits: Vec<&str> = line.split(' ').collect();
        let tone_pair: (Tone, Tone) = (
            Tone::from_str(&splits[2].chars().nth(0).unwrap().to_string()).unwrap(),
            Tone::from_str(&splits[2].chars().nth(1).unwrap().to_string()).unwrap(),
        );
        let characters = splits[0].to_string();
        let pinyin = splits[1].to_string();
        let pronounced_pinyin = get_pronounced_pinyin(&characters, &pinyin, &tone_pair);
        let pronounced_tone_pair = get_tones_from_pinyin(&pronounced_pinyin);
        hanzi_pairs.push(HanziPair {
            characters,
            pinyin,
            pronounced_pinyin,
            tone_pair,
            pronounced_tone_pair: (pronounced_tone_pair[0], pronounced_tone_pair[1]),
        });
    }
    hanzi_pairs
}
