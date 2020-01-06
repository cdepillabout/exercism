pub fn reply(message: &str) -> &str {
    // He answers 'Whoa, chill out!' if you YELL AT HIM (in all capitals).
    // He says 'Fine. Be that way!' if you address him without actually saying anything.
    // He answers 'Calm down, I know what I'm doing!' if you yell a question at him.
    // Bob answers 'Sure.' if you ask him a question, such as "How are you?".
    // He answers 'Whatever.' to anything else.

    // Is this a question?  Is the last non-whitespace character a question mark?
    let is_question = message.trim().ends_with("?");

    // Are there any alphabetic characters?
    let is_no_alphabetic = message.chars().all(|c| !c.is_alphabetic());

    // Are all the alphabetic characters uppercase?  Note that this returns false when there are no
    // alphabetic characters.
    let is_all_uppercase = message.chars().all(|c| !c.is_alphabetic() || c.is_uppercase()) && !is_no_alphabetic;

    // All all characters whitespace?
    let is_all_space = message.chars().all(|c| c.is_ascii_whitespace());

    match (is_question, is_all_uppercase, is_all_space) {
        (true, true, false) => "Calm down, I know what I'm doing!",
        (true, false, false) => "Sure.",
        (false, true, false) => "Whoa, chill out!",
        (_, _, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
