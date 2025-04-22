use std::f32::consts::E;
use crate::protos;
use anyhow::{anyhow, Result};
use protos::p_file::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MESSAGE_RE: Regex = Regex::new(r"^message\s+([a-zA-Z0-9_]+)\s*\{\s*(\/\/.*)?$").unwrap();
}

pub struct Parser {
    file_name: String,
    token: Option<String>,
    line_iter: Box<dyn Iterator<Item=Result<String>>>,
    comment_collections: String,
}
impl Parser {
    pub fn new(file_name: &str) -> Result<Self> {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file).lines();
        let mut ft: Option<String> = None;
        for line in &reader {
            match line {
                Ok(T) => {
                    if T.trim().len() == 0 {
                        continue;
                    }
                    ft = Some(String::from(T.trim()));
                }
                Err(e) => {
                    return Err(anyhow!(e))
                }
            }
        }
        match ft {
            None => {
                return Err(anyhow!("empty content"))
            }
            _ => {}
        }

        Ok(Parser {
            file_name: String::from(file_name),
            token: ft,
            line_iter: Box::new(reader),
        })
    }
    pub fn clone_token(&self) -> Option<String> {
        match &self.token {
            None => {
                None
            }
            Some(s) => {
                Some(s.clone())
            }
        }
    }
    pub fn next(&mut self) -> Result<Option<String>> {
        let tmp_opt = self.line_iter.next();
        let tmp: String;
        match tmp_opt {
            None => {
                return Ok(None)
            }
            Some(t) => {
                match t {
                    Err(e) => {
                        return Err(anyhow!(e))
                    }
                    Ok(s) => {
                        tmp = String::from(s.trim());
                    }
                }
            }
        }
        if tmp.len() == 0 {
            return self.next();
        }
        self.token = Some(tmp);
        Ok(self.clone_token())
    }
    pub fn get_token(&self) -> Option<String> {
        self.clone_token()
    }
    pub fn parse(&mut self) -> Result<ProtoFile> {
        // parse import files
        let mut import_file_list = vec![];
        loop {
            let now_token = self.get_token();
            match now_token {
                None => {
                    return Err(anyhow!("reach EOF when parsing imports"))
                }
                Some(s) => {
                    let import_file = Parser::parse_import(&s);
                    match import_file {
                        Some(f) => {
                            import_file_list.push(f?);
                        }
                        None => {
                            break
                        }
                    }
                }
            }
            self.next()?;
        }

        // parse body, message/enum/service
        Ok(ProtoFile {
            name: "".to_string(),
            path: "".to_string(),
            service_list: vec![],
            import_file_list: vec![],
            message_list: vec![],
            comment: "".to_string(),
        })
    }

    fn parse_import(line: &str) -> Option<Result<ImportedFile>> {
        if !line.starts_with("import") {
            return None;
        }
        if !line.ends_with(";") {
            return Some(Err(anyhow!("import line should end with ;")));
        }
        let line = line.trim_end_matches(";");
        let mut fields = line.split_whitespace();
        // import
        match fields.next() {
            Some(s) => {
                if s != "import" {
                    return Some(Err(anyhow!("first field should be keyword import")));
                }
            }
            None => {
                return Some(Err(anyhow!("first field should be keyword import")))
            }
        }
        // file path
        let file_path = match fields.next() {
            Some(s) => {
                if !s.starts_with("\"") || !s.ends_with("\"") {
                    return Some(Err(anyhow!("invalid file path, should be start and end with quote")));
                }
                let tmp = s.trim_start_matches("\"").trim_end_matches("\"");
                if tmp.len() == 0 {
                    return Some(Err(anyhow!("empty file path")));
                }
                PathBuf::from(tmp)
            }
            None => {
                return Some(Err(anyhow!("second field should be file path")))
            }
        };
        // alias name
        let alias_name = match fields.next() {
            None => {
                None
            }
            Some(t) => {
                if t != "as" {
                    return Some(Err(anyhow!("has third field but not keyword as")));
                }
                match fields.next() {
                    None => {
                        return Some(Err(anyhow!("should be alias name after as")))
                    }
                    Some(s) => {
                        Some(String::)
                    }
                }
            }
        };
        let file_name = match file_path.file_name() {
            None => {
                return Some(Err(anyhow!("get file name failed")))
            }
            Some(s) => {
                s
            }
        };
        let parser = Parser::new(file_path.to_str()?).unwrap();
        let f = parser.parse().unwrap();
        let res = ImportedFile {
            path: file_path,
            name: String::from(file_name),
            alias: alias_name,
            real_file: Rc::new(f),
        };
        Some(Ok(res))
    }

    fn parse_line(self, reader: &BufReader<File>) -> Vec<ImportedFile> {}

    fn parse_message(&mut self) -> Result<Message> {

        // get message define header
        // format: message MsgName{ // comment
        let header = match self.get_token() {
            None => {
                return Err(anyhow!("got EOF when starting to parse message header"))
            }
            Some(s) => {
                s
            }
        };

        let (full,[name,comment])= MESSAGE_RE.captures(&header).unwrap().extract();
        if name.len() == 0 {
            return Err(anyhow!("empty message name"))
        }
        Message {
            name: String::from(name),
            fields: vec![],

        }
    }
}

