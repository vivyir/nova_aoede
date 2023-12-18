use crate::types::CommandVec;

mod help;
mod ping;
mod piped;

pub fn commands() -> CommandVec {
    let mut piped = piped::piped();
    piped.description = Some("PLEASE WORK PLEASE APELAE".into());
    vec![help::help(), ping::ping(), piped]
}
