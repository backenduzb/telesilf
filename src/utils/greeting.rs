use regex::RegexSet;
use std::sync::OnceLock;

pub fn is_greeting_or_appeal(text: &str) -> bool {
    static PATTERNS: OnceLock<RegexSet> = OnceLock::new();

    let set = PATTERNS.get_or_init(|| {
        RegexSet::new(&[
            r"(?i)\b(salom|assalom|assaalom|assalomu\s*alaykum|assalomu\s*alaikum)\b",
            r"(?i)\b(салом|ассалом|ассалому\s*алайкум|ассалому\s*алайкум)\b",
            r"(?i)\b(qandaysiz|yaxshimisiz|ishlar\s*qanday|kallamysiz|qale|qalesiz)\b",
            r"(?i)\b(қандайсиз|яхшимисиз|ишлар\s*қандай|қале|қалесиз)\b",
            r"(?i)\b(uzr|kechirasiz|murojaat|savol|birodar|og'ayni|ukam|aka|uka)\b",
            r"(?i)\b(узр|кечирасиз|мурожаат|савол|биродар|оғайни|укам|ака|ука)\b",

            r"(?i)\b(привет|приветик|здравствуй|здравствуйте|здорово|здарова)\b",
            r"(?i)\b(добрый\s*(день|вечер|утро)|доброе\s*утро)\b",
            r"(?i)\b(как\s*дела|как\s*жизнь|как\s*оно|как\s*вы|извините|простите|вопрос)\b",

            r"(?i)\b(hi|hello|hey|heyy|greetings|sup|yo)\b",
            r"(?i)\b(good\s*(morning|afternoon|evening))\b",
            r"(?i)\b(how\b.*\b(are\s*you|doing|it\s*goes)|excuse\s*me|question)\b",
        ])
        .unwrap()
    });

    set.is_match(text)
}