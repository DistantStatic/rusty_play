use std::io;

struct GameBoard {
    corners: [Point; 4],
}

impl GameBoard {
    fn new() -> GameBoard {
        GameBoard {
            corners: [
                Point::from(0, 0),
                Point::from(0, 100),
                Point::from(100, 100),
                Point::from(100, 0),
            ]
        }
    }
    fn validate_move(self: &Self, coord: Point) -> bool {
        self.check_bounds(coord)
    }
    fn check_bounds(self: &Self, coord: Point) -> bool {
        for point in &self.corners {
            if coord.x > point.x || coord.y > point.y {
                return false;
            }
        }
        return true;
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
        }
    }
    fn new() -> Point {
        Point {
            x: 0,
            y: 0,
        }
    }
}

struct Player {
    position: Point,
    color: (u32, u32, u32)
}

impl Player {
    fn from(position: Point, color: (u32, u32, u32) ) -> Player {
        Player {
            position,
            color,
        }
    }
    fn new() -> Player {
        Player {
            position: Point::new(),
            color: (0, 0, 0)
        }
    }
    fn display_details(self: &Self) {
        println!("You are:\n\tPosition: ({},{}),\n\tColor: ({}, {}, {})", self.position.x, self.position.y, self.color.0, self.color.1, self.color.2);
    }
}

enum Message {
    Quit(),
    Move{point: Point},
    Write(String),
    ChangeColor(u32,  u32 ,u32)
}

fn get_move(player: &Player) -> Message {
    let mut wasd = String::new();

    let mut input_x: i32 = player.position.x;
    let mut input_y: i32 = player.position.y;
    loop {
        println!("Where would you like to go?\n  w\na s d\n");
        io::stdin().read_line(&mut wasd).expect("Unable to read line");

        println!("Received: {}", wasd.trim());
        match wasd.trim() {
            "w" => {input_y += 1; break},
            "a" => {input_x -= 1; break},
            "s" => {input_y -= 1; break},
            "d" => {input_x += 1; break},
            _ => {wasd.clear(); continue},
        }
    }
    
     Message::Move{point: Point{ x: input_x, y: input_y}}
}

fn get_write() -> Message {
    let mut input = String::new();
    println!("What would you like to write?");
    io::stdin().read_line(&mut input).expect("Unable to read line");
    Message::Write(input)
}

fn get_color() -> Message {
    let mut input = String::new();
    println!("Enter (R)ed value:");
    io::stdin().read_line(&mut input).expect("Unable to read line");
    let r_val: u32 = input.trim().parse().expect("Unable to parse (R)ed to u32)");
    input.clear();

    println!("Enter (B)lue value");
    io::stdin().read_line(&mut input).expect("Unable to read line");
    let b_val: u32 = input.trim().parse().expect("Unable to parse (B)lue to u32");
    input.clear();

    println!("Enter (G)reen value");
    io::stdin().read_line(&mut input).expect("Unable to read line");
    let g_val: u32 = input.trim().parse().expect("Unable to partse (G)reen to u32");
    input.clear();

    Message::ChangeColor(r_val, b_val, g_val)
}

fn get_input(player: &Player) -> Message {
    player.display_details();
    println!("What would you like to do?\n");
    println!("1: Quit\n2: Move\n3: Write\n4: Change Color\n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read line");
    let choice = input.trim().parse().expect("Unable to parse to number");
    match choice {
        1 => Message::Quit(),
        2 => get_move(player),
        3 => get_write(),
        4 => get_color(),
        _ => get_input(player)
    }
}

fn main() {
    let mut my_player = Player::new();

    loop {
        match get_input(&my_player) {
            Message::Quit() => break,
            Message::Move{point} => my_player.position = Point::from(point.x, point.y),
            Message::Write(message) => println!("Written Message ...\n{}", message),
            Message::ChangeColor(r, b, g) => println!("new color r:{}, b:{}, g:{}", r, b, g),
        }
    }
    println!("Goodbye");

}
