use anyhow::{anyhow, Result};
use rand::seq::SliceRandom;
use serde_json::Value;

pub struct Translate {
    pub lang: String,
    pub user_agents: Vec<String>,
}

impl Translate {
    pub fn new(lang: &str) -> Self {
        let user_agents = vec![
          "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36".to_string(),
          "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36".to_string(),
          "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:94.0) Gecko/20100101 Firefox/94.0".to_string(),
          "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:95.0) Gecko/20100101 Firefox/95.0".to_string(),
          "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36".to_string(),
          "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.55 Safari/537.36".to_string()
      ];

        Self {
            lang: lang.to_string(),
            user_agents,
        }
    }

    pub fn get_user_agent(&self) -> String {
        self.user_agents
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
    }

    pub fn generate_url(&self, text: &str) -> String {
        let text_encoded = urlencoding::encode(text);
        format!(
          "https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl={}&dt=t&q={}",
          self.lang, text_encoded
      )
    }

    pub fn get_translation(&self, text: &str) -> Result<(String, Vec<String>)> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&self.generate_url(text))
            .header("User-Agent", self.get_user_agent())
            .send()?
            .json::<Value>()?;

        let detected_lang = response[2]
            .as_str()
            .ok_or_else(|| anyhow!("无法检测语言"))?
            .to_string();

        let translations = response[0]
            .as_array()
            .ok_or_else(|| anyhow!("无效的响应格式"))?
            .iter()
            .filter_map(|v| v[0].as_str().map(String::from))
            .collect();

        Ok((detected_lang, translations))
    }
}
