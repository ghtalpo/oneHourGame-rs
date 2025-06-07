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

#[derive(Copy, Clone)]
enum MonsterEnum {
    Player = 0,
    Slime = 1,
    Boss = 2,
    Max,
}

#[derive(Copy, Clone)]
enum CharacterEnum {
    Player = 0,
    Monster = 1,
    Max,
}

#[derive(Debug, Copy, Clone)]
enum CommandEnum {
    Fight = 0,
    Spell = 1,
    Run = 2,
    Max,
}

use std::convert::TryFrom;
use std::convert::TryInto;

use rand::random_range;

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

struct Context {
    monsters: [Character; MonsterEnum::Max as usize],
    characters: [Character; CharacterEnum::Max as usize],
}

impl Context {
    pub fn new() -> Self {
        Self {
            monsters: [
                Character {
                    hp: 15,
                    max_hp: 15,
                    mp: 15,
                    max_mp: 15,
                    attack: 3,
                    name: "용사".to_string(),
                    aa: String::new(),
                    command: CommandEnum::Fight,
                    ..Character::default()
                },
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
        }
    }
}

fn init(ctx: &mut Context) {
    ctx.characters[CharacterEnum::Player as usize] =
        ctx.monsters[MonsterEnum::Player as usize].clone();
}

fn draw_battle_screen(ctx: &Context) {
    clearscreen::clear().unwrap();

    println!("{}", ctx.characters[CharacterEnum::Player as usize].name);
    println!(
        "HP:{}/{} MP:{}/{} ",
        ctx.characters[CharacterEnum::Player as usize].hp,
        ctx.characters[CharacterEnum::Player as usize].max_hp,
        ctx.characters[CharacterEnum::Player as usize].mp,
        ctx.characters[CharacterEnum::Player as usize].max_mp
    );
    print!("{}", ctx.characters[CharacterEnum::Monster as usize].aa);
    println!(
        "(HP:{}/{})",
        ctx.characters[CharacterEnum::Monster as usize].hp,
        ctx.characters[CharacterEnum::Monster as usize].max_hp,
    );
    print!("\n");
}

fn battle(ctx: &mut Context, monster: MonsterEnum) {
    ctx.characters[CharacterEnum::Monster as usize] = ctx.monsters[monster as usize].clone();

    ctx.characters[CharacterEnum::Player as usize].target = CharacterEnum::Monster;
    ctx.characters[CharacterEnum::Monster as usize].target = CharacterEnum::Player;

    draw_battle_screen(ctx);
    println!(
        "{}이(가) 나타났다!",
        ctx.characters[CharacterEnum::Monster as usize].name
    );
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    loop {
        select_command(ctx);
        for i in 0..CharacterEnum::Max as usize {
            // let _ = std::io::stdin().read_line(&mut line).unwrap();
            draw_battle_screen(ctx);
            match ctx.characters[i].command {
                CommandEnum::Fight => {
                    println!("{}의 공격", ctx.characters[i].name);
                    let mut line = String::new();
                    let _ = std::io::stdin().read_line(&mut line).unwrap();
                    let attack = ctx.characters[i].attack;
                    let damage = 1 + random_range(0..attack);
                    let target = ctx.characters[i].target;
                    ctx.characters[target as usize].hp -= damage;

                    println!(
                        "{}에게 {}의 데미지!",
                        ctx.characters[target as usize].name, damage
                    );

                    if ctx.characters[target as usize].hp <= 0 {
                        // ctx.characters[target as usize].hp = 0;
                        match target {
                            CharacterEnum::Player => {
                                println!("당신은 사망했습니다.",);
                            }
                            CharacterEnum::Monster => {
                                ctx.characters[target as usize].aa.clear();
                                draw_battle_screen(ctx);
                                println!(
                                    "{}을(를) 쓰러뜨렸다!",
                                    ctx.characters[target as usize].name
                                );
                            }
                            _ => {}
                        }
                        let _ = std::io::stdin().read_line(&mut line).unwrap();
                        return;
                    }

                    if ctx.characters[target as usize].hp < 0 {
                        ctx.characters[target as usize].hp = 0;
                    }
                    // draw_battle_screen(ctx);
                    let _ = std::io::stdin().read_line(&mut line).unwrap();
                }
                CommandEnum::Spell => {}
                CommandEnum::Run => {}
                CommandEnum::Max => {}
            }
        }
    }
}

fn select_command(ctx: &mut Context) {
    let command_names = ["싸운다", "주문", "도망친다"];
    loop {
        draw_battle_screen(ctx);

        for i in 0..CommandEnum::Max as usize {
            if i == ctx.characters[CharacterEnum::Player as usize].command as usize {
                print!(">");
            } else {
                print!(" ");
            }
            println!("{}", command_names[i]);
        }
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line).unwrap();
        match line.trim() {
            "w" => {
                ctx.characters[CharacterEnum::Player as usize]
                    .command
                    .decrease();
            }
            "s" => {
                ctx.characters[CharacterEnum::Player as usize]
                    .command
                    .increase();
            }
            _ => {
                return;
            }
        }
    }
}

fn main() {
    let mut ctx = Context::new();
    init(&mut ctx);
    // battle(&mut ctx, MonsterEnum::Slime);
    battle(&mut ctx, MonsterEnum::Boss);
}
