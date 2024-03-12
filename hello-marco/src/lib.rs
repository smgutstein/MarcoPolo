/*  A Marco Polo game is a simple word game that requires two players.
The first player says "Marco" and the program will reply "Polo",
otherwise the program asks for the player's name */

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        return String::from("Polo");
    } else {
        return String::from("What's your name?");
    }
}
