use anyhow::{anyhow, Result};
use clap::Parser;
use translate::Translate;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 要翻译的文本
    #[arg(value_name = "TEXT")]
    text: Vec<String>,

    /// 源语言 (例如: en, zh-CN, auto)
    #[arg(short, long, default_value = "auto")]
    source: String,

    /// 目标语言 (例如: en, zh-CN, auto)
    #[arg(short, long, default_value = "auto")]
    target: String,

    /// 显示支持的语言列表
    #[arg(short, long)]
    list: bool,
}

const LANGUAGES: &[&str] = &[
    "af", "sq", "am", "ar", "hy", "az", "eu", "be", "bn", "bs", "bg", "ca", "ceb", "zh-CN",
    "zh-TW", "co", "hr", "cs", "da", "nl", "en", "eo", "et", "fi", "fr", "fy", "gl", "ka", "de",
    "el", "gu", "ht", "ha", "haw", "he", "hi", "hmn", "hu", "is", "ig", "id", "ga", "it", "ja",
    "jv", "kn", "kk", "km", "ko", "ku", "ky", "lo", "la", "lv", "lt", "lb", "mk", "mg", "ms", "ml",
    "mt", "mi", "mr", "mn", "my", "ne", "no", "ny", "or", "ps", "fa", "pl", "pt", "pa", "ro", "ru",
    "sm", "gd", "sr", "st", "sn", "sd", "si", "sk", "sl", "so", "es", "su", "sw", "sv", "tl", "tg",
    "ta", "tt", "te", "th", "tr", "tk", "uk", "ur", "ug", "uz", "vi", "cy", "xh", "yi", "yo", "zu",
];

const LANGUAGE_NAMES: &[(&str, &str)] = &[
    ("af", "Afrikaans"),
    ("sq", "Albanian"),
    ("am", "Amharic"),
    ("ar", "Arabic"),
    ("hy", "Armenian"),
    ("az", "Azerbaijani"),
    ("eu", "Basque"),
    ("be", "Belarusian"),
    ("bn", "Bengali"),
    ("bs", "Bosnian"),
    ("bg", "Bulgarian"),
    ("ca", "Catalan"),
    ("ceb", "Cebuano"),
    ("zh-CN", "Chinese (Simplified)"),
    ("zh-TW", "Chinese (Traditional)"),
    ("co", "Corsican"),
    ("hr", "Croatian"),
    ("cs", "Czech"),
    ("da", "Danish"),
    ("nl", "Dutch"),
    ("en", "English"),
    ("eo", "Esperanto"),
    ("et", "Estonian"),
    ("fi", "Finnish"),
    ("fr", "French"),
    ("fy", "Frisian"),
    ("gl", "Galician"),
    ("ka", "Georgian"),
    ("de", "German"),
    ("el", "Greek"),
    ("gu", "Gujarati"),
    ("ht", "Haitian Creole"),
    ("ha", "Hausa"),
    ("haw", "Hawaiian"),
    ("he", "Hebrew"),
    ("hi", "Hindi"),
    ("hmn", "Hmong"),
    ("hu", "Hungarian"),
    ("is", "Icelandic"),
    ("ig", "Igbo"),
    ("id", "Indonesian"),
    ("ga", "Irish"),
    ("it", "Italian"),
    ("ja", "Japanese"),
    ("jv", "Javanese"),
    ("kn", "Kannada"),
    ("kk", "Kazakh"),
    ("km", "Khmer"),
    ("ko", "Korean"),
    ("ku", "Kurdish"),
    ("ky", "Kyrgyz"),
    ("lo", "Lao"),
    ("la", "Latin"),
    ("lv", "Latvian"),
    ("lt", "Lithuanian"),
    ("lb", "Luxembourgish"),
    ("mk", "Macedonian"),
    ("mg", "Malagasy"),
    ("ms", "Malay"),
    ("ml", "Malayalam"),
    ("mt", "Maltese"),
    ("mi", "Maori"),
    ("mr", "Marathi"),
    ("mn", "Mongolian"),
    ("my", "Myanmar (Burmese)"),
    ("ne", "Nepali"),
    ("no", "Norwegian"),
    ("ny", "Nyanja (Chichewa)"),
    ("or", "Odia (Oriya)"),
    ("ps", "Pashto"),
    ("fa", "Persian"),
    ("pl", "Polish"),
    ("pt", "Portuguese"),
    ("pa", "Punjabi"),
    ("ro", "Romanian"),
    ("ru", "Russian"),
    ("sm", "Samoan"),
    ("gd", "Scots Gaelic"),
    ("sr", "Serbian"),
    ("st", "Sesotho"),
    ("sn", "Shona"),
    ("sd", "Sindhi"),
    ("si", "Sinhala (Sinhalese)"),
    ("sk", "Slovak"),
    ("sl", "Slovenian"),
    ("so", "Somali"),
    ("es", "Spanish"),
    ("su", "Sundanese"),
    ("sw", "Swahili"),
    ("sv", "Swedish"),
    ("tl", "Tagalog (Filipino)"),
    ("tg", "Tajik"),
    ("ta", "Tamil"),
    ("tt", "Tatar"),
    ("te", "Telugu"),
    ("th", "Thai"),
    ("tr", "Turkish"),
    ("tk", "Turkmen"),
    ("uk", "Ukrainian"),
    ("ur", "Urdu"),
    ("ug", "Uyghur"),
    ("uz", "Uzbek"),
    ("vi", "Vietnamese"),
    ("cy", "Welsh"),
    ("xh", "Xhosa"),
    ("yi", "Yiddish"),
    ("yo", "Yoruba"),
    ("zu", "Zulu"),
];

fn print_languages() {
    println!("支持的语言列表：");
    for (code, name) in LANGUAGE_NAMES {
        println!("  {} - {}", code, name);
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.list {
        print_languages();
        return Ok(());
    }

    if args.text.is_empty() {
        return Err(anyhow!("请输入要翻译的文本"));
    }

    // 检查语言设置是否有效
    if args.target != "auto" && !LANGUAGES.contains(&args.target.as_str()) {
        return Err(anyhow!("不支持的目标语言: {}", args.target));
    }

    if args.source != "auto" && !LANGUAGES.contains(&args.source.as_str()) {
        return Err(anyhow!("不支持的源语言: {}", args.source));
    }

    let text = args.text.join(" ");
    
    // 如果没有指定目标语言，根据检测到的语言自动选择
    let translate = Translate::new(if args.target == "auto" { "en" } else { &args.target });
    let (detected_lang, translations) = translate.get_translation(&text)?;

    // 如果检测到的语言与目标语言相同，或目标语言为 auto 且检测到中文
    if (args.target != "auto" && detected_lang == args.target) || 
       (args.target == "auto" && detected_lang == "zh-CN") {
        let target_lang = if args.source != "auto" { &args.source } else { "en" };
        let translate = Translate::new(target_lang);
        let (_, translations) = translate.get_translation(&text)?;
        print_translations(&translations);
    } else {
        print_translations(&translations);
    }

    Ok(())
}

fn print_translations(translations: &[String]) {
    for translation in translations {
        println!("{}", translation);
    }
}
