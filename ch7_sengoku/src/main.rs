use std::io;

use getch_rs::{Getch, Key};
use rand::{Rng, rngs::ThreadRng, seq::IndexedRandom};

const TROOP_BASE: usize = 5;
const TROOP_MAX: usize = 9;
const TROOP_UNIT: usize = 1000;
const START_YEAR: u16 = 1570;
// const CHRONOLOGY_MAX: usize = 1024;

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, Debug)]
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
    connected_castles: Vec<CastleEnum>,
}

impl Castle {
    pub fn new(
        name: &str,
        owner: LordEnum,
        troop_count: usize,
        connected_castles: Vec<CastleEnum>,
    ) -> Self {
        Self {
            name: name.to_string(),
            owner,
            troop_count,
            connected_castles,
        }
    }
}

fn get_first_n_chars(s: &str, n: usize) -> String {
    let mut chars = s.char_indices();
    match chars.nth(n) {
        Some(index) => s[..index.0].to_string(),
        None => s.to_string(),
    }
}

struct Context {
    lords: [Lord; LordEnum::Max as usize],
    castles: [Castle; CastleEnum::Max as usize],
    year: u16,
    player_lord: LordEnum,
    chronology: String,
    rng: ThreadRng,
    g: Getch,
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
                Castle::new(
                    "요네자와성",
                    LordEnum::Date,
                    TROOP_BASE,
                    vec![CastleEnum::Kasugayama, CastleEnum::Odawara],
                ),
                Castle::new(
                    "가스가야마성",
                    LordEnum::Uesugi,
                    TROOP_BASE,
                    vec![
                        CastleEnum::Yonezawa,
                        CastleEnum::Tsutsujigasaki,
                        CastleEnum::Gifu,
                    ],
                ),
                Castle::new(
                    "쓰쓰지가사키관",
                    LordEnum::Takeda,
                    TROOP_BASE,
                    vec![
                        CastleEnum::Kasugayama,
                        CastleEnum::Odawara,
                        CastleEnum::Okazaki,
                    ],
                ),
                Castle::new(
                    "오다와라성",
                    LordEnum::Hojo,
                    TROOP_BASE,
                    vec![
                        CastleEnum::Yonezawa,
                        CastleEnum::Tsutsujigasaki,
                        CastleEnum::Okazaki,
                    ],
                ),
                Castle::new(
                    "오카자키성",
                    LordEnum::Tokugawa,
                    TROOP_BASE,
                    vec![
                        CastleEnum::Tsutsujigasaki,
                        CastleEnum::Odawara,
                        CastleEnum::Gifu,
                    ],
                ),
                Castle::new(
                    "기후성",
                    LordEnum::Oda,
                    TROOP_BASE,
                    vec![
                        CastleEnum::Kasugayama,
                        CastleEnum::Okazaki,
                        CastleEnum::Nijo,
                    ],
                ),
                Castle::new(
                    "니조성",
                    LordEnum::Ashikaga,
                    TROOP_BASE,
                    vec![
                        CastleEnum::Gifu,
                        CastleEnum::Yoshidakoriyama,
                        CastleEnum::Oko,
                    ],
                ),
                Castle::new(
                    "요시다고리야마성",
                    LordEnum::Mori,
                    TROOP_BASE,
                    vec![CastleEnum::Nijo, CastleEnum::Oko, CastleEnum::Uchi],
                ),
                Castle::new(
                    "오코성",
                    LordEnum::Chosokabe,
                    TROOP_BASE,
                    vec![
                        CastleEnum::Nijo,
                        CastleEnum::Yoshidakoriyama,
                        CastleEnum::Uchi,
                    ],
                ),
                Castle::new(
                    "우찌성",
                    LordEnum::Simazu,
                    TROOP_BASE,
                    vec![CastleEnum::Yoshidakoriyama, CastleEnum::Oko],
                ),
            ],
            year: 0,
            player_lord: LordEnum::Max,
            chronology: String::new(),
            rng: rand::rng(),
            g: Getch::new(),
        }
    }

    pub fn init(&mut self) {
        self.year = START_YEAR;

        self.chronology.clear();

        self.draw_screen();
    }

    fn draw_screen(&self) {
        clearscreen::clear().unwrap();
        // println!(
        //     "1570년　～～～～～～～～～～～～～～～～　　　　　　～\n\
        //     　　　　　～～～～～～～～～～～～～～～～　0요네5　～\n\
        //     ～～～～～～～～～～～～～～～～～～1가스5　다테　～～\n\
        //     ～～～～～～～～～～～～～～～　～～우에　　　　　～～\n\
        //     ～～～～～～～～～～～～～～～　～　　　　　　　　～～\n\
        //     ～～～～～～～～～～～～～～　　　　　2쓰쓰5　　　～～\n\
        //     ～～～～～～～～～～～～～　　　　　　다케　　　～～～\n\
        //     ～～～～～～　　　　　　　5기후5　　　　　　　　～～～\n\
        //     ～～～～　7요시5　6니조5　오다　4오카5　3오다5　～～～\n\
        //     ～～～　　모리　　아시　　　　　토쿠　　호조～～～～～\n\
        //     ～～　～～～～～～～　　　～～～～～～～～～～～～～～\n\
        //     ～　　　～　8오코5～～　～～～～～～～～～～～～～～～\n\
        //     ～　　　～～조소～～～～～～～～～～～～～～～～～～～\n\
        //     ～9우찌5～～～～～～～～～～～～～～～～～～～～～～～\n\
        //     ～시마～～～～～～～～～～～～～～～～～～～～～～～～\n\
        //     ～～～～～～～～～～～～～～～～～～～～～～～～～～～\n"
        // ); // 16

        // [6-2-2]지도의 1번째 행을 그린다
        println!(
            "{}년　　～～～～～～～～～～～～～～～～　　　　　～",
            self.year,
        ); // 년

        // [6-2-3]지도의 2번째 행을 그린다
        println!(
            "　　　　　～～～～～～～～～～～～～～～～　{}{:2}{}　～",
            // 요네자와성의 성 번호
            CastleEnum::Yonezawa as usize,
            // 요네자와성의 이름
            get_first_n_chars(&self.castles[CastleEnum::Yonezawa as usize].name, 2),
            // 요네자와성의 병력 수
            self.castles[CastleEnum::Yonezawa as usize].troop_count,
        );

        // [6-2-4]지도의 3번째 행을 그린다
        println!(
            "～～～～～～～～～～～～～～～～～～{}{:2}{}　{:2}　～～",
            // 가스가야마성의 성 번호
            CastleEnum::Kasugayama as usize,
            // 가스가야마성의 이름
            get_first_n_chars(&self.castles[CastleEnum::Kasugayama as usize].name, 2),
            // 가스가야마성의 병력 수
            self.castles[CastleEnum::Kasugayama as usize].troop_count,
            // 요네자와성의 성씨
            get_first_n_chars(
                &self.lords[self.castles[CastleEnum::Yonezawa as usize].owner as usize].family_name,
                2
            ),
        );

        // [6-2-5]지도의 4번째 행을 그린다
        println!(
            "～～～～～～～～～～～～～～～　～～{:2}　　　　　～～",
            // 가스가야마성의 성씨
            get_first_n_chars(
                &self.lords[self.castles[CastleEnum::Kasugayama as usize].owner as usize]
                    .family_name,
                2
            ),
        );

        // [6-2-6]지도의 5번째 행을 그린다
        println!("～～～～～～～～～～～～～～～　～　　　　　　　　～～");

        // [6-2-7]지도의 6번째 행을 그린다
        println!(
            "～～～～～～～～～～～～～～　　　　　{}{:2}{}　　　～～",
            // 쓰쓰지가사키관의 성 번호
            CastleEnum::Tsutsujigasaki as usize,
            // 쓰쓰지가사키관의 이름
            get_first_n_chars(&self.castles[CastleEnum::Tsutsujigasaki as usize].name, 2),
            // 쓰쓰지가사키관의 병력 수
            self.castles[CastleEnum::Tsutsujigasaki as usize].troop_count,
        );

        // [6-2-8]지도의 7번째 행을 그린다
        println!(
            "～～～～～～～～～～～～～　　　　　　{:2}　　　～～～",
            // 쓰쓰지가사키관의 성주의 성씨
            get_first_n_chars(
                &self.lords[self.castles[CastleEnum::Tsutsujigasaki as usize].owner as usize]
                    .family_name,
                2
            ),
        );

        // [6-2-9]지도의 8번째 행을 그린다
        println!(
            "～～～～～～　　　　　　　{}{:2}{}　　　　　　　　～～～",
            // 기후성의 성 번호
            CastleEnum::Gifu as usize,
            // 기후성의 이름
            get_first_n_chars(&self.castles[CastleEnum::Gifu as usize].name, 2),
            // 기후성의 병력 수
            self.castles[CastleEnum::Gifu as usize].troop_count,
        );

        // [6-2-10]지도의 9번째 행을 그린다
        println!(
            "～～～～　{}{:2}{}　{}{:2}{}　{:2}　　　　　{}{:2}{}　～～～",
            // 요시다고리야마성의 성 번호
            CastleEnum::Yoshidakoriyama as usize,
            // 요시다고리야마성의 이름
            get_first_n_chars(&self.castles[CastleEnum::Yoshidakoriyama as usize].name, 2),
            // 요시다고리야마성의 병력 수
            self.castles[CastleEnum::Yoshidakoriyama as usize].troop_count,
            // 니조성의 성 번호
            CastleEnum::Nijo as usize,
            // 니조성의 이름
            get_first_n_chars(&self.castles[CastleEnum::Nijo as usize].name, 2),
            // 니조성의 병력 수
            self.castles[CastleEnum::Nijo as usize].troop_count,
            // 기후성 성주의 성
            get_first_n_chars(
                &self.lords[self.castles[CastleEnum::Gifu as usize].owner as usize].family_name,
                2
            ),
            // 오다와라성의 성 번호
            CastleEnum::Odawara as usize,
            // 오다와라성의 이름
            get_first_n_chars(&self.castles[CastleEnum::Odawara as usize].name, 2),
            // 오다와라성의 병력 수
            self.castles[CastleEnum::Odawara as usize].troop_count,
        );

        // [6-2-11]지도의 10번째 행을 그린다
        println!(
            "～～～　　{:2}　　{:2}　～　　　{}{:2}{}　{:2}～～～～～",
            // 요시다고리야마성의 성주의 성씨
            get_first_n_chars(
                &self.lords[self.castles[CastleEnum::Yoshidakoriyama as usize].owner as usize]
                    .family_name,
                2
            ),
            // 니조성 성주의 성씨
            get_first_n_chars(
                &self.lords[self.castles[CastleEnum::Nijo as usize].owner as usize].family_name,
                2
            ),
            // 오카자키성의 성 번호
            CastleEnum::Okazaki as usize,
            // 오카자키성의 이름
            get_first_n_chars(&self.castles[CastleEnum::Okazaki as usize].name, 2),
            // 오카자키성의 병력 수
            self.castles[CastleEnum::Okazaki as usize].troop_count,
            // 오다와라성 성주의 성씨
            get_first_n_chars(
                &self.lords[self.castles[CastleEnum::Odawara as usize].owner as usize].family_name,
                2
            ),
        );

        // [6-2-12]지도의 11번째 행을 그린다
        println!(
            "～～　～～～～～～～　　　　～～{:2}～　～　～～～～～",
            // 오카자키성 성주의 성씨
            get_first_n_chars(
                &self.lords[self.castles[CastleEnum::Okazaki as usize].owner as usize].family_name,
                2
            ),
        );

        // [6-2-13]지도의 12번째 행을 그린다
        println!(
            "～　　　～　{}{:2}{}　～　　　　～～～～～～～～～～～～",
            // 오코성의 성 번호
            CastleEnum::Oko as usize,
            // 오코성의 이름
            get_first_n_chars(&self.castles[CastleEnum::Oko as usize].name, 2),
            // 오코성의 병력 수
            self.castles[CastleEnum::Oko as usize].troop_count,
        );

        // [6-2-14]지도의 13번째 행을 그린다
        println!(
            "～　　　～　{:2}　～～　　～～～～～～～～～～～～～～",
            // 오코성 성주의 성씨
            get_first_n_chars(
                &self.lords[self.castles[CastleEnum::Oko as usize].owner as usize].family_name,
                2
            ),
        );

        // [6-2-15]지도의 14번째 행을 그린다
        println!(
            "～{}{:2}{}～～～～～～～～～～～～～～～～～～～～～～～",
            // 우찌성의 성 번호
            CastleEnum::Uchi as usize,
            // 우찌성의 이름
            get_first_n_chars(&self.castles[CastleEnum::Uchi as usize].name, 2),
            // 우찌성의 병력 수
            self.castles[CastleEnum::Uchi as usize].troop_count,
        );

        // [6-2-16]지도의 15번째 행을 그린다
        println!(
            "～{:2}～～～～～～～～～～～～～～～～～～～～～～～～",
            // 우찌성 성주의 성씨
            get_first_n_chars(
                &self.lords[self.castles[CastleEnum::Uchi as usize].owner as usize].family_name,
                2
            ),
        );

        // [6-2-17]지도의 16번째 행을 그린다
        println!("～～～～～～～～～～～～～～～～～～～～～～～～～～～");

        // [6-2-18]1행 비워둔다
        println!();
    }

    fn siege(&mut self, offensive_lord: LordEnum, mut offensive_troop_count: usize, castle: usize) {
        clearscreen::clear().unwrap();

        println!("~{} 전투~\n", self.castles[castle].name);

        let defensive_lord = self.castles[castle].owner;
        loop {
            println!(
                "{}군({:4}명) X {}군({:4}명)\n",
                self.lords[offensive_lord as usize].family_name,
                offensive_troop_count * TROOP_UNIT,
                self.lords[defensive_lord as usize].family_name,
                self.castles[castle].troop_count * TROOP_UNIT,
            );

            if offensive_troop_count <= 0 || self.castles[castle].troop_count <= 0 {
                break;
            }

            if self.rng.random_range(0..3) == 0 {
                self.castles[castle].troop_count -= 1;
            } else {
                offensive_troop_count -= 1;
            }
        }
        self.pause_a_key();

        println!();

        if self.castles[castle].troop_count <= 0 {
            println!("{} 함락!!\n", self.castles[castle].name,);

            self.castles[castle].owner = offensive_lord;

            self.castles[castle].troop_count = offensive_troop_count;

            println!(
                "{}은(는) {} 가문의 것이 됩니다.\n",
                self.castles[castle].name, self.lords[offensive_lord as usize].family_name
            );

            if self.get_castle_count(defensive_lord) <= 0 {
                self.chronology.push_str(&format!(
                    "{}년 {}{}이(가) {}에서 {}{}을(를) 멸망시키다.\n",
                    self.year,
                    self.lords[offensive_lord as usize].family_name,
                    self.lords[offensive_lord as usize].first_name,
                    self.castles[castle].name,
                    self.lords[defensive_lord as usize].family_name,
                    self.lords[defensive_lord as usize].first_name,
                ));
            }
        } else {
            println!(
                "{}군 전멸!!\n",
                self.lords[offensive_lord as usize].family_name
            );

            println!(
                "{}군이 {}을(를) 지켜냈습니다!",
                self.lords[defensive_lord as usize].family_name, self.castles[castle].name,
            );
        }
    }

    fn pause_a_key(&self) {
        match self.g.getch() {
            Ok(Key::Esc) => {
                std::process::exit(0);
            }
            _ => {}
        }
    }

    fn get_castle_count(&self, lord: LordEnum) -> usize {
        let mut castle_count = 0;

        for i in 0..CastleEnum::Max as usize {
            if self.castles[i].owner == lord {
                castle_count += 1;
            }
        }
        castle_count
    }
}

fn input_number() -> usize {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(_) => continue,
            _ => {}
        }

        let num = match input.trim().parse::<usize>() {
            Ok(num) => num,
            _ => continue,
        };

        println!("{}", num);
        return num;
    }
}

fn main() {
    let mut ctx = Context::new();
    ctx.init();

    println!(
        "주군님, 우리 성은\n\
        이 지도의 어디에 있습니까? (0~{})]\n",
        CastleEnum::Max as usize - 1,
    );

    let mut selected_castle: usize;
    loop {
        selected_castle = input_number();

        if selected_castle < CastleEnum::Max as usize {
            break;
        }
    }

    ctx.player_lord = ctx.castles[selected_castle].owner;

    println!(
        "{}님, {}에서 천하 통일을\n\
        목표로 합시다!",
        ctx.lords[ctx.player_lord as usize].first_name, ctx.castles[ctx.player_lord as usize].name
    );

    ctx.pause_a_key();

    loop {
        let mut turn_order = [0_usize; CastleEnum::Max as usize];
        for i in 0..CastleEnum::Max as usize {
            turn_order[i] = i;
        }

        for i in 0..CastleEnum::Max as usize {
            turn_order.swap(i, ctx.rng.random_range(0..CastleEnum::Max as usize))
        }

        for i in 0..CastleEnum::Max as usize {
            ctx.draw_screen();

            for j in 0..CastleEnum::Max as usize {
                print!("{}", if j == i { ">" } else { " " },);
                print!(
                    "{:2}",
                    get_first_n_chars(&ctx.castles[turn_order[j]].name, 2),
                );
            }

            println!("\n");

            let current_castle = turn_order[i];

            ctx.pause_a_key();

            println!(
                "{} 가문의 {} 전략회의...",
                ctx.lords[ctx.castles[current_castle].owner as usize].family_name,
                ctx.castles[current_castle].name,
            );

            if ctx.castles[current_castle].owner == ctx.player_lord {
                println!(
                    "{}님, 어디로 진군하시겠습니까?",
                    ctx.lords[ctx.castles[current_castle].owner as usize].first_name,
                );
                for j in 0..ctx.castles[current_castle].connected_castles.len() {
                    println!(
                        "{} {}",
                        *ctx.castles[current_castle]
                            .connected_castles
                            .get(j)
                            .unwrap() as usize,
                        ctx.castles[*ctx.castles[current_castle]
                            .connected_castles
                            .get(j)
                            .unwrap() as usize]
                            .name,
                    );
                }

                println!("");

                let target_castle: usize = input_number();

                ctx.pause_a_key();

                let mut is_connected = false;

                for castle in ctx.castles[current_castle].connected_castles.iter() {
                    if *castle as usize == target_castle {
                        is_connected = true;

                        break;
                    }
                }

                if !is_connected {
                    println!("진군을 취소했습니다.");

                    ctx.pause_a_key();

                    continue;
                }

                let mut troop_max = ctx.castles[current_castle].troop_count;

                if ctx.castles[target_castle].owner == ctx.player_lord {
                    let target_capacity = TROOP_MAX - ctx.castles[target_castle].troop_count;

                    troop_max = std::cmp::min(troop_max, target_capacity);
                } else {
                }
                println!(
                    "{}에 몇 명 진군하시겠습니까?(0-{})",
                    ctx.castles[current_castle].name, troop_max
                );

                let mut troop_count;

                loop {
                    troop_count = input_number();
                    if troop_count <= troop_max {
                        break;
                    }
                }

                ctx.castles[current_castle].troop_count -= troop_count;

                if ctx.castles[target_castle].owner == ctx.player_lord {
                    ctx.castles[target_castle].troop_count += troop_count;
                }
                println!();

                println!(
                    "{}에 {}명{}",
                    ctx.castles[target_castle].name,
                    troop_count * TROOP_UNIT,
                    if ctx.castles[target_castle].owner == ctx.player_lord {
                        " 이동했습니다."
                    } else {
                        "으로 출진이다~!!"
                    }
                );

                if ctx.castles[target_castle].owner != ctx.player_lord {
                    ctx.pause_a_key();

                    ctx.siege(ctx.player_lord, troop_count, target_castle);
                }
            } else {
                let mut connected_enemy_castles = Vec::new();

                for j in 0..ctx.castles[current_castle].connected_castles.len() {
                    if ctx.castles[ctx.castles[current_castle].connected_castles[j] as usize].owner
                        != ctx.castles[current_castle].owner
                    {
                        connected_enemy_castles
                            .push(ctx.castles[current_castle].connected_castles[j]);
                    }
                }

                if connected_enemy_castles.len() > 0 {
                    connected_enemy_castles.sort_by(|a, b| {
                        ctx.castles[*a as usize]
                            .troop_count
                            .cmp(&ctx.castles[*b as usize].troop_count)
                    });

                    while connected_enemy_castles.len() > 1
                        && ctx.castles[connected_enemy_castles[0] as usize].troop_count
                            < ctx.castles[connected_enemy_castles[connected_enemy_castles.len() - 1]
                                as usize]
                                .troop_count
                    {
                        connected_enemy_castles.pop().unwrap();
                    }

                    let target_castle =
                        *connected_enemy_castles.choose(&mut ctx.rng).unwrap() as usize;
                    if ctx.castles[current_castle].troop_count >= TROOP_BASE
                        || (ctx.castles[current_castle].troop_count - 1
                            >= ctx.castles[target_castle].troop_count * 2)
                    {
                        let troop_count =
                            std::cmp::max(ctx.castles[current_castle].troop_count - 1, 0);

                        ctx.castles[current_castle].troop_count -= troop_count;

                        println!(
                            "{}의 {}{}이(가) {}에 공격해 들어왔습니다!",
                            ctx.castles[current_castle].name,
                            ctx.lords[ctx.castles[current_castle].owner as usize].family_name,
                            ctx.lords[ctx.castles[current_castle].owner as usize].first_name,
                            ctx.castles[target_castle].name,
                        );

                        ctx.siege(
                            ctx.castles[current_castle].owner,
                            troop_count,
                            target_castle,
                        );
                    }
                } else {
                    let mut front_castles = Vec::new();

                    for neighbor in &ctx.castles[current_castle].connected_castles {
                        for neighbor_neighbor in &ctx.castles[*neighbor as usize].connected_castles
                        {
                            if ctx.castles[*neighbor_neighbor as usize].owner
                                != ctx.castles[*neighbor as usize].owner
                            {
                                front_castles.push(*neighbor);
                                break;
                            }
                        }
                    }

                    let mut dest_castles = if front_castles.is_empty() {
                        ctx.castles[current_castle].connected_castles.clone()
                    } else {
                        front_castles.clone()
                    };

                    dest_castles.sort_by(|a, b| {
                        ctx.castles[*a as usize]
                            .troop_count
                            .cmp(&ctx.castles[*b as usize].troop_count)
                    });

                    while dest_castles.len() > 1
                        && ctx.castles[dest_castles[0] as usize].troop_count
                            < ctx.castles[dest_castles[dest_castles.len() - 1] as usize].troop_count
                    {
                        dest_castles.pop().unwrap();
                    }

                    let target_castle = *dest_castles.choose(&mut ctx.rng).unwrap() as usize;

                    let mut send_troop_count = TROOP_MAX - ctx.castles[target_castle].troop_count;

                    if !front_castles.is_empty() {
                        send_troop_count = std::cmp::min(
                            send_troop_count,
                            ctx.castles[current_castle].troop_count,
                        );
                    } else {
                        if ctx.castles[current_castle].troop_count + 1 >= TROOP_BASE {
                            send_troop_count = std::cmp::min(
                                send_troop_count,
                                ctx.castles[current_castle].troop_count + 1 - TROOP_BASE,
                            );
                        }
                    }

                    if send_troop_count > 0 {
                        ctx.castles[current_castle].troop_count -= send_troop_count;

                        ctx.castles[target_castle].troop_count += send_troop_count;

                        println!(
                            "{}에서 {}로 {}명 이동했습니다!",
                            ctx.castles[current_castle].name,
                            ctx.castles[target_castle].name,
                            send_troop_count * TROOP_UNIT,
                        );
                    }
                }
            }
            ctx.pause_a_key();
        }

        if ctx.get_castle_count(ctx.player_lord) <= 0 {
            ctx.draw_screen();

            println!("{}", ctx.chronology);

            println!("");

            println!("GAME OVER");

            ctx.pause_a_key();
        }
        ctx.year += 1;

        for i in 0..CastleEnum::Max as usize {
            if ctx.castles[i].troop_count < TROOP_BASE {
                ctx.castles[i].troop_count += 1;
            } else if ctx.castles[i].troop_count > TROOP_BASE {
                ctx.castles[i].troop_count -= 1;
            }
        }
    }
}
