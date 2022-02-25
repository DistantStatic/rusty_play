use std::io;

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
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(u32,  u32 ,u32)
}

fn get_move() -> Message {
    println!("Enter X:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read line");
    let input_x: i32 = input.trim().parse().expect("Unable to parse to i32");
    input.clear();
    println!("Enter Y:");
    io::stdin().read_line(&mut input).expect("Unable to read line");
    let input_y: i32 = input.trim().parse().expect("Unable to parse to i32");
    Message::Move{x: input_x, y: input_y}
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

fn get_input() -> Message {
    println!("What would you like to do?\n");
    println!("1: Quit\n2: Move\n3: Write\n4: Change Color\n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read line");
    let choice = input.trim().parse().expect("Unable to parse to number");
    match choice {
        1 => Message::Quit(),
        2 => get_move(),
        3 => get_write(),
        4 => get_color(),
        _ => get_input()
    }
}

fn main() {
    let mut my_player = Player::new();

    loop {
        match get_input() {
            Message::Quit() => break,
            Message::Move{x, y} => my_player.position = Point::from(x, y),
            Message::Write(message) => println!("Written Message ...\n{}", message),
            Message::ChangeColor(r, b, g) => println!("new color r:{}, b:{}, g:{}", r, b, g),
        }
    }

}
