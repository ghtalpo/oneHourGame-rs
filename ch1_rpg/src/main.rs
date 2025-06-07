#[derive(Debug, Default, Clone)]
struct Character {
    hp: isize,
    max_hp: isize,
    mp: isize,
    max_mp: isize,
    name: String,
    aa: String, // ascii art
}

enum MonsterEnum {
    Player,
    Slime,
    Max,
}

enum CharacterEnum {
    Player,
    Monster,
    Max,
}

struct context {
    monsters: [Character; MonsterEnum::Max as usize],
    characters: [Character; CharacterEnum::Max as usize],
}

impl context {
    pub fn new() -> Self {
        Self {
            monsters: [Character {
                hp: 15,
                max_hp: 15,
                mp: 15,
                max_mp: 15,
                name: "용사".to_string(),
                aa: "".to_string(),
            },Character {
                hp: 3,
                max_hp: 3,
                mp: 0,
                max_mp: 0,
                name: "슬라임".to_string(),
                aa: "/·Д·\\".to_string(),
            }],
            characters: [Character::default(), Character::default()],
        }
    }
}

fn init(ctx: &mut context) {
    ctx.characters[CharacterEnum::Player as usize] =
        ctx.monsters[MonsterEnum::Player as usize].clone();
}

fn draw_battle_screen(ctx: &context) {
    println!("{}\n", ctx.characters[CharacterEnum::Player as usize].name);
    println!(
        "HP:{}/{} MP:{}/{} \n",
        ctx.characters[CharacterEnum::Player as usize].hp,
        ctx.characters[CharacterEnum::Player as usize].max_hp,
        ctx.characters[CharacterEnum::Player as usize].mp,
        ctx.characters[CharacterEnum::Player as usize].max_mp
    );
}

fn battle(ctx: &context) {
    draw_battle_screen(ctx);
}

fn main() {
    let mut ctx = context::new();
    println!("monsters? {:?}", ctx.monsters);
    println!("characters? {:?}", ctx.characters);
    init(&mut ctx);
    println!("monsters? {:?}", ctx.monsters);
    println!("characters? {:?}", ctx.characters);
    draw_battle_screen(&ctx);
}
