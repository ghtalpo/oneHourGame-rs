#[derive(Debug, Clone)]
struct Character {
    hp: isize,
    max_hp: isize,
    mp: isize,
    max_mp: isize,
    name: String,
    aa: String, // ascii art
    command: CommandEnum,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            hp: 0,
            max_hp: 0,
            mp: 0,
            max_mp: 0,
            name: String::new(),
            aa: String::new(),
            command: CommandEnum::Fight,
        }
    }
}

#[derive(Copy, Clone)]
enum MonsterEnum {
    Player = 0,
    Slime = 1,
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
                    name: "용사".to_string(),
                    aa: String::new(),
                    command: CommandEnum::Fight,
                },
                Character {
                    hp: 3,
                    max_hp: 3,
                    mp: 0,
                    max_mp: 0,
                    name: "슬라임".to_string(),
                    aa: "/·Д·\\\n".to_string(),
                    command: CommandEnum::Fight,
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
    draw_battle_screen(ctx);
    println!("{}이(가) 나타났다!", ctx.characters[monster as usize].name);
    loop {
        select_command(ctx);
        for i in 0..CharacterEnum::Max as usize {
            draw_battle_screen(ctx);
            match ctx.characters[i].command {
                CommandEnum::Fight => {
                    println!("{}의 공격", ctx.characters[i].name);
                    let mut line = String::new();
                    let _ = std::io::stdin().read_line(&mut line).unwrap();
                    break;
                }
                CommandEnum::Spell => {
                    break;
                }
                CommandEnum::Run => {
                    break;
                }
                CommandEnum::Max => {
                    break;
                }
            }
        }
    }
}

fn select_command(ctx: &Context) {
    let command_names = ["싸운다", "주문", "도망친다"];
    loop {
        draw_battle_screen(ctx);

        for command_name in command_names {
            println!("{}", command_name);
        }
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line).unwrap();
    }
}

fn main() {
    let mut ctx = Context::new();
    init(&mut ctx);
    battle(&mut ctx, MonsterEnum::Slime);
}
