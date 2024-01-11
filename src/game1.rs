use std::fmt::Debug;

struct Monster {
    health: i32,
}

struct Wizard;

#[derive(Debug)]
struct Ranger;

trait FightClose {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!("You attack with your sword, your opponent now has {} health left", opponent.health);
    }

    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!("You attack with your hand, your opponent now has {} health left", opponent.health);
    }
}

impl FightClose for Wizard {}

impl FightClose for Ranger {}

trait FightFromDistance: Debug {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!("You attack with your bow, your opponent now has {} health left", opponent.health);
        }
    }

    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
            println!("You attack with your rock, your opponent now has {} health left", opponent.health);
        }
    }
}

impl FightFromDistance for Ranger {}

//trait bounds

trait Magic {}

fn fireball<T>(character: &T, opponent: &mut Monster, distance: u32)
    where T: Debug + Magic {
    if distance < 15 {
        opponent.health -= 20;

        println!("You attack with your fireball, your opponent now has {} health left, you are now at{:?}", opponent.health, character);
    }
}

