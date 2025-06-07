#[derive(Debug,Default,Clone)]
struct Character {
    hp: isize,
    max_hp: isize,
    mp: isize,
    max_mp: isize,
    name: String,
}

enum MonsterEnum {
    Player,
    Max,
}

enum CharacterEnum {
    Player,
    Monster,
    Max,
}

struct context {
    monsters: [Character;MonsterEnum::Max as usize],
    characters: [Character;CharacterEnum::Max as usize],
}

impl context {
    pub fn new() -> Self {
        Self {
            monsters:[Character {
        hp: 15,
        max_hp: 15,
        mp: 15,
        max_mp: 15,
        name : "용사".to_string(),
    }],
    characters: [Character::default(),Character::default()
    ]
        }
    }
}
fn init(ctx: &mut context) {
    ctx.characters[CharacterEnum::Player as usize] = ctx.monsters[MonsterEnum::Player as usize].clone();
}

fn main() {
    let mut ctx = context::new();
    println!("monsters? {:?}", ctx.monsters);
    println!("characters? {:?}", ctx.characters);
    init(&mut ctx);
    println!("monsters? {:?}", ctx.monsters);
    println!("characters? {:?}", ctx.characters);
}
