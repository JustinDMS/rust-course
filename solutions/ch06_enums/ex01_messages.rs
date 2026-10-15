//! # An enum with data (solution)

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn describe(&self) -> String {
        match self {
            Message::Quit => String::from("quit"),
            Message::Move { x, y } => format!("move to ({x}, {y})"),
            Message::Write(text) => format!("write '{text}'"),
            Message::ChangeColor(r, g, b) => format!("color #{r:02x}{g:02x}{b:02x}"),
        }
    }

    fn is_noop(&self) -> bool {
        match self {
            Message::Move { x: 0, y: 0 } => true,
            Message::Write(text) => text.is_empty(),
            _ => false,
        }
    }
}

#[test]
fn descriptions() {
    assert_eq!(Message::Quit.describe(), "quit");
    assert_eq!(Message::Move { x: 1, y: -2 }.describe(), "move to (1, -2)");
    assert_eq!(Message::Write(String::from("hi")).describe(), "write 'hi'");
    assert_eq!(Message::ChangeColor(0, 10, 255).describe(), "color #000aff");
}

#[test]
fn noops() {
    assert!(Message::Move { x: 0, y: 0 }.is_noop());
    assert!(Message::Write(String::new()).is_noop());
    assert!(!Message::Move { x: 0, y: 1 }.is_noop());
    assert!(!Message::Write(String::from("x")).is_noop());
    assert!(!Message::Quit.is_noop());
    assert!(!Message::ChangeColor(0, 0, 0).is_noop());
}

#[test]
fn debug_is_derived() {
    assert_eq!(format!("{:?}", Message::Quit), "Quit");
    assert_eq!(format!("{:?}", Message::Move { x: 3, y: 4 }), "Move { x: 3, y: 4 }");
}
