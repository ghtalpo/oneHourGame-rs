#[derive(Debug,Default)]
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

fn main() {
    println!("Hello, world!");
    let monsters:[Character;MonsterEnum::Max as usize] = [Character {
        hp: 15,
        max_hp: 15,
        mp: 15,
        max_mp: 15,
        name : "용사".to_string(),
    }];
    println!("monsters? {:?}", monsters);
    let characters:[Character;CharacterEnum::Max as usize] = [Character::default(),Character::default()
    ];
    println!("characters? {:?}", characters);
}
