use crate::{AppResult, fail};

pub(crate) enum Command {
    Migrate,
    Serve,
}

pub(crate) fn parse() -> AppResult<Command> {
    parse_from(std::env::args().skip(1))
}

fn parse_from<I, S>(arguments: I) -> AppResult<Command>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut arguments = arguments.into_iter();
    let command = match arguments.next() {
        Some(argument) if argument.as_ref() == "migrate" => Command::Migrate,
        Some(argument) if argument.as_ref() == "serve" => Command::Serve,
        _ => return Err(fail("usage: app <migrate|serve>")),
    };
    if arguments.next().is_some() {
        return Err(fail("usage: app <migrate|serve>"));
    }
    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_exactly_one_known_command() {
        assert!(matches!(parse_from(["migrate"]), Ok(Command::Migrate)));
        assert!(matches!(parse_from(["serve"]), Ok(Command::Serve)));
        assert!(parse_from(std::iter::empty::<&str>()).is_err());
        assert!(parse_from(["unknown"]).is_err());
        assert!(parse_from(["serve", "extra"]).is_err());
    }
}
