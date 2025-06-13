enum LordEnum {
    Date = 0,
    Uesugi,
    Takeda,
    Hojo,
    Tokugawa,
    Oda,
    Ashikaga,
    Mori,
    Chosokabe,
    Simazu,
    Max,
}

struct Lord {
    family_name: String,
    first_name: String,
}

impl Lord {
    pub fn new(family_name: &str, first_name: &str) -> Self {
        Self {
            family_name: family_name.to_string(),
            first_name: first_name.to_string(),
        }
    }
}

struct Context {
    lords: [Lord; LordEnum::Max as usize],
}

impl Context {
    pub fn new() -> Self {
        Self {
            lords: [
                Lord::new("다테", "테루무네"),
                Lord::new("우에스기", "겐신"),
                Lord::new("다케다", "신겐"),
                Lord::new("호조", "우지마사"),
                Lord::new("도쿠가와", "이에야스"),
                Lord::new("오다", "노부나가"),
                Lord::new("아시카가", "요시아키"),
                Lord::new("모리", "모토나리"),
                Lord::new("조소카베", "모토치카"),
                Lord::new("시마즈", "요시히사"),
            ],
        }
    }
}
fn main() {}
