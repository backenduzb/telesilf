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

pub fn is_portfolio_request(text: &str) -> bool {
    static PATTERNS: OnceLock<RegexSet> = OnceLock::new();

    let set = PATTERNS.get_or_init(|| {
        RegexSet::new(&[
            r"(?i)\bportfolio\b",
            r"(?i)\bportfoliongiz\b",
            r"(?i)\bportfolioingiz\b",
            r"(?i)\bportfolio\s*(bormi|ber(ing)?|yubor(ing)?|ko'?rsat(ing)?|tashla)\b",
            r"(?i)\bishlaringizni\s*(ko'?rsat(ing)?|yubor(ing)?)\b",
            r"(?i)\bavvalgi\s*ish(lar(ingiz)?)?\b",
            r"(?i)\bqilgan\s*ish(lar(ingiz)?)?\b",
            r"(?i)\bnamuna\s*ish(lar)?\b",
            r"(?i)\bmisol\s*ish(lar)?\b",
            r"(?i)\breferens\b",
            r"(?i)\bexample\s*project\b",

            r"(?i)\bпортфолио\b",
            r"(?i)\bпортфолионгиз\b",
            r"(?i)\bпортфолиони\s*(беринг|юборинг|кўрсатинг)\b",
            r"(?i)\bишларингизни\s*(кўрсатинг|юборинг)\b",
            r"(?i)\bаввалги\s*ишлар\b",
            r"(?i)\bқилган\s*ишлар\b",
            r"(?i)\bнамуна\s*ишлар\b",
            r"(?i)\bмисол\s*ишлар\b",
            r"(?i)\bреференс\b",

            r"(?i)\bпортфолио\b",
            r"(?i)\bпокажи(те)?\s*портфолио\b",
            r"(?i)\bпришли(те)?\s*портфолио\b",
            r"(?i)\bесть\s*портфолио\b",
            r"(?i)\bможно\s*портфолио\b",
            r"(?i)\bваши\s*работы\b",
            r"(?i)\bпримеры?\s*работ\b",
            r"(?i)\bпредыдущие\s*работы\b",
            r"(?i)\bкейсы\b",
            r"(?i)\bреференсы\b",
            r"(?i)\bпримеры?\s*проектов\b",

            r"(?i)\bportfolio\b",
            r"(?i)\bshow\s*(me\s*)?(your\s*)?portfolio\b",
            r"(?i)\bsend\s*(me\s*)?(your\s*)?portfolio\b",
            r"(?i)\bcan\s*i\s*see\s*(your\s*)?portfolio\b",
            r"(?i)\bmay\s*i\s*see\s*(your\s*)?portfolio\b",
            r"(?i)\bdo\s*you\s*have\s*a\s*portfolio\b",
            r"(?i)\bportfolio\s*link\b",
            r"(?i)\bportfolio\s*please\b",
            r"(?i)\bprevious\s*work\b",
            r"(?i)\bpast\s*projects\b",
            r"(?i)\bexamples?\s*of\s*(your\s*)?work\b",
            r"(?i)\bsamples?\s*of\s*(your\s*)?work\b",
            r"(?i)\bshow\s*(me\s*)?(your\s*)?work\b",
            r"(?i)\byour\s*work\b",
            r"(?i)\bwork\s*examples\b",
            r"(?i)\bcase\s*stud(y|ies)\b",
            r"(?i)\breferences?\b",
        ])
        .unwrap()
    });

    set.is_match(text)
}