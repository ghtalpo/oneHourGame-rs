#[derive(Debug, Default, Clone)]
struct Character {
    hp: isize,
    max_hp: isize,
    mp: isize,
    max_mp: isize,
    name: String,
    aa: String, // ascii art
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

struct context {
    monsters: [Character; MonsterEnum::Max as usize],
    characters: [Character; CharacterEnum::Max as usize],
}

impl context {
    pub fn new() -> Self {
        Self {
            monsters: [
                Character {
                    hp: 15,
                    max_hp: 15,
                    mp: 15,
                    max_mp: 15,
                    name: "용사".to_string(),
                    aa: "".to_string(),
                },
                Character {
                    hp: 3,
                    max_hp: 3,
                    mp: 0,
                    max_mp: 0,
                    name: "슬라임".to_string(),
                    aa: "/·Д·\\\n".to_string(),
                },
            ],
            characters: [Character::default(), Character::default()],
        }
    }
}

fn init(ctx: &mut context) {
    ctx.characters[CharacterEnum::Player as usize] =
        ctx.monsters[MonsterEnum::Player as usize].clone();
}

fn draw_battle_screen(ctx: &context) {
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

fn battle(ctx: &mut context, monster: MonsterEnum) {
    ctx.characters[CharacterEnum::Monster as usize] = ctx.monsters[monster as usize].clone();
    draw_battle_screen(ctx);
    println!("{}이(가) 나타났다!", ctx.characters[monster as usize].name);
}

fn main() {
    let mut ctx = context::new();
    init(&mut ctx);
    battle(&mut ctx, MonsterEnum::Slime);
}
