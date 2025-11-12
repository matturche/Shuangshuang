use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

// use leptos::leptos_dom::logging::console_log;
use rand::Rng;
use thiserror::Error;

use crate::exercise::{AudioQuality, HanziPair, ShuffleMode, Tone};

const WHILE_BREAK_LIMIT: u32 = 50;

#[derive(Error, Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum TextHandlingError {
    #[error("The text handled is not exactly two hanzi long.")]
    WordIsNotTwoHanziLong,
    #[error("The tone numbers obtained from pinyin were not correct.")]
    PinyinToneNumbersUnmatch,
    #[error("Unkown error")]
    Unknown,
}

pub fn format_word_url(chinese_word: &str, audio_quality: AudioQuality) -> String {
    format!(
        "https://github.com/hugolpz/audio-cmn/raw/refs/heads/master/{}/hsk/cmn-{chinese_word}.mp3",
        audio_quality.to_string()
    )
}

pub fn format_toned_syllable_url(syllable: &str, tone: &str) -> String {
    format!(
        "https://github.com/hugolpz/audio-cmn/raw/refs/heads/master/64k/syllabs/cmn-{syllable}{tone}.mp3"
    )
}

pub fn get_random_hanzi_pairs_idxs(
    nb_elements: u32,
    hanzi_pairs: &Vec<HanziPair>,
    shuffle_mode: ShuffleMode,
) -> Vec<usize> {
    let mut idxs: Vec<usize> = vec![];
    let mut used_idxs: HashSet<usize> = HashSet::new();
    let mut rng = rand::rng();
    let mut break_counter = 0;
    match shuffle_mode {
        ShuffleMode::Random => {
            for _ in 0..nb_elements {
                let mut random_idx: usize = rng.random_range(0..hanzi_pairs.len());
                while used_idxs.contains(&random_idx) {
                    random_idx = rng.random_range(0..hanzi_pairs.len());
                    break_counter += 1;
                    if break_counter >= WHILE_BREAK_LIMIT {
                        break;
                    }
                }
                idxs.push(random_idx);
                used_idxs.insert(random_idx);
            }
        }
        ShuffleMode::Even => {
            let mut tone_pairs_map: HashMap<String, Vec<usize>> = HashMap::new();
            for (i, hanzi_pair) in hanzi_pairs.iter().enumerate() {
                let tone_pair_key = hanzi_pair.pronounced_tone_pair.0.to_string()
                    + &hanzi_pair.pronounced_tone_pair.1.to_string();
                if tone_pairs_map.contains_key(&tone_pair_key) {
                    tone_pairs_map.get_mut(&tone_pair_key).unwrap().push(i);
                } else {
                    tone_pairs_map.insert(tone_pair_key, vec![i]);
                }
            }
            let tone_pairs_keys: Vec<&String> = tone_pairs_map.keys().collect();
            for _ in 0..nb_elements {
                let random_tone_pair_key_idx: usize = rng.random_range(0..tone_pairs_keys.len());
                let random_tone_pair_key = tone_pairs_keys[random_tone_pair_key_idx];
                let mut random_idx: usize =
                    rng.random_range(0..tone_pairs_map[random_tone_pair_key].len());
                let mut hanzi_pair_idx: usize = tone_pairs_map[random_tone_pair_key][random_idx];
                while used_idxs.contains(&hanzi_pair_idx) {
                    random_idx = rng.random_range(0..tone_pairs_map[random_tone_pair_key].len());
                    hanzi_pair_idx = tone_pairs_map[random_tone_pair_key][random_idx];
                    break_counter += 1;
                    if break_counter >= WHILE_BREAK_LIMIT {
                        break;
                    }
                }
                idxs.push(hanzi_pair_idx);
                used_idxs.insert(hanzi_pair_idx);
            }
        }
    }
    idxs
}

#[allow(dead_code)]
pub fn get_length_of_chinese_string(text: &str) -> usize {
    let mut length = 0;
    for idx in 0..text.len() {
        if text.is_char_boundary(idx) {
            length += 1;
        }
    }
    length
}

pub fn get_tones_only_from_pronounced_pinyin(pinyin: &str) -> String {
    let mut pinyin_tone_numbers = pinyin.replace(char::is_alphabetic, "");
    // // Add Neutral tone marker if needed
    if pinyin_tone_numbers.len() == 1 {
        pinyin_tone_numbers.push_str(&Tone::NeutralTone.to_string());
    }
    pinyin_tone_numbers.to_string()
}

pub fn get_pronounced_pinyin(word: &str, pinyin: &str, tone_pair: &(Tone, Tone)) -> String {
    let mut pronounced_pinyin = String::from(pinyin);
    // Bu becomes second tone with fourth tone rule
    if bu_is_first_hanzi(word) {
        if let Tone::Tone4 = tone_pair.1 {
            pronounced_pinyin =
                pinyin.replacen(&Tone::Tone4.to_string(), &Tone::Tone2.to_string(), 1);
        }
    }
    if yi_is_first_hanzi(word) {
        // Yi becomes second tone with fourth tone rule
        if let Tone::Tone4 = tone_pair.1 {
            pronounced_pinyin = pinyin.replace(&Tone::Tone1.to_string(), &Tone::Tone2.to_string());
        } else {
            // Yi becomes fourth tone with other tones rule
            // NOTE: This only works because there are no pairs in our corpus were Yi1 acts as an ordinal number!
            pronounced_pinyin =
                pinyin.replacen(&Tone::Tone1.to_string(), &Tone::Tone4.to_string(), 1);
        }
    }
    // Double 3 tone change rule
    if tone_pair.0 == Tone::Tone3 && tone_pair.1 == Tone::Tone3 {
        pronounced_pinyin = pinyin.replacen(&Tone::Tone3.to_string(), &Tone::Tone2.to_string(), 1);
    }
    pronounced_pinyin
}

pub fn bu_is_first_hanzi(word: &str) -> bool {
    if let Some(index) = word.find("不") {
        if index == 0 {
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub fn yi_is_first_hanzi(word: &str) -> bool {
    if let Some(index) = word.find("一") {
        if index == 0 {
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub fn get_tones_from_pinyin(pinyin_with_nums: &str) -> Vec<Tone> {
    let mut tones: Vec<Tone> = vec![];
    let pinyin_tone_numbers = pinyin_with_nums.replace(char::is_alphabetic, "");
    for tone_number in pinyin_tone_numbers.chars() {
        tones.push(Tone::from_str(&tone_number.to_string()).expect(&format!(
            "Failed to build Tone from tone number: {tone_number}"
        )));
    }
    // Add a neutral tone if needed
    if tones.len() == 1 {
        tones.push(Tone::NeutralTone);
    }
    tones
}

#[cfg(test)]
mod tests {
    use super::*;
    use pinyin::ToPinyin;

    pub fn get_pinyin_from_chinese_word(word: &str) -> Option<String> {
        let mut pinyin_output = String::new();
        for pinyin in word.to_pinyin() {
            if let Some(pinyin) = pinyin {
                pinyin_output.push_str(pinyin.with_tone_num_end());
            }
        }
        if pinyin_output.len() > 0 {
            Some(pinyin_output)
        } else {
            None
        }
    }

    #[test]
    fn get_tones_from_pinyin_works() {
        let pinyin_with_nums = "ni3hao3";
        let expected_tones = vec![Tone::Tone3, Tone::Tone3];
        let result_tones = get_tones_from_pinyin(pinyin_with_nums);
        assert_eq!(expected_tones, result_tones);
    }

    #[test]
    fn test_no_tone_change_works() {
        let chinese_word = "严肃";
        let expected_pinyin = "yan2su4";
        let tone_pair = (Tone::Tone2, Tone::Tone4);
        let normal_pinyin = get_pinyin_from_chinese_word(chinese_word);
        let result_pinyin = get_pronounced_pinyin(
            chinese_word,
            &normal_pinyin.expect("No pinyin detected"),
            &tone_pair,
        );
        assert_eq!(expected_pinyin, &result_pinyin);
    }

    #[test]
    fn test_third_tone_change_works() {
        let chinese_word = "你好";
        let expected_pinyin = "ni2hao3";
        let tone_pair = (Tone::Tone3, Tone::Tone3);
        let normal_pinyin = get_pinyin_from_chinese_word(chinese_word);
        let result_pinyin = get_pronounced_pinyin(
            chinese_word,
            &normal_pinyin.expect("No pinyin detected"),
            &tone_pair,
        );
        assert_eq!(expected_pinyin, &result_pinyin);
    }

    #[test]
    fn test_yi_1_2_tone_change_works() {
        let chinese_word = "一会";
        let expected_pinyin = "yi2hui4";
        let tone_pair = (Tone::Tone1, Tone::Tone4);
        let normal_pinyin = get_pinyin_from_chinese_word(chinese_word);
        let result_pinyin = get_pronounced_pinyin(
            chinese_word,
            &normal_pinyin.expect("No pinyin detected"),
            &tone_pair,
        );
        assert_eq!(expected_pinyin, &result_pinyin);
    }

    #[test]
    fn test_yi_1_4_tone_change_works() {
        let chinese_word = "一点";
        let expected_pinyin = "yi4dian3";
        let tone_pair = (Tone::Tone1, Tone::Tone3);
        let normal_pinyin = get_pinyin_from_chinese_word(chinese_word);
        let result_pinyin = get_pronounced_pinyin(
            chinese_word,
            &normal_pinyin.expect("No pinyin detected"),
            &tone_pair,
        );
        assert_eq!(expected_pinyin, &result_pinyin);
    }

    #[test]
    fn test_bu_4_2_tone_change_works() {
        let chinese_word = "不要";
        let expected_pinyin = "bu2yao4";
        let tone_pair = (Tone::Tone4, Tone::Tone4);
        let normal_pinyin = get_pinyin_from_chinese_word(chinese_word);
        let result_pinyin = get_pronounced_pinyin(
            chinese_word,
            &normal_pinyin.expect("No pinyin detected"),
            &tone_pair,
        );
        assert_eq!(expected_pinyin, &result_pinyin);
    }
}
