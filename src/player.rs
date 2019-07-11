// Copyright © 2019 Ebraheem AlAthari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
#[allow(dead_code)]

#[derive(Debug)]
enum Weapons {
    Fists,
    Dagger,
    Sword,
    Mace,
    Axe,
    Pickaxe,
}

struct Player<'a> {
    name: &'a str,
    weapon_mulitplyer: u8,
    weapon: Weapons,
}

pub fn setup_player() {
    let team_player: Player = Player{name: "Test", weapon_mulitplyer: 0, weapon: Weapons::Fists};
    println!("{} {} {:?}", team_player.name, team_player.weapon_mulitplyer, team_player.weapon);
}
