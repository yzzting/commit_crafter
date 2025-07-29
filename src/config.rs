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

const PROMPT_ZH: &str = "请根据以下git diff内容，结合最近5条提交信息，生成一条风格一致、语言纯正的简洁Git提交信息。请仅用简体中文描述，避免中英混用。根据本次更改的性质，从以下前缀中选择一个开头：'build'（构建系统）、'chore'（杂务）、'ci'（持续集成）、'docs'（文档）、'feat'（新功能）、'fix'（修复）、'perf'（性能）、'refactor'（重构）、'style'（样式）、'test'（测试）。提交信息应重点突出本次更改的核心目的和影响，避免冗长和无关细节。示例：\n\nfeat: 支持多语言提交信息生成\nfix: 修复API密钥配置无法保存的问题\n\n请生成一条与上文风格一致的提交信息：";

const PROMPT_EN: &str = "Based on the following git diff content and the last 5 commit messages, generate a concise and stylistically consistent Git commit message. Only use English; avoid mixing languages. Start the message with one of these prefixes according to the nature of the change: 'build' (build system), 'chore' (miscellaneous), 'ci' (continuous integration), 'docs' (documentation), 'feat' (new feature), 'fix' (fix), 'perf' (performance), 'refactor' (refactor), 'style' (style), 'test' (test). The message should clearly state the main purpose and impact of this change, without unnecessary details. For example:\n\nfeat: add multi-language commit message support\nfix: resolve issue with API key configuration not saving\n\nPlease generate a message matching the above style:";

const PROMPT_JP: &str = "以下のgitの差分内容と直近5件のコミットメッセージを参考に、統一感のある簡潔なGitコミットメッセージを日本語のみで生成してください（他言語を混ぜない）。変更内容に応じて、次のいずれかの接頭辞で始めてください：'build'（ビルドシステム）、'chore'（その他）、'ci'（継続的インテグレーション）、'docs'（ドキュメント）、'feat'（新機能）、'fix'（修正）、'perf'（パフォーマンス）、'refactor'（リファクタリング）、'style'（スタイル）、'test'（テスト）。変更の主旨と影響を簡潔に伝え、冗長な説明や不要な詳細は避けてください。例：\n\nfeat: 多言語コミットメッセージ生成をサポート\nfix: APIキー設定が保存できない問題を修正\n\n上記のスタイルに合わせてコミットメッセージを生成してください：";

const PROMPT_ZH_TW: &str = "請根據以下git差異內容，以及最近5筆提交訊息，生成一條風格一致、語言純正且精簡的Git提交訊息。請只用繁體中文描述，避免中英混合。根據此次更改的性質，從下列前綴選擇一個作為開頭：'build'（構建系統）、'chore'（雜務）、'ci'（持續整合）、'docs'（文件）、'feat'（新功能）、'fix'（修復）、'perf'（效能）、'refactor'（重構）、'style'（樣式）、'test'（測試）。訊息需明確說明此更動的主要目的與影響，避免冗長或無關資訊。範例：\n\nfeat: 支援多語言提交訊息生成\nfix: 修正API金鑰設定無法儲存的問題\n\n請產生一條符合上述風格的提交訊息：";

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

    let key = match validate_config_key(key) {
        Ok(valid_key) => valid_key,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                e,
            )))
        }
    };

    match key {
        VALID_OPENAI_API_KEY => config.openai_api_key = value.to_string(),
        VALID_OPENAI_URL => config.openai_url = value.to_string(),
        VALID_OPENAI_MODEL => config.openai_model = value.to_string(),
        VALID_USER_LANGUAGE => config.user_language = value.to_string(),
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid configuration key",
            )))
        }
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
        openai_model: "gpt-4o-mini".to_string(),
        user_language: "en".to_string(),
    };
    toml::to_string(&config).expect("Could not serialize config")
}

pub fn write_config_to_toml(config_toml: &str, path: &Path) -> Result<(), Error> {
    fs::write(path, config_toml).expect("Could not write to config file");
    Ok(())
}

pub fn ensure_config_initialized<P: AsRef<Path>>(
    config_dir: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir_path = config_dir.as_ref();
    let config_file_path = config_dir_path.join("config.toml");
    let prompt_file_path = config_dir_path.join("prompt.toml");

    // Ensure the directory exists
    fs::create_dir_all(config_dir_path)?;

    // Initialize config.toml if it doesn't exist
    if !config_file_path.exists() {
        let default_config = generate_config_toml();
        fs::write(&config_file_path, default_config)?;
        println!(
            "Created new project config at: {}",
            config_file_path.display()
        );
    }

    // Initialize prompt.toml if it doesn't exist
    if !prompt_file_path.exists() {
        move_prompt_toml(&prompt_file_path);
        println!(
            "Created new prompt config at: {}",
            prompt_file_path.display()
        );
    }

    Ok(())
}
