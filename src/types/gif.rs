#[allow(non_camel_case_types)]
#[derive(poise::ChoiceParameter)]
pub enum Gif {
    cityslop,
    metagame,
    plotslop,
    pointslop,
    slopslop,
    stallslop,
    walkslop,
}

impl Gif {
    pub fn link(&self) -> &str {
        // I will NOT be making my code less shitty thank you very much
        match self {
            Self::cityslop  => "https://media.discordapp.net/attachments/1276346665887862918/1506794404839686285/caption.gif?ex=6a0f8f25&is=6a0e3da5&hm=d242a2979eaefd7a88ce4162404f59aa202bc472aa13d1b6472095e7f9125a39&=&width=334&height=235",
            Self::metagame  => "https://cdn.discordapp.com/attachments/1336169907896844391/1523915062488727725/caption.gif?ex=6a4dd7ff&is=6a4c867f&hm=249e5d8d5a236f6609de46392abf0ac0dd58ab2635ad8756da8fd1c991c23f48&",
            Self::plotslop  => "https://media.discordapp.net/attachments/1276346665887862918/1506796988916695060/caption.gif?ex=6a0f918d&is=6a0e400d&hm=db5afa255af1f618eb1f254b790f0ba083de44dfdcd0d104db6e6b23127290cb&=&width=225&height=377",
            Self::pointslop => "https://media.discordapp.net/attachments/1276346665887862918/1508720134158684160/caption.gif?ex=6a16909e&is=6a153f1e&hm=612f61147c61b26ca4849874cc836ebbb8e847d40560f119f903010968ab3e8b&=&width=334&height=286",
            Self::slopslop  => "https://cdn.discordapp.com/attachments/1276346665887862918/1508720536237379654/caption.gif?ex=6a1690fe&is=6a153f7e&hm=3f0e58183d486cdc8bd9d4c1f2e9b2986597fec30ce935205bf8456e0e81414f&",
            Self::stallslop => "https://cdn.discordapp.com/attachments/1447058975785877614/1447450392785850539/caption.gif?ex=6a0f37f9&is=6a0de679&hm=713f359ac320252689a239c64732bd3741127e1fec8333e8a91960c35fd14ad2&",
            Self::walkslop  => "https://media.discordapp.net/attachments/1443451488360988762/1443452496944304262/caption.gif?ex=6a0f2d25&is=6a0ddba5&hm=298a773384a2f81970691f8e0e2dee234577f246d67876378997b5191e5d6cc8&=&width=188&height=371",
        }
    }
}
