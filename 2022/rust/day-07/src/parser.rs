use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline, u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
pub enum Command<'a> {
    Ls(Vec<TreeStruct<'a>>),
    Cd(&'a str),
}

#[derive(Debug, Clone)]
pub enum TreeStruct<'a> {
    Dir(&'a str),
    File((u32, &'a str)),
}

pub fn parse_file(input: &str) -> IResult<&str, TreeStruct> {
    let (input, (file_size, file_name)) =
        separated_pair(u32, tag(" "), is_a("qwertyuiopasdfghjklzxcvbnm."))(input)?;
    Ok((input, TreeStruct::File((file_size, file_name))))
}

pub fn parse_dir(input: &str) -> IResult<&str, TreeStruct> {
    let (input, _) = tag("dir ")(input)?;
    let (input, dir_name) = alpha1(input)?;
    Ok((input, TreeStruct::Dir(dir_name)))
}

pub fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((parse_file, parse_dir)))(input)?;
    Ok((input, Command::Ls(files)))
}

pub fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir_name) = alt((tag(".."), tag("/"), alpha1))(input)?;
    Ok((input, Command::Cd(dir_name)))
}

pub fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, commands) = separated_list1(newline, alt((parse_ls, parse_cd)))(input)?;
    Ok((input, commands))
}
