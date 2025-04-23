use std::{collections::HashMap, fmt, fs, io::{self, Write}};

pub struct Outcome<'a>
{
    lines_with_str: Vec<(usize, &'a str)>,
    to_find: &'a str,
    is_case_sens: bool
}
impl<'a> Outcome<'a>
{
    pub fn new(text: &'a str, str_to_find: &'a str, case_sensitive: bool) -> Outcome<'a>
    {
        let mut valid_lines = Vec::new();

        for (line_num, line) in text.lines().enumerate()
        {
            if case_sensitive
            {
                if line.contains(str_to_find)
                {
                    valid_lines.push((line_num, line));
                }
            }
            else
            {
                if line.to_lowercase().contains(&str_to_find.to_lowercase())
                {
                    valid_lines.push((line_num, line));
                }
            }
        }

        Outcome
        {
            lines_with_str: valid_lines,
            to_find: str_to_find,
            is_case_sens: case_sensitive
        }
    }

    pub fn separate(&self) -> Outcome<'a>
    {
        let mut occurrences_as_separate = Vec::new();
        for (line_num, line) in &self.lines_with_str
        {
            let line_to_check =
                if self.is_case_sens
                {
                    line.to_string()
                }
                else
                {
                    line.to_lowercase()
                };

            let occurrences: Vec<String> = line_to_check.split(" ")
                .filter(|element|
                    if self.is_case_sens
                    {
                        *element == self.to_find
                    }
                    else
                    {
                        element == &self.to_find.to_lowercase()
                    })
                .map(|element| element.to_string())
                .collect();
            
            if !occurrences.is_empty()
            {
                occurrences_as_separate.push((*line_num, *line));
            }
        }
        Outcome
        {
            lines_with_str: occurrences_as_separate,
            to_find: self.to_find,
            is_case_sens: self.is_case_sens
        }
    }
}
impl<'a> fmt::Display for Outcome<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for (line_num, line) in &self.lines_with_str
        {
            writeln!(f, "Line {}: {}", line_num + 1, line)?;
        }
        Ok(())
    }
}

pub fn help()
{
    print!("How to use:\n\tminigrep \"Option Value\" (Arguments are case-insensitive)");
    print!("\nPath: takes the relative or absolute path to the file");
    print!("\nFind: takes a string of characters");
    print!("\nCase: takes true or false");
    print!("\nSeparate: takes true or false");
    io::stdout().flush().expect("Flush did not work");
}

pub fn file_result(path: &str) -> Result<String, io::Error>
{
    let content = fs::read_to_string(path)?;
    Ok(content)
}

pub fn case_sensitiveness(map: &HashMap<String, String>) -> Result<bool, core::str::ParseBoolError>
{
    match map.get("case")
    {
        Some(is_case_sens) =>
        {
            match is_case_sens.parse::<bool>()
            {
                Ok(value) => Ok(value),
                Err(e) => Err(e)
            }
        }
        None => Ok(true)
    }
}