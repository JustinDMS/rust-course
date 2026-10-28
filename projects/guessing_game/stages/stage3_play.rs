//! Stage 3: the game loop, scripted.

use guessing_game::play;

#[test]
fn a_short_game() {
    let messages = play(&["50", "25", "37"], 37);
    assert_eq!(messages, vec!["Too big!", "Too small!", "You win!"]);
}

#[test]
fn bad_input_asks_for_a_number_and_continues() {
    let messages = play(&["abc", "", "10"], 10);
    assert_eq!(
        messages,
        vec!["Please type a number!", "Please type a number!", "You win!"]
    );
}

#[test]
fn the_game_stops_at_the_first_win() {
    let messages = play(&["5", "5", "6"], 5);
    assert_eq!(messages, vec!["You win!"]);
}

#[test]
fn running_out_of_input_just_ends() {
    let messages = play(&["1", "2"], 99);
    assert_eq!(messages, vec!["Too small!", "Too small!"]);
    assert!(play(&[], 1).is_empty());
}
