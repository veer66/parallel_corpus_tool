use crate::bi_rtoks::BiRToks;
use crate::bi_text::BiText;
use crate::lang::LangKey;
use crate::link::Link;
use crate::reader::Reader;
use crate::rtoks_builder::RToksBuilder;
use std::error::Error;

#[derive(Debug)]
pub struct Textunit {
    pub bi_text: BiText,
    pub bi_rtoks: BiRToks,
    pub links: Vec<Link>,
}

impl Textunit {
    pub fn load(
        reader: &Reader,
        rtoks_builder: &RToksBuilder,
    ) -> Result<Vec<Textunit>, Box<Error>> {
        let links_list = reader.read_links()?;
        let source_toks_list = reader.read_toks(LangKey::SOURCE)?;
        let target_toks_list = reader.read_toks(LangKey::TARGET)?;
        let source_text_list = reader.read_lines(LangKey::SOURCE)?;
        let target_text_list = reader.read_lines(LangKey::TARGET)?;
        let mut textunits = vec![];
        for ((((links, source_toks), target_toks), source_text), target_text) in links_list
            .into_iter()
            .zip(source_toks_list.into_iter())
            .zip(target_toks_list.into_iter())
            .zip(source_text_list.into_iter())
            .zip(target_text_list.into_iter())
        {
            let source_rtoks = rtoks_builder.align_text_tok(&source_text, &source_toks)?;
            let target_rtoks = rtoks_builder.align_text_tok(&target_text, &target_toks)?;
            let bi_text = BiText {
                source: source_text,
                target: target_text,
            };
            let bi_rtoks = BiRToks {
                source: source_rtoks,
                target: target_rtoks,
            };
            let textunit = Textunit {
                bi_text: bi_text,
                bi_rtoks: bi_rtoks,
                links: links,
            };
            textunits.push(textunit);
        }
        Ok(textunits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::config::Langs;
    use crate::lang::LangKey;
    use crate::rtok::RTok;
    use crate::rtoks_builder::RToksBuilder;

    #[test]
    fn load_simple_textunits() {
        let root = env!("CARGO_MANIFEST_DIR");
        let langs = Langs {
            source: String::from("en"),
            target: String::from("th"),
        };
        let conf = Config {
            corpus_dir: format!("{}/data", root),
            tok_prefix: String::from("tu-toks"),
            langs: langs,
            alignment_file_path: format!("{}/data/tu-links", root),
            orig_prefix: String::from("tu-lines"),
            output_amphigram_path: String::from(""),
            textunit_limit: 100,
            textunit_offset: 0,
        };
        let reader = Reader { config: conf };
        let rtoks_builder = RToksBuilder::new();
        let textunits = Textunit::load(&reader, &rtoks_builder).unwrap();
        let rtok = RTok {
            s: 3,
            e: 5,
            text: String::from("ดำ"),
        };
        assert_eq!(textunits[0].bi_rtoks.target[1], rtok);
    }

}
