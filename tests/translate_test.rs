use anyhow::Result;

// 从 main binary 中导入 Translate
use translate::Translate;

#[test]
fn test_translate_new() {
    let translate = Translate::new("en");
    assert_eq!(translate.lang, "en");
    assert!(!translate.user_agents.is_empty());
}

#[test]
fn test_get_user_agent() {
    let translate = Translate::new("en");
    let agent = translate.get_user_agent();
    assert!(!agent.is_empty());
    assert!(translate.user_agents.contains(&agent));
}

#[test]
fn test_generate_url() {
    let translate = Translate::new("zh-CN");
    let url = translate.generate_url("hello");
    assert!(url.contains("tl=zh-CN"));
    assert!(url.contains("q=hello"));
    assert!(url.starts_with("https://translate.googleapis.com"));
}

#[test]
fn test_real_translation() -> Result<()> {
    let translate = Translate::new("zh-CN");
    let (lang, translations) = translate.get_translation("hello")?;

    assert_eq!(lang, "en");
    assert!(!translations.is_empty());
    assert!(translations[0].contains("你好"));

    Ok(())
}

#[test]
fn test_chinese_to_english() -> Result<()> {
    let translate = Translate::new("en");
    let (lang, translations) = translate.get_translation("你好")?;

    assert_eq!(lang, "zh-CN");
    assert!(!translations.is_empty());
    assert!(translations[0].to_lowercase().contains("hello"));

    Ok(())
}
