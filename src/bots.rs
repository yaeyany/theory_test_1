use std::{collections::BTreeMap, fmt::Display, ops::AddAssign};


pub struct Bot {
    title: BotTitle,
    coordinates: BotCoordinates,
    state: BotState,
    charge: BotCharge
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BotId(u32);

pub enum BotState{
    Stationary,
    Moving
}

pub struct BotCoordinates(i32,i32);

#[derive(Clone)]
pub struct BotTitle(String);

pub struct BotCharge(u8);

pub struct BotStore {
    id: BotId,
    bots: BTreeMap<BotId, Bot>
}

impl AddAssign<u32> for BotId {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs
    }
}

impl Display for BotId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for BotCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

impl Display for BotState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BotState::Stationary => write!(f, "Stationary"),
            BotState::Moving => write!(f, "Moving"),
        }
    }
}

impl Display for BotTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for BotCharge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BotStore {
    pub fn new() -> Self {
        Self { 
            id: BotId(0), 
            bots: BTreeMap::new() 
        }
    }

    pub fn add(&mut self, title: BotTitle, state: BotState, coordinates: BotCoordinates, charge: BotCharge) -> BotId {
        let id = self.id;
        let bot = Bot { 
            title: title, 
            state: state,
            coordinates: coordinates,
            charge: charge, 
        };
        
        self.bots
        .insert(id, bot);
        
        self.id += 1;
        id
    }

    pub fn list(&self) -> Vec<(BotId, BotTitle)> {
        self.bots.iter().map(|(id, bot)| (id.clone(), bot.title.clone())).collect::<Vec<(BotId, BotTitle)>>()
    }
}

