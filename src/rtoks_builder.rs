use crate::rtok::RTok;
use crate::utf16::{substring, utf16_len};

trait StrMod {
    fn mod_orig(&self, s: &str) -> String {
        s.to_string()
    }
    fn mod_tok(&self, s: &str) -> String {
        s.to_string()
    }
}

struct IdentityStrMod;

impl StrMod for IdentityStrMod {
    fn mod_tok(&self, s: &str) -> String {
        s.replace("&apos;", "'")
            .replace("&quot;", "\"")
            .replace("&amp;", "&")
            .replace("&#91;", "[")
            .replace("&#93;", "]")
            .to_string()
    }

    fn mod_orig(&self, s: &str) -> String {
        s.replace("\u{F112}", " ").to_string()
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum CharAlignError {
        CannotMatchSomeToks(text: String, toks: Vec<String>, i: usize, s: usize, orig_len: usize) {
            display("Cannon match: text={} toks={:?} i={} s={} orig_len={}", &text, &toks, i, s, orig_len)
        }
    }
}

pub struct RToksBuilder {
    str_mods: Vec<Box<StrMod>>,
}

impl RToksBuilder {
    pub fn new() -> RToksBuilder {
        RToksBuilder {
            str_mods: vec![Box::new(IdentityStrMod {}), Box::new(IdentityStrMod {})],
        }
    }

    fn match_tok_internal<T: ?Sized + StrMod>(
        s: usize,
        orig: &str,
        tok: &str,
        str_mod: &Box<T>,
    ) -> Option<String> {
        let tok = str_mod.mod_tok(tok);
        let e = s + utf16_len(&tok);
        if e > utf16_len(orig) {
            return None;
        }
        let prefix = substring(orig, s, e).unwrap();
        let mod_prefix = str_mod.mod_orig(&prefix);
        if mod_prefix.eq_ignore_ascii_case(&tok) {
            Some(prefix)
        } else {
            None
        }
    }

    fn match_tok(&self, orig: &str, tok: &str, s: usize) -> Option<String> {
        for mod_str in &self.str_mods {
            let prefix = Self::match_tok_internal(s, orig, tok, &mod_str);
            if prefix.is_some() {
                return prefix;
            }
        }
        None
    }

    pub fn align_text_toks(
        &self,
        orig: &str,
        toks: &[String],
    ) -> Result<Vec<RTok>, CharAlignError> {
        let mut s = 0;
        let mut i = 0;
        let mut aligned_toks = vec![];
        let orig_len = utf16_len(orig);
        loop {
            if i == toks.len() {
                return Ok(aligned_toks);
            }
            if s >= orig_len {
                return Err(CharAlignError::CannotMatchSomeToks(
                    String::from(orig),
                    toks.to_vec(),
                    i,
                    s,
                    orig_len,
                ));
            }
            let tok = &toks[i];
            if let Some(prefix) = self.match_tok(orig, tok, s) {
                let e = s + utf16_len(&prefix[..]);
                let aligned_tok = RTok {
                    s: s,
                    e: e,
                    text: prefix,
                };
                s = e;
                i += 1;
                aligned_toks.push(aligned_tok);
            } else {
                s += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rtok::RTok;

    #[test]
    fn align_text_toks_simple() {
        let builder = RToksBuilder::new();
        let text = "AB C";
        let toks = vec![String::from("AB"), String::from(" "), String::from("C")];
        let rtoks = builder.align_text_toks(text, &toks).unwrap();
        let expected = vec![
            RTok {
                text: String::from("AB"),
                s: 0,
                e: 2,
            },
            RTok {
                text: String::from(" "),
                s: 2,
                e: 3,
            },
            RTok {
                text: String::from("C"),
                s: 3,
                e: 4,
            },
        ];
        assert_eq!(rtoks, expected);
    }

    #[test]
    fn align_text_toks_f112() {
        let builder = RToksBuilder::new();
        let text = "AB\u{F112}C";
        let toks = vec![String::from("AB"), String::from(" "), String::from("C")];
        let rtoks = builder.align_text_toks(text, &toks).unwrap();
        let expected = vec![
            RTok {
                text: String::from("AB"),
                s: 0,
                e: 2,
            },
            RTok {
                text: String::from("\u{F112}"),
                s: 2,
                e: 3,
            },
            RTok {
                text: String::from("C"),
                s: 3,
                e: 4,
            },
        ];
        assert_eq!(rtoks, expected);
    }
}
