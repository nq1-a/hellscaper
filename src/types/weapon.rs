use crate::types::traits::Bias;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, poise::ChoiceParameter)]
pub enum Weapon {
    bow,
    crossbow,
    #[name = "machine gun"]
    machine,
    derringer,
    #[name = "pipe gun"]
    pipe,
    slingshot,
    #[name = "thrown misc."]
    thrown_misc,
    musket,
    shotgun,
    spit,
    #[name = "thrown blade"]
    thrown_blade,
    #[name = "zip gun"]
    zip,
    pistol,
    revolver,
    #[name = "smg"]
    submachine,
    #[name = "thrown expl."]
    thrown_expl,
    #[name = "assault rifle"]
    assault,
    #[name = "electron gun"]
    electron_gun,
    railgun,
    rifle,
    coilgun,
    varmint,
    sniper,
    laser,
    #[name = "server ban"]
    server_ban,
}

impl Weapon {
    pub fn auto(&self) -> bool {
        match self {
            Self::machine       => true,
            Self::submachine    => true,
            _                   => false
        }
    }

    pub fn experimental(&self) -> bool {
        match self {
            Self::coilgun       => true,
            Self::pipe          => true,
            Self::railgun       => true,
            Self::zip           => true,
            _                   => false
        }
    }

    pub fn jam_msg(&self) -> &str {
        if !self.jammable() {return "MISS";}

        match self {
            Self::pipe          => "BOOM!",
            Self::server_ban    => "DEMOTED",
            Self::spit          => "TOO DRY",
            _                   => "JAMMED"
        }
    }

    pub fn jammable(&self) -> bool {
        match self {
            Self::bow           => false,
            Self::crossbow      => false,
            Self::electron_gun  => false,
            Self::laser         => false,
            Self::slingshot     => false,
            Self::thrown_blade  => false,
            Self::thrown_expl   => false,
            Self::thrown_misc   => false,
            _                   => true
        }
    }
}

impl Bias for Weapon {
    fn bias(&self) -> i32 {
        match self {
            Self::derringer     => -4,
            Self::machine       => -3,
            Self::slingshot     => -3,
            Self::pipe          => -2,
            Self::thrown_misc   => -2,
            Self::musket        => -1,
            Self::spit          => -1,
            Self::zip           => -1,
            Self::bow           =>  0,
            Self::revolver      =>  0,
            Self::shotgun       =>  0,
            Self::thrown_blade  =>  0,
            Self::crossbow      =>  1,
            Self::pistol        =>  1,
            Self::railgun       =>  1,
            Self::rifle         =>  1,
            Self::submachine    =>  1,
            Self::assault       =>  2,
            Self::electron_gun  =>  2,
            Self::thrown_expl   =>  2,
            Self::coilgun       =>  3,
            Self::varmint       =>  3,
            Self::laser         =>  4,
            Self::sniper        =>  4,
            Self::server_ban    =>  999,
        }
    }
}
