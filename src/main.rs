use minigrep::*;
use std::{collections::HashMap, env, process};

fn main()
{
    let mut settings: HashMap<String, String> = HashMap::new();

    for argument in env::args().skip(1)
    {
        match argument.split_once(" ")
        {
            Some(references) =>
            {
                settings.insert(references.0.to_string().to_lowercase(), references.1.to_string());
            }
            None =>
            {
                eprintln!("Cannot parse argument '{}'", argument);
            }
        }
    }

    if settings.is_empty()
    {
        help();
        process::exit(0);
    }

    let path;
    match settings.get("path")
    {
        Some(_path) =>
        {
            path = _path;
        }
        None =>
        {
            panic!("No path found");
        }
    }

    let str_to_find;
    match settings.get("find")
    {
        Some(to_find) =>
        {
            str_to_find = to_find;
        }
        None =>
        {
            panic!("Did not specify what to find");
        }
    }

    let file_content;
    match file_result(path)
    {
        Ok(content) =>
        {
            file_content = content;
        }
        Err(e) =>
        {
            panic!("{}", e);
        }
    }

    let is_case_sensitive;
    match case_sensitiveness(&settings)
    {
        Ok(value) =>
        {
            is_case_sensitive = value;
        }
        Err(e) =>
        {
            panic!("{}", e);
        }
    }
    
    let mut outcome = Outcome::new(&file_content, &str_to_find, is_case_sensitive);

    match settings.get("separate")
    {
        Some(value) =>
        {
            match value.parse::<bool>()
            {
                Ok(bool_value) =>
                {
                    if bool_value
                    {
                        outcome = outcome.separate();
                    }
                }
                Err(e) =>
                {
                    panic!("{}", e);
                }
            }
        }
        None => {}
    }

    println!("{}", outcome);
}
