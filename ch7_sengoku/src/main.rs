const TROOP_BASE: usize = 5;
const START_YEAR: u16 = 1570;

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

enum CastleEnum {
    Yonezawa = 0,
    Kasugayama,
    Tsutsujigasaki,
    Odawara,
    Okazaki,
    Gifu,
    Nijo,
    Yoshidakoriyama,
    Oko,
    Uchi,
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

struct Castle {
    name: String,
    owner: LordEnum,
    troop_count: usize,
}

impl Castle {
    pub fn new(name: &str, owner: LordEnum, troop_count: usize) -> Self {
        Self {
            name: name.to_string(),
            owner,
            troop_count,
        }
    }
}

struct Context {
    lords: [Lord; LordEnum::Max as usize],
    castles: [Castle; CastleEnum::Max as usize],
    year: u16,
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
            castles: [
                Castle::new("요네자와성", LordEnum::Date, TROOP_BASE),
                Castle::new("가스가야마성", LordEnum::Uesugi, TROOP_BASE),
                Castle::new("쓰쓰지가사키관", LordEnum::Takeda, TROOP_BASE),
                Castle::new("오다와라성", LordEnum::Hojo, TROOP_BASE),
                Castle::new("오카자키성", LordEnum::Tokugawa, TROOP_BASE),
                Castle::new("기후성", LordEnum::Oda, TROOP_BASE),
                Castle::new("니조성", LordEnum::Ashikaga, TROOP_BASE),
                Castle::new("요시다고리야마성", LordEnum::Mori, TROOP_BASE),
                Castle::new("오코성", LordEnum::Chosokabe, TROOP_BASE),
                Castle::new("우찌성", LordEnum::Simazu, TROOP_BASE),
            ],
            year: 0,
        }
    }

    pub fn init(&mut self) {
        self.year = START_YEAR;
    }
}

fn main() {
    let mut ctx = Context::new();
    ctx.init();
    // loop {}
}
