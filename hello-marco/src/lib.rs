/* Marco Polo Game

if the name Marco is given, the response should be Polo
otherwise, the response should be "what's your name?"

*/

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "What's your name?".to_string()
    }
}


