use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;
use std::path::Path;
use toml;

#[derive(Deserialize, Serialize)]
struct Config {
    openai_api_key: String,
    openai_url: String,
    openai_model: String,
    user_language: String,
}

#[derive(Deserialize, Serialize)]
struct PromptConfig {
    prompt_zh: String,
    prompt_en: String,
    prompt_jp: String,
    prompt_zh_tw: String,
}

pub const VALID_OPENAI_API_KEY: &str = "openai_api_key";
pub const VALID_OPENAI_URL: &str = "openai_url";
pub const VALID_OPENAI_MODEL: &str = "openai_model";
pub const VALID_USER_LANGUAGE: &str = "user_language";

const PROMPT_ZH: &str = "根据以下的git差异内容，生成一个简洁的提交信息。根据更改的性质，以以下其中一个前缀开头：'build'（构建系统），'chore'（杂务），'ci'（持续集成），'docs'（文档），'feat'（新功能），'fix'（修复），'perf'（性能），'refactor'（重构），'style'（样式），'test'（测试）：";
const PROMPT_EN: &str = "Based on the following git diff content, generate a concise commit message. Start with one of the following prefixes according to the nature of the change: 'build' (build system), 'chore' (miscellaneous), 'ci' (continuous integration), 'docs' (documentation), 'feat' (new feature), 'fix' (fix), 'perf' (performance), 'refactor' (refactor), 'style' (style), 'test' (test):";
const PROMPT_JP: &str = "以下のgitの差分内容に基づいて、簡潔なコミットメッセージを生成します。変更の性質に応じて、次の接頭辞のいずれかで始めます：'build'（ビルドシステム）、'chore'（その他）、'ci'（継続的統合）、'docs'（ドキュメント）、'feat'（新機能）、'fix'（修正）、'perf'（パフォーマンス）、'refactor'（リファクタリング）、'style'（スタイル）、'test'（テスト）：";
const PROMPT_ZH_TW: &str = "根據以下的git差異內容，生成一個簡潔的提交信息。根據更改的性質，以以下其中一個前綴開頭：'build'（構建系統），'chore'（雜務），'ci'（持續集成），'docs'（文檔），'feat'（新功能），'fix'（修復），'perf'（性能），'refactor'（重構），'style'（樣式），'test'（測試）：";

pub fn validate_config_key(key: &str) -> Result<&str, &'static str> {
    match key {
        VALID_OPENAI_API_KEY | VALID_OPENAI_URL | VALID_OPENAI_MODEL => Ok(key),
        VALID_USER_LANGUAGE => Ok(key),
        _ => Err("Invalid configuration key"),
    }
}

pub fn get_config_key<P: AsRef<Path>>(keys: &[&str], path: P) -> Result<Vec<String>, &'static str> {
    let config_file = fs::read_to_string(path.as_ref()).expect("Could not read config file");
    let config: Config = toml::from_str(&config_file).expect("Could not parse config file");

    let mut result = Vec::new();

    for key in keys {
        match validate_config_key(key) {
            Ok(vaild_key) => {
                let value = match vaild_key {
                    VALID_OPENAI_API_KEY => config.openai_api_key.clone(),
                    VALID_OPENAI_URL => config.openai_url.clone(),
                    VALID_OPENAI_MODEL => config.openai_model.clone(),
                    VALID_USER_LANGUAGE => config.user_language.clone(),
                    _ => panic!("Invalid configuration key"),
                };
                result.push(value);
            }
            Err(_) => return Err("Invalid configuration key"),
        }
    }
    Ok(result)
}

pub fn set_config_key<P: AsRef<Path> + Clone>(
    key: &str,
    value: &str,
    path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_file = fs::read_to_string(path.as_ref()).expect("Could not read config file");
    let mut config: Config = toml::from_str(&config_file).expect("Could not parse config file");

    let key = validate_config_key(key).expect("Invalid configuration key");
    match key {
        VALID_OPENAI_API_KEY => config.openai_api_key = value.to_string(),
        VALID_OPENAI_URL => config.openai_url = value.to_string(),
        VALID_OPENAI_MODEL => config.openai_model = value.to_string(),
        VALID_USER_LANGUAGE => config.user_language = value.to_string(),
        _ => panic!("Invalid configuration key"),
    }
    let new_config = toml::to_string(&config).expect("Could not serialize config");
    fs::write(path, new_config).expect("Could not write to config file");

    Ok(())
}

pub fn get_language<P: AsRef<Path> + Clone>(user_language: &str, path: P) -> String {
    let prompt_file = fs::read_to_string(path).expect("Could not read prompt config file");
    let prompt_config: PromptConfig =
        toml::from_str(&prompt_file).expect("Could not parse prompt config file");

    match user_language {
        "zh" => prompt_config.prompt_zh.clone(),
        "en" => prompt_config.prompt_en.clone(),
        "jp" => prompt_config.prompt_jp.clone(),
        "zh_tw" => prompt_config.prompt_zh_tw.clone(),
        _ => panic!("Invalid user language"),
    }
}

pub fn move_prompt_toml<P: AsRef<Path> + Clone>(path: P) {
    let prompt_config = PromptConfig {
        prompt_zh: PROMPT_ZH.to_string(),
        prompt_en: PROMPT_EN.to_string(),
        prompt_jp: PROMPT_JP.to_string(),
        prompt_zh_tw: PROMPT_ZH_TW.to_string(),
    };
    let prompt_toml = toml::to_string(&prompt_config).expect("Could not serialize prompt config");
    fs::write(path, prompt_toml).expect("Could not write to prompt config file");
}

pub fn generate_config_toml() -> String {
    let config = Config {
        openai_api_key: "".to_string(),
        openai_url: "https://api.openai.com".to_string(),
        openai_model: "".to_string(),
        user_language: "en".to_string(),
    };
    toml::to_string(&config).expect("Could not serialize config")
}

pub fn write_config_to_toml(config_toml: &str, path: &Path) -> Result<(), Error> {
    fs::write(path, config_toml).expect("Could not write to config file");
    Ok(())
}
