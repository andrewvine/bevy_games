use bevy::prelude::*;

const METEOR_SIZE_BIG: u16 = 84;
const METEOR_SIZE_MED: u16 = 43;
const METEOR_SIZE_SMALL: u16 = 28;
const METEOR_SIZE_TINY: u16 = 18;

#[derive(Clone)]
pub struct MeteorType {
    pub size: u16, 
    pub image: String,
    pub speed: f32,
}

pub fn random_type() -> MeteorType {
    let types = [
        MeteorType{size: METEOR_SIZE_BIG, image: String::from("sprites/meteors/meteorGrey_big1.png"), speed: 60.0 },
        MeteorType{size: METEOR_SIZE_BIG, image: String::from("sprites/meteors/meteorBrown_big1.png"), speed: 60.0},
        MeteorType{size: METEOR_SIZE_MED, image: String::from("sprites/meteors/meteorGrey_med1.png"), speed: 80.0},
        MeteorType{size: METEOR_SIZE_MED, image: String::from("sprites/meteors/meteorBrown_med1.png"), speed: 80.0},
        MeteorType{size: METEOR_SIZE_SMALL, image: String::from("sprites/meteors/meteorGrey_small1.png"), speed: 120.0},
        MeteorType{size: METEOR_SIZE_SMALL, image: String::from("sprites/meteors/meteorBrown_small1.png"), speed: 120.0},
        MeteorType{size: METEOR_SIZE_TINY, image: String::from("sprites/meteors/meteorGrey_tiny1.png"), speed: 180.0},
        MeteorType{size: METEOR_SIZE_TINY, image: String::from("sprites/meteors/meteorBrown_tiny1.png"), speed: 180.0},
    ];
    let x = rand::random::<u8>() % types.len() as u8;
    return types[x as usize].clone();
}


#[derive(Component)]
pub struct Meteor {
    pub direction: Vec2,
    pub size: u16, 
    pub speed: f32,
}

impl Meteor {
    pub fn explosion(&self) -> String {
        match self.size {
            METEOR_SIZE_BIG => String::from("sprites/effects/explosion01.png"),
            METEOR_SIZE_MED => String::from("sprites/effects/explosion02.png"),
            METEOR_SIZE_SMALL => String::from("sprites/effects/explosion03.png"),
            _ => String::from("sprites/effects/explosion04.png"),
        }
    }
}