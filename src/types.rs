#[allow(non_camel_case_types)]
#[derive(Debug, poise::ChoiceParameter)]
pub enum Weapon {
    machine,
    derringer,
    pipe,
    slingshot,
    #[name = "thrown misc"]
    thrown_misc,
    musket,
    shotgun,
    spit,
    #[name = "thrown blade"]
    thrown_blade,
    zip,
    pistol,
    revolver,
    submachine,
    #[name = "thrown expl."]
    thrown_expl,
    assault,
    carbine,
    #[name = "electron gun"]
    electron_gun,
    railgun,
    rifle,
    coilgun,
    dmr,
    varmint,
    sniper,
    laser,
    #[name = "server ban"]
    server_ban,
}

impl Weapon {
    pub fn bias(&self) -> i32 {
        match self {
            Self::machine       => -6,
            Self::derringer     => -4,
            Self::pipe          => -3,
            Self::shotgun       => -2,
            Self::slingshot     => -2,
            Self::thrown_misc   => -2,
            Self::zip           => -2,
            Self::musket        => -1,
            Self::spit          => -1,
            Self::thrown_blade  => -1,
            Self::pistol        =>  0,
            Self::revolver      =>  0,
            Self::submachine    =>  0,
            Self::carbine       =>  1,
            Self::railgun       =>  1,
            Self::rifle         =>  1,
            Self::assault       =>  2,
            Self::dmr           =>  2,
            Self::electron_gun  =>  2,
            Self::thrown_expl   =>  2,
            Self::coilgun       =>  3,
            Self::varmint       =>  3,
            Self::laser         =>  4,
            Self::sniper        =>  5,
            Self::server_ban    =>  999,
        }
    }

    pub fn jammable(&self) -> bool {
        match self {
            Self::pipe          => false,
            Self::server_ban    => false,
            Self::slingshot     => false,
            Self::spit          => false,
            Self::thrown_blade  => false,
            Self::thrown_expl   => false,
            Self::thrown_misc   => false,
            _                   => true
        }
    }
}
