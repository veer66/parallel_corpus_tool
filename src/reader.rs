use crate::config::Config;
use crate::lang::LangKey;
use crate::link::Link;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

lazy_static! {
    static ref LINK_RE: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
}

pub struct Reader {
    pub config: Config,
}

quick_error! {
    #[derive(Debug, Clone)]
    pub enum ParsingError {
        ParseLink { }
        GetValue { }
        ParseNumber { }
    }
}

impl Reader {
    pub fn parse_link(txt: &str) -> Result<Link, ParsingError> {
        let caps = LINK_RE.captures(txt).ok_or(ParsingError::ParseLink)?;
        let source = caps.get(1).ok_or(ParsingError::GetValue)?;
        let target = caps.get(2).ok_or(ParsingError::GetValue)?;
        Ok(Link {
            source: source
                .as_str()
                .parse()
                .map_err(|_| ParsingError::ParseNumber)?,
            target: target
                .as_str()
                .parse()
                .map_err(|_| ParsingError::ParseNumber)?,
        })
    }

    pub fn parse_links(line: &str) -> Result<Vec<Link>, ParsingError> {
        let links: Vec<_> = line
            .split_whitespace()
            .map(|tok| Self::parse_link(tok))
            .collect();
        for l in &links {
            if l.is_err() {
                let e = l.as_ref().err();
                return Err(e.unwrap().clone());
            }
        }
        let links: Vec<Link> = links.into_iter().map(|l| l.unwrap()).collect();
        Ok(links)
    }

    pub fn parse_toks(line: &str) -> Vec<String> {
        line.split_whitespace().map(|tok| tok.to_string()).collect()
    }

    fn lang_key_to_lang(&self, lang_key: LangKey) -> String {
        match lang_key {
            LangKey::SOURCE => self.config.langs.source.to_string(),
            LangKey::TARGET => self.config.langs.target.to_string(),
        }
    }

    pub fn read_toks(&self, lang_key: LangKey) -> Result<Vec<Vec<String>>, Box<Error>> {
        let lang = self.lang_key_to_lang(lang_key);
        let path = format!(
            "{}/{}.{}",
            self.config.corpus_dir, self.config.tok_prefix, lang
        );
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let toks_list: Vec<_> = reader
            .lines()
            .skip(self.config.textunit_offset)
            .take(self.config.textunit_limit)
            .map(|line| Self::parse_toks(&line.unwrap()))
            .collect();
        Ok(toks_list)
    }

    pub fn read_lines(&self, lang_key: LangKey) -> Result<Vec<String>, Box<Error>> {
        let lang = self.lang_key_to_lang(lang_key);
        let path = format!(
            "{}/{}.{}",
            self.config.corpus_dir, self.config.orig_prefix, lang
        );
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines: Vec<_> = reader
            .lines()
            .skip(self.config.textunit_offset)
            .take(self.config.textunit_limit)
            .map(|line| line.unwrap())
            .collect();
        Ok(lines)
    }

    pub fn read_links(&self) -> Result<Vec<Vec<Link>>, Box<Error>> {
        let f = File::open(&self.config.alignment_file_path)?;
        let reader = BufReader::new(f);
        let mut links_list = vec![];
        for line in reader
            .lines()
            .skip(self.config.textunit_offset)
            .take(self.config.textunit_limit)
        {
            let line = line?;
            let links = Self::parse_links(&line)?;
            links_list.push(links);
        }
        Ok(links_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::link::Link;
    use crate::config::Langs;
    use crate::lang::LangKey;
    
    #[test]
    fn parse_simple_links() {
        assert_eq!(
            Reader::parse_links("1-2 3-4").unwrap(),
            vec![
                Link {
                    source: 1,
                    target: 2
                },
                Link {
                    source: 3,
                    target: 4
                }
            ]
        )
    }

    #[test]
    fn parse_simple_link() {
        assert_eq!(
            Reader::parse_link("1-2").unwrap(),
            Link {
                source: 1,
                target: 2
            }
        )
    }

    #[test]
    fn read_simple_lines() {
        let root = env!("CARGO_MANIFEST_DIR");
        let langs = Langs {source: String::from("en"),
                           target: String::from("th")};        
        let conf = Config {
            corpus_dir: format!("{}/data", root),
            tok_prefix: String::from(""),
            langs: langs,
            alignment_file_path: String::from(""),
            orig_prefix: String::from("simple_lines"),
            output_amphigram_path: String::from(""),
            textunit_limit: 100,
            textunit_offset: 0
        };
        let reader = Reader { config: conf };
        let lines = reader.read_lines(LangKey::SOURCE).unwrap();
        let expected = vec![String::from("ABC"), String::from("EFG")];        
        assert_eq!(expected, lines);
    }
    
    #[test]
    fn read_simple_toks() {
        let root = env!("CARGO_MANIFEST_DIR");
        let langs = Langs {source: String::from("en"),
                           target: String::from("th")};        
        let conf = Config {
            corpus_dir: format!("{}/data", root),
            tok_prefix: String::from("simple_toks"),
            langs: langs,
            alignment_file_path: String::from(""),
            orig_prefix: String::from(""),
            output_amphigram_path: String::from(""),
            textunit_limit: 100,
            textunit_offset: 0
        };
        let reader = Reader { config: conf };
        let toks = reader.read_toks(LangKey::SOURCE).unwrap();
        assert_eq!(vec![vec![String::from("AB"), String::from("CD"), String::from("EF")]], toks);
    }

    #[test]
    fn read_simple_links() {
        let root = env!("CARGO_MANIFEST_DIR");
        let langs = Langs {source: String::from("en"),
                           target: String::from("th")};        
        let conf = Config {
            corpus_dir: String::from(""), 
            tok_prefix: String::from(""),
            langs: langs,
            alignment_file_path: format!("{}/data/simple_align", root),
            orig_prefix: String::from(""),
            output_amphigram_path: String::from(""),
            textunit_limit: 100,
            textunit_offset: 0
        };
        let reader = Reader { config: conf };
        let links = reader.read_links().unwrap();
        let expected = vec![vec![Link {source: 1, target: 2}, Link {source: 5, target: 10}],
                            vec![Link {source: 70, target: 100}]];
        assert_eq!(links, expected);
    }
}
