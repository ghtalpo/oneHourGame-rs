use std::convert::TryFrom;
use std::convert::TryInto;

use clearscreen::clear;
use getch_rs::Getch;
use getch_rs::Key;
// use rand::random_range;
use rand::Rng;
use rand::rngs::ThreadRng;

// [2]상수를 정의하는 곳
const SPELL_COST: i64 = 3;
const MAP_WIDTH: usize = 16;
const MAP_HEIGHT: usize = 16;
const SCREEN_WIDTH: usize = 16;
const SCREEN_HEIGHT: usize = 12;

// [3-1]몬스터의 종류를 정의한다
#[derive(Copy, Clone)]
enum MonsterEnum {
    Player = 0,
    Slime = 1,
    Boss = 2,
    Max,
}

// [3-2]캐릭터의 종류를 정의한다
#[derive(Copy, Clone)]
enum CharacterEnum {
    Player = 0,
    Monster = 1,
    Max,
}

// [3-3]명령의 종류를 정의한다
#[derive(Debug, Copy, Clone)]
enum CommandEnum {
    Fight = 0,
    Spell = 1,
    Run = 2,
    Max,
}

impl TryFrom<usize> for CommandEnum {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == CommandEnum::Fight as usize => Ok(CommandEnum::Fight),
            x if x == CommandEnum::Spell as usize => Ok(CommandEnum::Spell),
            x if x == CommandEnum::Run as usize => Ok(CommandEnum::Run),
            _ => Err(()),
        }
    }
}

impl CommandEnum {
    pub fn increase(&mut self) {
        *self = ((*self as usize + 1) % Self::Max as usize)
            .try_into()
            .unwrap();
    }
    pub fn decrease(&mut self) {
        *self = ((*self as usize + Self::Max as usize - 1) % Self::Max as usize)
            .try_into()
            .unwrap();
    }
}

#[derive(Copy, Clone)]
enum MapEnum {
    Field = 0,
    KingCastle,
    Max,
}

// [4-1]캐릭터의 구조체를 선언한다

#[derive(Clone)]
struct Character {
    hp: i64,
    max_hp: i64,
    mp: i64,
    max_mp: i64,
    attack: i64,
    name: String,
    aa: String, // ascii art
    command: CommandEnum,
    target: CharacterEnum,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            hp: 0,
            max_hp: 0,
            mp: 0,
            max_mp: 0,
            attack: 0,
            name: String::new(),
            aa: String::new(),
            command: CommandEnum::Fight,
            target: CharacterEnum::Max,
        }
    }
}

struct Context {
    monsters: [Character; MonsterEnum::Max as usize],
    characters: [Character; CharacterEnum::Max as usize],
    map: [u8; MapEnum::Max as usize * MAP_HEIGHT * MAP_WIDTH],
    current_map: MapEnum,
    player_x: usize,
    player_y: usize,
    rng: ThreadRng,
    g: Getch,
}

impl Context {
    pub fn new() -> Self {
        let mut map = [0_u8; MapEnum::Max as usize * MAP_HEIGHT * MAP_WIDTH];

        for (i, v) in vec![
            "~~~~~~~~~~~~~~~~",
            "~~MMMMM~~MMMM.~~",
            "~M...M.##..M...~",
            "~M.M.M.~~M.M.M.~",
            "~M.M...~~M...M.~",
            "~M.MMMM~~MMMM..~",
            "~M..MM.~~~~~~#~~",
            "~~M.M.~~~~~~~#~~",
            "~~M.MM~~~~BMM..~",
            "~~...MM~~M.MMM.~",
            "~...~~M~~M...M.~",
            "~..~~~K~~MMM.M.~",
            "~..~~~.~~M...M.~",
            "~......~~M.MM..~",
            "~~....~~~~....~~",
            "~~~~~~~~~~~~~~~~",
            "HHH.......HHH\0\0\0",
            "H.H.......H.H\0\0\0",
            "HHHHHHHHHHHHH\0\0\0",
            ".H.........H.\0\0\0",
            ".H.HHH.HHH.H.\0\0\0",
            ".H.H0H.H1H.H.\0\0\0",
            ".H.........H.\0\0\0",
            ".HW.......WH.\0\0\0",
            ".HY.......YH.\0\0\0",
            "HHHHHH.HHHHHH\0\0\0",
            "H.H~~~#~~~H.H\0\0\0",
            "HHH~~~#~~~HHH\0\0\0",
            "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ]
        .join("")
        .to_string()
        .bytes()
        .enumerate()
        {
            map[i] = v;
        }

        Self {
            monsters: [
                // [5-1-1]MONSTER_PLAYER    플레이어
                Character {
                    hp: 100,
                    max_hp: 100,
                    mp: 15,
                    max_mp: 15,
                    attack: 30,
                    name: "용사".to_string(),
                    aa: String::new(),
                    command: CommandEnum::Fight,
                    ..Character::default()
                },
                // [5-1-8]MONSTER_SLIME 슬라임
                Character {
                    hp: 3,
                    max_hp: 3,
                    mp: 0,
                    max_mp: 0,
                    attack: 2,
                    name: "슬라임".to_string(),
                    aa: "/·Д·\\".to_string(),
                    command: CommandEnum::Fight,
                    ..Character::default()
                },
                // [5-1-16]MONSTER_BOSS 마왕
                Character {
                    hp: 255,
                    max_hp: 255,
                    mp: 0,
                    max_mp: 0,
                    attack: 50,
                    name: "마왕".to_string(),
                    aa: "  A@A  \nφ(▼皿▼)φ".to_string(),
                    command: CommandEnum::Fight,
                    ..Character::default()
                },
            ],
            characters: [Character::default(), Character::default()],
            map,
            current_map: MapEnum::KingCastle,
            player_x: 4,
            player_y: 6,
            rng: rand::rng(),
            g: Getch::new(),
        }
    }

    // [6-1]게임을 초기화하는 함수를 선언한다
    pub fn init(&mut self) {
        // [6-1-1]플레이어의 상태를 초기화한다
        self.characters[CharacterEnum::Player as usize] =
            self.monsters[MonsterEnum::Player as usize].clone();
    }

    // [6-2]전투 장면의 화면을 그리는 함수를 선언한다
    fn draw_battle_screen(&self) {
        // [6-2-1]화면을 클리어한다
        clearscreen::clear().unwrap();

        // [6-2-2]플레이어의 이름을 표시한다
        println!("{}", self.characters[CharacterEnum::Player as usize].name);

        // [6-2-3]플레이어의 상태를 표시한다
        println!(
            "HP:{}/{} MP:{}/{}",
            self.characters[CharacterEnum::Player as usize].hp,
            self.characters[CharacterEnum::Player as usize].max_hp,
            self.characters[CharacterEnum::Player as usize].mp,
            self.characters[CharacterEnum::Player as usize].max_mp
        );

        println!();

        // [6-2-5]몬스터의 아스키아트를 그린다
        print!("{}", self.characters[CharacterEnum::Monster as usize].aa);

        // [6-2-6]몬스터의 HP를 표시한다
        println!(
            "(HP:{}/{})",
            self.characters[CharacterEnum::Monster as usize].hp,
            self.characters[CharacterEnum::Monster as usize].max_hp,
        );

        println!();
    }

    // [6-3]명령을 선택하는 함수를 선언한다
    fn select_command(&mut self) {
        let command_names = ["싸운다", "주문", "도망친다"];

        // [6-3-1]플레이어의 명령을 초기화한다
        self.characters[CharacterEnum::Player as usize].command = CommandEnum::Fight;

        loop {
            self.draw_battle_screen();

            // [6-3-4]명령 목록을 표시한다
            for (i, command_name) in command_names.iter().enumerate() {
                if i == self.characters[CharacterEnum::Player as usize].command as usize {
                    print!(">");
                } else {
                    print!(" ");
                }
                println!("{}", command_name);
            }

            // [6-3-10]입력된 키에 따라 분기한다
            match self.g.getch() {
                Ok(Key::Char('w')) => {
                    self.characters[CharacterEnum::Player as usize]
                        .command
                        .decrease();
                }
                Ok(Key::Char('s')) => {
                    self.characters[CharacterEnum::Player as usize]
                        .command
                        .increase();
                }
                _ => {
                    return;
                }
            }
        }
    }

    // [6-4]전투 장면의 함수를 선언한다
    fn battle(&mut self, monster: MonsterEnum) {
        // [6-4-1]몬스터의 상태를 초기화한다
        self.characters[CharacterEnum::Monster as usize] = self.monsters[monster as usize].clone();

        // [6-4-2]플레이어의 공격 대상을 몬스터로 설정한다
        self.characters[CharacterEnum::Player as usize].target = CharacterEnum::Monster;

        // [6-4-3]몬스터의 공격 대상을 플레이어로 설정한다
        self.characters[CharacterEnum::Monster as usize].target = CharacterEnum::Player;

        // [6-4-4]전투 장면의 화면을 그리는 함수를 호출한다
        self.draw_battle_screen();

        // [6-4-5]전투 장면의 첫 메시지를 표시한다
        println!(
            "{}이(가) 나타났다!",
            self.characters[CharacterEnum::Monster as usize].name
        );

        let _ = self.g.getch();

        // [6-4-7]전투가 끝날 때까지 루프한다
        loop {
            self.select_command();

            // [6-4-9]각 캐릭터를 반복한다
            for i in 0..CharacterEnum::Max as usize {
                self.draw_battle_screen();

                let target = self.characters[i].target;

                // [6-4-11]선택된 명령에 따라 분기한다
                match self.characters[i].command {
                    CommandEnum::Fight => {
                        println!("{}의 공격", self.characters[i].name);

                        let _ = self.g.getch();

                        // [6-4-15]적에게 주는 대미지를 계산한다
                        let attack = self.characters[i].attack;

                        // [6-4-16]적에게 대미지를 준다
                        let damage = 1 + self.rng.random_range(0..attack);
                        self.characters[target as usize].hp -= damage;

                        // [6-4-17]적의 HP가 음의 값이 되었는지를 판정한다
                        if self.characters[target as usize].hp < 0 {
                            self.characters[target as usize].hp = 0;
                        }

                        self.draw_battle_screen();

                        // [6-4-20]적에게 대미지를 준 메시지를 표시한다
                        println!(
                            "{}에게 {}의 데미지!",
                            self.characters[target as usize].name, damage
                        );

                        let _ = self.g.getch();
                    }
                    CommandEnum::Spell => {
                        // [6-4-23]MP가 충분한지 여부를 판정한다
                        if self.characters[i].mp < SPELL_COST {
                            println!("MP가 부족하다.");

                            let _ = self.g.getch();
                            continue;
                        }

                        // [6-4-27]MP를 소비시킨다
                        self.characters[i].mp -= SPELL_COST;

                        self.draw_battle_screen();

                        // [6-4-29]주문을 외운 메시지를 표시한다
                        println!("{}은(는) 주문을 외웠다.", self.characters[i].name);

                        let _ = self.g.getch();

                        // [6-4-31]HP를 회복시킨다
                        self.characters[i].hp = self.characters[i].max_hp;

                        self.draw_battle_screen();

                        // [6-4-33]HP가 회복된 메시지를 표시한다
                        println!("{}의 상처가 회복되었다.", self.characters[i].name);

                        let _ = self.g.getch();
                    }
                    CommandEnum::Run => {
                        // [6-4-36]도망친 메시지를 표시한다
                        println!("{}은(는) 도망쳤다.", self.characters[i].name);

                        let _ = self.g.getch();
                        return;
                    }
                    CommandEnum::Max => {}
                }
                // [6-4-39]공격 대상을 쓰러뜨렸는지 여부를 판정한다
                if self.characters[target as usize].hp <= 0 {
                    // [6-4-40]공격 대상에 따라 처리를 분기시킨다
                    match target {
                        CharacterEnum::Player => {
                            // [6-4-42]플레이어가 사망한 메시지를 표시한다
                            println!("당신은 사망했습니다.",);
                        }
                        CharacterEnum::Monster => {
                            // [6-4-44]몬스터의 아스키아트를 아무것도 표시하지 않게 다시 작성한다
                            self.characters[target as usize].aa.clear();

                            self.draw_battle_screen();

                            // [6-4-46]몬스터를 쓰러뜨린 메시지를 표시한다
                            println!(
                                "{}을(를) 쓰러뜨렸다!",
                                self.characters[target as usize].name
                            );
                        }
                        _ => {}
                    }
                    let _ = self.g.getch();
                    return;
                }
            }
        }
    }

    fn get_cell_xy(&self, map: MapEnum, x: usize, y: usize) -> u8 {
        self.map[map as usize * (MAP_HEIGHT * MAP_WIDTH) + y * MAP_WIDTH + x]
    }

    fn draw_map(&self) {
        clearscreen::clear().unwrap();

        let y0 = self.player_y as isize - SCREEN_HEIGHT as isize / 2;
        let y1 = self.player_y as isize + SCREEN_HEIGHT as isize / 2;
        let x0 = self.player_x as isize - SCREEN_WIDTH as isize / 2;
        let x1 = self.player_x as isize + SCREEN_WIDTH as isize / 2;
        for y in y0..y1 {
            for x in x0..x1 {
                if x == self.player_x as isize && y == self.player_y as isize {
                    print!("勇");
                } else if x < 0
                    || x >= MAP_WIDTH as isize
                    || y < 0
                    || y >= MAP_HEIGHT as isize
                    || self.get_cell_xy(self.current_map, x as usize, y as usize) == 0
                {
                    match self.current_map {
                        MapEnum::Field => print!("~~"),
                        MapEnum::KingCastle => print!(". "),
                        _ => {}
                    }
                } else {
                    match self.get_cell_xy(self.current_map, x as usize, y as usize) as char {
                        '~' => print!("~~"),
                        '.' => print!(". "),
                        'M' => print!("MM"),
                        '#' => print!("##"),
                        'K' => print!("王"),
                        'B' => print!("魔"),
                        'H' => print!("□□"),
                        'W' => print!("炎"),
                        'Y' => print!("Y "),
                        '0' => print!("王"),
                        '1' => print!("姫"),
                        _ => {}
                    }
                }
            }
            println!();
        }
        println!();
    }

    fn pause_a_key(&self) {
        if let Ok(Key::Esc) = self.g.getch() {
            std::process::exit(0);
        }
    }
}

fn main() {
    let mut ctx = Context::new();

    // [6-6-2]게임을 초기화하는 함수를 호출한다
    ctx.init();

    // [6-6-3]전투 장면의 함수를 호출한다
    // ctx.battle(MonsterEnum::Boss);
    loop {
        ctx.draw_map();

        let last_player_x = ctx.player_x;
        let last_player_y = ctx.player_y;

        match ctx.g.getch() {
            Ok(Key::Char('w')) => ctx.player_y -= 1,
            Ok(Key::Char('s')) => ctx.player_y += 1,
            Ok(Key::Char('a')) => ctx.player_x -= 1,
            Ok(Key::Char('d')) => ctx.player_x += 1,
            Ok(Key::Esc) => std::process::exit(0),
            _ => {}
        }

        if ctx.player_x < 0
            || ctx.player_x >= MAP_WIDTH
            || ctx.player_y < 0
            || ctx.player_y >= MAP_HEIGHT
            || ctx.get_cell_xy(
                ctx.current_map,
                ctx.player_x as usize,
                ctx.player_y as usize,
            ) == 0
        {
            match ctx.current_map {
                MapEnum::KingCastle => {
                    ctx.current_map = MapEnum::Field;

                    ctx.player_x = 6;
                    ctx.player_y = 12;
                }
                _ => {}
            }
        }

        match ctx.get_cell_xy(ctx.current_map, ctx.player_x, ctx.player_y) as char {
            '.' | '#' => {}
            'K' => {
                ctx.current_map = MapEnum::KingCastle;

                ctx.player_x = 6;
                ctx.player_y = 12;
            }
            '0' => {
                println!(
                    "*[용사여!\n\
                    동쪽 마성의 마왕을\n\
                    물리치고 세상을 구해주세요!"
                );

                ctx.pause_a_key();
            }
            '1' => {
                println!(
                    "*[신께 기도드립니다.!\n\
                    오, 신이시여!\n\
                    용사님에게 축복을!"
                );

                ctx.pause_a_key();
            }
            _ => {
                ctx.player_x = last_player_x;
                ctx.player_y = last_player_y;
            }
        }
    }
}
