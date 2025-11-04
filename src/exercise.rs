use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

#[derive(Error, Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum ExerciseError {
    #[error("Invalid str value for ExerciseType")]
    ParseExerciseTypeError,
    #[error("Invalid str value for InputStyle")]
    ParseInputStyleError,
    #[error("Invalid str value for AudioQuality")]
    ParseAudioQualityError,
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum ToneError {
    #[error("Invalid str value for Tone")]
    ParseToneError,
    #[error("Unknown error")]
    Unknown,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[allow(dead_code)]
pub enum Tone {
    Tone1,
    Tone2,
    Tone3,
    Tone4,
    #[default]
    NeutralTone,
}

impl FromStr for Tone {
    type Err = ToneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::Tone1),
            "2" => Ok(Self::Tone2),
            "3" => Ok(Self::Tone3),
            "4" => Ok(Self::Tone4),
            "5" => Ok(Self::NeutralTone),
            _ => Err(ToneError::ParseToneError),
        }
    }
}

impl From<char> for Tone {
    fn from(value: char) -> Self {
        match value {
            '1' => Self::Tone1,
            '2' => Self::Tone2,
            '3' => Self::Tone3,
            '4' => Self::Tone4,
            '5' => Self::NeutralTone,
            _ => Self::NeutralTone,
        }
    }
}

impl ToString for Tone {
    fn to_string(&self) -> String {
        match self {
            Self::Tone1 => "1".to_string(),
            Self::Tone2 => "2".to_string(),
            Self::Tone3 => "3".to_string(),
            Self::Tone4 => "4".to_string(),
            Self::NeutralTone => "5".to_string(),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum ExerciseType {
    #[default]
    ToneOnly,
    NoTonePinyin,
    Pinyin,
}

impl ToString for ExerciseType {
    fn to_string(&self) -> String {
        match self {
            Self::ToneOnly => "tone_only".to_string(),
            Self::NoTonePinyin => "no_tone_pinyin".to_string(),
            Self::Pinyin => "pinyin".to_string(),
        }
    }
}

impl FromStr for ExerciseType {
    type Err = ExerciseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tone_only" => Ok(Self::ToneOnly),
            "no_tone_pinyin" => Ok(Self::NoTonePinyin),
            "pinyin" => Ok(Self::Pinyin),
            _ => Err(ExerciseError::ParseExerciseTypeError),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum InputStyle {
    #[default]
    Keyboard,
    Touch,
}

impl ToString for InputStyle {
    fn to_string(&self) -> String {
        match self {
            Self::Keyboard => "keyboard".to_string(),
            Self::Touch => "touch".to_string(),
        }
    }
}

impl FromStr for InputStyle {
    type Err = ExerciseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "keyboard" => Ok(Self::Keyboard),
            "touch" => Ok(Self::Touch),
            _ => Err(ExerciseError::ParseInputStyleError),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum AudioQuality {
    Q18k,
    Q24k,
    #[default]
    Q64k,
    Q96k,
}

impl ToString for AudioQuality {
    fn to_string(&self) -> String {
        match self {
            Self::Q18k => "18k-abr".to_string(),
            Self::Q24k => "24k-abr".to_string(),
            Self::Q64k => "64k".to_string(),
            Self::Q96k => "96k".to_string(),
        }
    }
}

impl FromStr for AudioQuality {
    type Err = ExerciseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "18k-abr" => Ok(Self::Q18k),
            "24k-abr" => Ok(Self::Q24k),
            "64k" => Ok(Self::Q64k),
            "96k" => Ok(Self::Q96k),
            _ => Err(ExerciseError::ParseAudioQualityError),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ExerciseParams {
    pub exercise_size: u32,
    pub exercise_type: ExerciseType,
    pub input_style: InputStyle,
    pub timer_on: bool,
    pub audio_quality: AudioQuality,
}

impl Default for ExerciseParams {
    fn default() -> Self {
        Self {
            exercise_size: 10,
            exercise_type: ExerciseType::default(),
            input_style: InputStyle::default(),
            timer_on: false,
            audio_quality: AudioQuality::default(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShuangElement {
    pub hanzi_pair: HanziPair,
    pub user_answer: String,
    pub is_correct: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HanziPair {
    pub characters: String,
    pub pinyin: String,
    pub pronounced_pinyin: String,
    pub tone_pair: (Tone, Tone),
    pub pronounced_tone_pair: (Tone, Tone),
}

impl Default for HanziPair {
    fn default() -> Self {
        Self {
            characters: String::new(),
            pinyin: String::new(),
            pronounced_pinyin: String::new(),
            tone_pair: (Tone::default(), Tone::default()),
            pronounced_tone_pair: (Tone::default(), Tone::default()),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExerciseSummary {
    pub correct_answers: u32,
    pub test_elements: Vec<ShuangElement>,
    pub tone_pair_mistakes: HashMap<(Tone, Tone), u32>,
}

impl From<Vec<ShuangElement>> for ExerciseSummary {
    fn from(value: Vec<ShuangElement>) -> Self {
        let mut tone_pair_mistakes: HashMap<(Tone, Tone), u32> = HashMap::new();
        for elem in value.iter() {
            if !elem.is_correct {
                if let Some(mistakes) =
                    tone_pair_mistakes.get_mut(&elem.hanzi_pair.pronounced_tone_pair)
                {
                    *mistakes += 1;
                } else {
                    tone_pair_mistakes.insert(elem.hanzi_pair.pronounced_tone_pair.clone(), 1);
                }
            }
        }
        Self {
            correct_answers: value.iter().map(|x| x.is_correct as u32).sum(),
            test_elements: value,
            tone_pair_mistakes,
        }
    }
}

impl ExerciseSummary {
    pub fn get_correct_percentage(&self) -> f32 {
        (self.correct_answers as f32 / self.test_elements.len() as f32) * 100.0
    }
}

pub struct UserData {
    pub tone_pairs_correct_answers: HashMap<(Tone, Tone), u32>,
    pub tone_pairs_incorrect_answers: HashMap<(Tone, Tone), u32>,
}
