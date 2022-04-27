use std::io;

const HERO_ONE: &str = "Hero One";
const HERO_TWO: &str = "Hero Two";
const HERO_THREE: &str = "Hero Three";

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

struct Point {
    x: i32,
    y: i32,
}

enum EntityType {
    Monster = 0,
    FriendHero,
    OpponentHero,
}

impl EntityType {
    fn convert_number(n: i32) -> Option<EntityType> {
        match n {
            0 => Some(EntityType::Monster),
            1 => Some(EntityType::FriendHero),
            2 => Some(EntityType::OpponentHero),
            _ => None,
        }
    }
}

trait Entity {
}

struct Hero {
}

struct Monster {
}

struct BaseData {
    position: Point,
    health: i32,
    mana: i32,
    hero_count: i32,
}

fn read_initial_input() -> BaseData {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x = parse_input!(inputs[0], i32); // The corner of the map representing your base
    let y = parse_input!(inputs[1], i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let hero_count = parse_input!(input_line, i32); // Always 3
    return BaseData {
        position: Point { x, y },
        health: 0,
        mana: 0,
        hero_count,
    };
}

fn read_player_data(base_data: &mut BaseData) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    base_data.health = parse_input!(inputs[0], i32); // Each player's base health
    base_data.mana = parse_input!(inputs[1], i32); // Ignore in the first league; Spend ten mana to cast a spell
}

fn point_distance(a: &Point, b: &Point) -> i32 {
    let val: f64 = ((a.x - b.x).abs() + (a.y - b.y).abs()) as f64;
    return val.sqrt() as i32;
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut base_data: BaseData = read_initial_input();
    let mut opponent_base_data = BaseData {
        position: Point { x: 0, y: 0 },
        health: base_data.health,
        hero_count: base_data.hero_count,
        mana: base_data.mana
    };
    // game loop
    loop {
        read_player_data(&mut base_data);
        read_player_data(&mut opponent_base_data);

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, i32); // Amount of heros and monsters you can see
        for i in 0..entity_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let id = parse_input!(inputs[0], i32); // Unique identifier
            let _type = parse_input!(inputs[1], i32); // 0=monster, 1=your hero, 2=opponent hero
            let x = parse_input!(inputs[2], i32); // Position of this entity
            let y = parse_input!(inputs[3], i32);
            let shield_life = parse_input!(inputs[4], i32); // Ignore for this league; Count down until shield spell fades
            let is_controlled = parse_input!(inputs[5], i32); // Ignore for this league; Equals 1 when this entity is under a control spell
            let health = parse_input!(inputs[6], i32); // Remaining health of this monster
            let vx = parse_input!(inputs[7], i32); // Trajectory of this monster
            let vy = parse_input!(inputs[8], i32);
            let near_base = parse_input!(inputs[9], i32); // 0=monster with no target yet, 1=monster targeting a base
            let threat_for = parse_input!(inputs[10], i32); // Given this monster's trajectory, is it a threat to 1=your base, 2=your opponent's base, 0=neither
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        // In the first league: MOVE <x> <y> | WAIT; In later leagues: | SPELL <spellParams>;
        println!("WAIT");
        println!("WAIT");
        println!("WAIT");
    }
}
