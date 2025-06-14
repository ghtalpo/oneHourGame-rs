use std::io;

use getch_rs::{Getch, Key};
use rand::{Rng, rngs::ThreadRng, seq::IndexedRandom};

// [2]상수를 정의하는 곳

const TROOP_BASE: usize = 5; // [2-1]기본 병력 수를 정의한다
const TROOP_MAX: usize = 9; // [2-2]최대 병력 수를 정의한다
const TROOP_UNIT: usize = 1000; // [2-3]병력 수 단위를 정의한다
const START_YEAR: u16 = 1570; // [2-4]시작 연도를 정의한다
// const CHRONOLOGY_MAX: usize = 1024;

// [3]열거 상수를 정의하는 곳

// [3-1]다이묘의 종류를 정의한다
#[derive(Clone, Copy, PartialEq)]
enum LordEnum {
    Igak = 0,// [3-1- 1] 이각 
    Yubi,    // [3-1- 2] 유비
    Wonso,    // [3-1- 3] 원소
    Jojo,      // [3-1- 4] 조조
    Yeopo,  // [3-1- 5] 여포
    Yupyo,       // [3-1- 6] 유표
    Sonchaek,  // [3-1- 7] 손책
    Yujang,      // [3-1- 8] 유장
    Madeung, // [3-1- 9] 마등
    Gongsonchan,    // [3-1-10] 공손찬
    Max,       // [3-1-11]종류의 개수
}

impl TryFrom<usize> for LordEnum {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == LordEnum::Igak as usize => Ok(LordEnum::Igak),
            x if x == LordEnum::Yubi as usize => Ok(LordEnum::Yubi),
            x if x == LordEnum::Wonso as usize => Ok(LordEnum::Wonso),
            x if x == LordEnum::Jojo as usize => Ok(LordEnum::Jojo),
            x if x == LordEnum::Yeopo as usize => Ok(LordEnum::Yeopo),
            x if x == LordEnum::Yupyo as usize => Ok(LordEnum::Yupyo),
            x if x == LordEnum::Sonchaek as usize => Ok(LordEnum::Sonchaek),
            x if x == LordEnum::Yujang as usize => Ok(LordEnum::Yujang),
            x if x == LordEnum::Madeung as usize => Ok(LordEnum::Madeung),
            x if x == LordEnum::Gongsonchan as usize => Ok(LordEnum::Gongsonchan),
            _ => Err(()),
        }
    }
}

// [3-2]성의 종류를 정의한다
#[derive(Clone, Copy, Debug)]
enum CastleEnum {
    Yonezawa = 0,    // [3-2- 1]요네자와성
    Kasugayama,      // [3-2- 2]가스가야마성
    Tsutsujigasaki,  // [3-2- 3]쓰쓰지가사키관
    Odawara,         // [3-2- 4]오다와라성
    Okazaki,         // [3-2- 5]오카자키성
    Gifu,            // [3-2- 6]기후성
    Nijo,            // [3-2- 7]니조성
    Yoshidakoriyama, // [3-2- 8]요시다고리야마성
    Oko,             // [3-2- 9]오코성
    Uchi,            // [3-2-10]우찌성
    Max,             // [3-2-11]종류의 개수
}

// [4]구조체를 선언하는 곳

// [4-1]다이묘 구조체를 선언한다
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

// [4-2]성 구조체를 선언한다
struct Castle {
    name: String,
    owner: LordEnum,
    troop_count: usize,
    connected_castles: Vec<CastleEnum>, // [4-2-4]연결된 성 리스트
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

// 유틸리티 함수
fn get_first_n_chars(s: &str, n: usize) -> String {
    let mut chars = s.char_indices();
    match chars.nth(n) {
        Some(index) => s[..index.0].to_string(),
        None => s.to_string(),
    }
}

fn input_number() -> usize {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { continue }

        let num = match input.trim().parse::<usize>() {
            Ok(num) => num,
            _ => continue,
        };

        println!("{}", num);
        return num;
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
            // [5-1]다이묘 배열을 선언한다
            lords: [
                Lord::new("이각", "치연"),
                Lord::new("유비", "현덕"),
                Lord::new("원소", "본초"),
                Lord::new("조조", "맹덕"),
                Lord::new("여포", "봉선"),
                Lord::new("유표", "경승"),
                Lord::new("손책", "백부"),
                Lord::new("유장", "계옥"),
                Lord::new("마등", "수성"),
                Lord::new("공손찬", "백규"),
            ],
            // [5-2]성 배열을 선언한다
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
            year: 0,                    // [5-3]현재 연도를 선언한다
            player_lord: LordEnum::Max, // [5-4]플레이어의 다이묘를 선언한다
            chronology: String::new(),  // [5-5]연표를 선언한다
            rng: rand::rng(),
            g: Getch::new(),
        }
    }

    // [6-1]성의 개수를 세는 함수를 선언한다
    fn get_castle_count(&self, lord: LordEnum) -> usize {
        let mut castle_count = 0;

        for i in 0..CastleEnum::Max as usize {
            if self.castles[i].owner == lord {
                castle_count += 1;
            }
        }
        castle_count
    }

    // [6-2]기본 정보를 그리는 함수를 선언한다
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

    // [6-3]게임을 초기화하는 함수를 선언한다
    pub fn init(&mut self) {
        self.year = START_YEAR;

        for i in 0..CastleEnum::Max as usize {
            // [6-3-3]성주를 초기화한다
            self.castles[i].owner = i.try_into().unwrap();

            // [6-3-4]병력 수를 초기화한다
            self.castles[i].troop_count = TROOP_BASE;
        }

        // [6-3-5]연표를 클리어한다
        self.chronology.clear();

        self.draw_screen();

        // [6-3-7]다이묘 선택을 촉구하는 메시지를 표시한다
        println!(
            "주군님, 우리 성은\n\
            이 지도의 어디에 있습니까? (0~{})\n",
            CastleEnum::Max as usize - 1,
        );

        // [6-3-9]선택된 성을 보유하는 변수를 선언한다
        let mut selected_castle: usize;

        // [6-3-10]범위 내의 성 번호가 입력될 때까지 반복한다
        loop {
            selected_castle = input_number();

            if selected_castle < CastleEnum::Max as usize {
                break;
            }
        }

        // [6-3-12]선택한 성의 성주를 플레이어 다이묘로 한다
        self.player_lord = self.castles[selected_castle].owner;

        // [6-3-13]결정한 다이묘를 통지하는 메시지를 표시한다
        println!(
            "{}님, {}에서 천하 통일을\n\
            목표로 합시다!",
            self.lords[self.player_lord as usize].first_name,
            self.castles[self.player_lord as usize].name
        );

        self.pause_a_key();
    }

    // [6-4]공성전의 함수를 선언한다
    fn siege(&mut self, offensive_lord: LordEnum, mut offensive_troop_count: usize, castle: usize) {
        clearscreen::clear().unwrap();

        // [6-4-2]공성전의 이름을 표시한다
        println!("~{} 전투~\n", self.castles[castle].name);

        // [6-4-4]공격당한 다이묘를 선언한다
        let defensive_lord = self.castles[castle].owner;

        loop {
            // [6-4-6]전투의 경과를 표시한다
            println!(
                "{}군({:4}명) X {}군({:4}명)\n",
                self.lords[offensive_lord as usize].family_name,
                offensive_troop_count * TROOP_UNIT,
                self.lords[defensive_lord as usize].family_name,
                self.castles[castle].troop_count * TROOP_UNIT,
            );

            self.pause_a_key();

            // [6-4-8]공격과 수비 중 어느 한쪽의 병력 수가 0이하인지 여부를 판정한다
            if offensive_troop_count == 0 || self.castles[castle].troop_count == 0 {
                break;
            }

            // [6-4-10]0~2의 난수가 0인지 여부를 판정한다
            if self.rng.random_range(0..3) == 0 {
                self.castles[castle].troop_count -= 1;
            } else {
                offensive_troop_count -= 1;
            }
        }

        println!();

        // [6-4-15]수비 측의 병력이 전멸했는지 여부를 판정한다
        if self.castles[castle].troop_count == 0 {
            // [6-4-16]성이 함락되었다는 메시지를 표시한다
            println!("{} 함락!!\n", self.castles[castle].name,);

            // [6-4-18]공격 측의 다이묘 성으로 한다
            self.castles[castle].owner = offensive_lord;

            // [6-4-19]공격 측의 병력을 입성시킨다
            self.castles[castle].troop_count = offensive_troop_count;

            // [6-4-20]성주가 공격한 다이묘로 바뀐 메시지를 표시한다
            println!(
                "{}은(는) {} 가문의 것이 됩니다.\n",
                self.castles[castle].name, self.lords[offensive_lord as usize].family_name
            );

            // [6-4-22]수비 측의 다이묘가 성을 모두 잃었는지 여부를 판정한
            if self.get_castle_count(defensive_lord) == 0 {
                // [6-4-25]연표에 문자열을 추가한다
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
            // [6-4-26]수비 측의 병력이 전멸하지 않았다면
            // [6-4-27]공격 측이 전멸한 메시지를 표시한다
            println!(
                "{}군 전멸!!\n\
                \n\
                {}군이 {}을(를) 지켜냈습니다!",
                self.lords[offensive_lord as usize].family_name,
                self.lords[defensive_lord as usize].family_name,
                self.castles[castle].name,
            );
        }
    }

    fn pause_a_key(&self) {
        if let Ok(Key::Esc) = self.g.getch() {
            std::process::exit(0);
        }
    }
}

fn main() {
    let mut ctx = Context::new();

    // [6-5-2]게임 시작 라벨
    'start: loop {
        ctx.init(); // [6-5-4]게임을 리셋하는 함수를 호출한다

        // [6-5-5]메인 루프
        loop {
            let mut turn_order = [0_usize; CastleEnum::Max as usize]; // [6-5-6]턴 순서의 테이블을 선언한다

            // [6-5-7]턴 순서를 초기화한다
            // for i in 0..CastleEnum::Max as usize {
            for (i, turn) in turn_order.iter_mut().enumerate() {
                    *turn = i;
            }

            for i in 0..CastleEnum::Max as usize {
                // [6-5-10]턴을 랜덤으로 바꾼다
                turn_order.swap(i, ctx.rng.random_range(0..CastleEnum::Max as usize))
            }

            // [6-5-11]모든 턴을 반복한다
            for i in 0..CastleEnum::Max as usize {
                ctx.draw_screen();

                for (j, castle) in turn_order.iter().enumerate() {
                    // [6-5-14]현재 턴의 성에 커서를 그린다
                    print!("{}", if j == i { ">" } else { " " },);

                    // [6-5-15]각 턴의 성 이름을 그린다
                    print!(
                        "{:2}",
                        get_first_n_chars(&ctx.castles[*castle].name, 2),
                    );
                }

                println!("\n");

                // [6-5-17]현재 턴의 성 번호를 선언한다
                let current_castle = turn_order[i];

                println!(
                    "{} 가문의 {} 전략회의...\n",
                    ctx.lords[ctx.castles[current_castle].owner as usize].family_name,
                    ctx.castles[current_castle].name,
                );

                // [6-5-20]현재 성의 성주가 플레이어인지 여부를 판정한다
                if ctx.castles[current_castle].owner == ctx.player_lord {
                    // [6-5-21]진군할 성의 지정을 촉구하는 메시지를 표시한다
                    println!(
                        "{}님, 어디로 진군하시겠습니까?]\n",
                        ctx.lords[ctx.castles[current_castle].owner as usize].first_name,
                    );

                    // [6-5-23]모든 연결된 성을 반복한다
                    for j in 0..ctx.castles[current_castle].connected_castles.len() {
                        // [6-5-24]연결된 성 번호와 이름을 표시한다
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

                    println!();

                    // [6-5-26]진군 목표의 성을 입력하여 선언한다
                    let target_castle: usize = input_number();

                    // [6-5-27]현재 성과 대상 성이 연결되어 있는지 여부를 보유하는 플래그를 선언한다
                    let mut is_connected = false;

                    for castle in ctx.castles[current_castle].connected_castles.iter() {
                        // [6-5-29]대상 성과 연결이 확인되면
                        if *castle as usize == target_castle {
                            is_connected = true;

                            break;
                        }
                    }

                    // [6-5-32]연결된 성이 선택되지 않으면
                    if !is_connected {
                        println!("진군을 취소했습니다.");

                        ctx.pause_a_key();

                        continue; // [6-5-35]다음 국가의 전략 회의로 스킵한다
                    }

                    // [6-5-36]현재 성의 병력 수를 최대 진군 수로 선언한다
                    let mut troop_max = ctx.castles[current_castle].troop_count;

                    // [6-5-37]진군 목적지가 플레이어의 성인지 여부를 판정한다
                    if ctx.castles[target_castle].owner == ctx.player_lord {
                        // [6-5-38]진군 목적지 성의 빈 병력 수를 선언한다
                        let target_capacity = TROOP_MAX - ctx.castles[target_castle].troop_count;

                        // [6-5-39]현재 성의 병력 수나 진군 목적지의 빈 병력 수 중에서 적은 쪽을 최대 진군 병력 수로 한다
                        troop_max = std::cmp::min(troop_max, target_capacity);
                    }

                    // [6-5-40]입력된 성을 통지하고, 이동하는 병력 수의 입력을 촉구하는 메시지를 표시한다
                    println!(
                        "{}에 몇 명 진군하시겠습니까?(0-{})",
                        ctx.castles[current_castle].name, troop_max
                    );

                    // [6-5-41]진군 병력 수를 선언한다
                    let mut troop_count;

                    // [6-5-42]범위 내의 병력 수가 입력될 때까지 반복한다
                    loop {
                        // [6-5-43]진군 병력 수를 입력한다
                        troop_count = input_number();
                        if troop_count <= troop_max {
                            break;
                        }
                    }

                    // [6-5-44]현재 성의 병력 수를 이동하는 만큼 뺀다
                    ctx.castles[current_castle].troop_count -= troop_count;

                    // [6-5-45]이동 목적지가 플레이어의 성이면
                    if ctx.castles[target_castle].owner == ctx.player_lord {
                        // [6-5-46]진군 목적지의 성 병력 수에 이동 병력 수를 더한다
                        ctx.castles[target_castle].troop_count += troop_count;
                    }

                    println!();

                    // [6-5-48]입력된 진군 병력 수를 통지한다
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

                    // [6-5-49]진군 목적지가 적의 성인지 여부를 판정한다
                    if ctx.castles[target_castle].owner != ctx.player_lord {
                        ctx.pause_a_key();

                        // [6-5-51]공성전 함수를 호출한다
                        ctx.siege(ctx.player_lord, troop_count, target_castle);
                    }
                } else {
                    // [6-5-52]현재 성의 성주가 플레이어가 아니면

                    // [6-5-53]연결된 적의 성 리스트를 선언한다
                    let mut connected_enemy_castles = Vec::new();

                    // [6-5-54]모든 연결된 성을 반복한다
                    for j in 0..ctx.castles[current_castle].connected_castles.len() {
                        // [6-5-55]적의 성인지 여부를 판정한다
                        if ctx.castles[ctx.castles[current_castle].connected_castles[j] as usize]
                            .owner
                            != ctx.castles[current_castle].owner
                        {
                            // [6-5-56]연결된 적의 성 리스트에 더한다
                            connected_enemy_castles
                                .push(ctx.castles[current_castle].connected_castles[j]);
                        }
                    }

                    // [6-5-57]연결된 적의 성이 있는지 여부를 판정한다
                    if !connected_enemy_castles.is_empty() {
                        // [6-5-58]병력이 적은 순으로 정렬한다
                        connected_enemy_castles.sort_by(|a, b| {
                            ctx.castles[*a as usize]
                                .troop_count
                                .cmp(&ctx.castles[*b as usize].troop_count)
                        });

                        // [6-5-59]가장 병력이 적은 성만 남을 때까지 반복한다
                        while
                        // 인접하는 적의 성이 2성 이상이다
                        connected_enemy_castles.len() > 1
                            // 그리고 그중에서 가장 병력 수가 적은 성보다도 병력 수가 많은 성이 있으면
                            && ctx.castles[connected_enemy_castles[0] as usize].troop_count
                                < ctx.castles[connected_enemy_castles
                                    [connected_enemy_castles.len() - 1]
                                    as usize]
                                    .troop_count
                        {
                            // [6-5-60]리스트에서 맨 끝을 삭제한다
                            connected_enemy_castles.pop().unwrap();
                        }

                        // [6-5-61]공격하는 성을 선언한다
                        let target_castle =
                            *connected_enemy_castles.choose(&mut ctx.rng).unwrap() as usize;

                        // [6-5-62]공격할지 여부를 판정한다
                        if
                        // 병력 수가 기준치 이상인가
                        ctx.castles[current_castle].troop_count >= TROOP_BASE
                            // 이쪽의 병력 수가 수비병을 제하고 상대의 2배이상이면
                            || (ctx.castles[current_castle].troop_count > ctx.castles[target_castle].troop_count * 2)
                        {
                            // [6-5-63]공격하는 병력 수를 선언한다
                            let troop_count =
                                std::cmp::max(ctx.castles[current_castle].troop_count - 1, 0);

                            // [6-5-64]현재 성의 병력 수에서 공격하는 병력 수를 뺀다
                            ctx.castles[current_castle].troop_count -= troop_count;

                            // [6-5-65]공격하는 메시지를 표시한다
                            println!(
                                "{}의 {}{}이(가) {}에 공격해 들어왔습니다!",
                                ctx.castles[current_castle].name,
                                ctx.lords[ctx.castles[current_castle].owner as usize].family_name,
                                ctx.lords[ctx.castles[current_castle].owner as usize].first_name,
                                ctx.castles[target_castle].name,
                            );

                            ctx.pause_a_key();

                            // [6-5-67]공성전 함수를 호출한다
                            ctx.siege(
                                ctx.castles[current_castle].owner,
                                troop_count,
                                target_castle,
                            );
                        }
                    } else {
                        // [6-5-68]연결된 적의 성이 없으면

                        // [6-5-69]연결된 전선의 성 리스트를 선언한다
                        let mut front_castles = Vec::new();

                        for neighbor in &ctx.castles[current_castle].connected_castles {
                            // [6-5-71]인접하는 성에 연결된 모든 성을 반복한다
                            for neighbor_neighbor in
                                &ctx.castles[*neighbor as usize].connected_castles
                            {
                                // [6-5-72]대상 성이 적의 성에 인접하고 있는지 여부를 판정한다
                                if ctx.castles[*neighbor_neighbor as usize].owner
                                    != ctx.castles[*neighbor as usize].owner
                                {
                                    front_castles.push(*neighbor); // [6-5-73]전선의 성 리스트에 추가한다
                                    break;
                                }
                            }
                        }

                        // [6-5-75]병력을 보내는 성 리스트를 선언한다
                        let mut dest_castles =
                            // 전선이 성이 없는지 여부를 판정한다
                            if front_castles.is_empty() {
                                // 없으면 연결된 성 리스트를 설정한다
                                ctx.castles[current_castle].connected_castles.clone()
                            } else {
                                // 있으면 전선의 성 리스트를 설정한다
                                front_castles.clone()
                            };

                        // [6-5-76]병력이 적은 순으로 정렬한다
                        dest_castles.sort_by(|a, b| {
                            ctx.castles[*a as usize]
                                .troop_count
                                .cmp(&ctx.castles[*b as usize].troop_count)
                        });

                        // [6-5-77]가장 병력이 적은 성만 남을 때까지 반복한다
                        while
                        // 병력을 보내는 목적지 성의 후보가 여러 개 있다
                        dest_castles.len() > 1
                            // 그리고 그 중에서 가장 병력 수가 적은 성보다도 병력 수가 많은 성이 있으면
                            && ctx.castles[dest_castles[0] as usize].troop_count
                                < ctx.castles[dest_castles[dest_castles.len() - 1] as usize]
                                    .troop_count
                        {
                            // [6-5-78]리스트에서 맨 끝을 삭제한다
                            dest_castles.pop().unwrap();
                        }

                        // [6-5-79]병력을 보내는 성을 선언한다
                        let target_castle = *dest_castles.choose(&mut ctx.rng).unwrap() as usize;

                        // [6-5-80]보내는 병력 수를 선언한다
                        let mut send_troop_count =
                            TROOP_MAX - ctx.castles[target_castle].troop_count;

                        // [6-5-81]병력을 보내는 목적지의 성이 전선인지 여부를 판정한다
                        if !front_castles.is_empty() {
                            // [6-5-82]보내는 목적지의 빈 병력 수와 보내는 곳의 병력 수 중, 적은 병력 수를 설정한다

                            send_troop_count = std::cmp::min(
                                send_troop_count,
                                ctx.castles[current_castle].troop_count,
                            );
                        } else {
                            // [6-5-83]병력을 보내는 목적지의 성이 전선이 아닌 아군의 성이면
                            if ctx.castles[current_castle].troop_count + 1 >= TROOP_BASE {
                                send_troop_count = std::cmp::min(
                                    send_troop_count,
                                    ctx.castles[current_castle].troop_count + 1 - TROOP_BASE,
                                );
                            }
                        }

                        // [6-5-85]보내는 병력이 있는지 여부를 판정한다
                        if send_troop_count > 0 {
                            // [6-5-86]보내는 곳의 병력 수를 뺀다
                            ctx.castles[current_castle].troop_count -= send_troop_count;

                            // [6-5-87]보내는 곳의 병력 수를 늘린다
                            ctx.castles[target_castle].troop_count += send_troop_count;

                            // [6-5-88]병사가 이동한 메시지를 표시한다
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

                // [6-5-90]플레이어의 성이 없는지 여부를 판정한다
                if ctx.get_castle_count(ctx.player_lord) == 0 {
                    ctx.draw_screen();

                    println!("{}", ctx.chronology);

                    println!();

                    println!("GAME OVER");

                    ctx.pause_a_key();

                    continue 'start;
                } else if ctx.get_castle_count(ctx.player_lord) >= CastleEnum::Max as usize {
                    // [6-5-97]플레이어가 모든 성을 소유하고 있는지 여부를 판정한다

                    ctx.draw_screen();

                    println!("{}", ctx.chronology);

                    // [6-5-100]엔딩 메시지를 표시한다
                    println!(
                        "{}년 {}{}이(가) 정이대장군에 임명된다\n\
                        {}년 {}{}이(가) {}막부를 연다\n\
                        \n\
                        THE END",
                        ctx.year + 3,
                        ctx.lords[ctx.player_lord as usize].family_name,
                        ctx.lords[ctx.player_lord as usize].first_name,
                        ctx.year + 3,
                        ctx.lords[ctx.player_lord as usize].family_name,
                        ctx.lords[ctx.player_lord as usize].first_name,
                        ctx.lords[ctx.player_lord as usize].family_name,
                    );

                    ctx.pause_a_key();

                    continue 'start;
                }
            }

            ctx.year += 1; // [6-5-103]연도를 진행한다

            for i in 0..CastleEnum::Max as usize {
                // [6-5-105]대상 성의 병력 수가 기본 병력 수 미만인지 여부를 판정한다
                if ctx.castles[i].troop_count < TROOP_BASE {
                    ctx.castles[i].troop_count += 1; // [6-5-106]병력 수를 늘린다
                }
                // [6-5-107]대상 성의 병력 수가 기본 병력 수보다 많은지 여부를 판정한다
                else if ctx.castles[i].troop_count > TROOP_BASE {
                    ctx.castles[i].troop_count -= 1; // [6-5-108]병력 수를 줄인다
                }
            }

            // [6-5-109]「혼노지의 변」이벤트가 발생하는 조건을 마족하고 있는지 여부를 판정한다
            if 
                // 1582년이다
                ctx.year == 1582 
                // 그리고 오다 가문이 니조성을 소유하고 있다
                && ctx.castles[CastleEnum::Nijo as usize].owner == LordEnum::Oda {
                // [6-5-110]오다 가문 다이묘의 성을「하시바」로 변경한다
                ctx.lords[LordEnum::Oda as usize].family_name = "하시바".to_string();

                // [6-5-111]오다 가문 다이묘의 이름을「히데요시」로 변경한다
                ctx.lords[LordEnum::Oda as usize].first_name = "히데요시".to_string();

                ctx.draw_screen();

                // [6-5-113]「혼노지의 변」이벤트의 메시지를 표시한다
                println!(
                    "아케치 미쓰히데 [적은 혼노지에 있다!]\n\
                    \n\
                    아케치 미쓰히데가 혼노지의 오다 노부나가를 습격했다!\n\
                    \n\
                    오다 노부나가 [할 수 없지...]\n\
                    \n\
                    오다 노부나가는 혼노지에서 자결했다!\n\
                    \n\
                    후일, 하시바 히데요시가 야마자키 전투에서 아케치 미쓰히데를 물리치고,\n\
                    오다 가문 후계의 영토를 찬탈했다!"
                );

                ctx.pause_a_key();
            }
        }
    }
}
