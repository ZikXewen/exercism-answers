pub fn reply(message: &str) -> &str {
    let message = message.trim();
    match (
        message.ends_with("?"),
        message == message.to_uppercase() && message != message.to_lowercase(),
    ) {
        _ if message.is_empty() => "Fine. Be that way!",
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}
