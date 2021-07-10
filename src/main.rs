#[macro_use]
extern crate clap;
use clap::App;
use clap::Arg;

extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Attr, Name};

struct Bing {
    key: String,
    // doc: Document,
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    // Learn from here: https://stackoverflow.com/questions/55345730/how-can-i-prevent-the-last-argument-from-needing-to-be-quoted-with-clap
    let matches = App::from_yaml(yaml)
        .setting(clap::AppSettings::TrailingVarArg)
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .arg(
            Arg::with_name("key")
                .help("The key word to look up")
                .required(true)
                .takes_value(true)
                .multiple(false),
        )
        .get_matches();

    let keys: Vec<&str> = matches.values_of("key").unwrap().collect();
    let bing = Bing::with_key(keys[0]);
    bing.lookup();
}

impl Bing {
    fn with_key(name: &str) -> Bing {
        Bing {
            key: name.to_string(),
        }
    }

    fn parse_voice(&self, doc: &Document) {
        let mut pr_us: String = "".to_string();
        let mut pr_uk: String = "".to_string();
        for node in doc.find(Attr("class", "hd_prUS b_primtxt")) {
            pr_us = node.text();
        }
        for node in doc.find(Attr("class", "hd_pr b_primtxt")) {
            pr_uk = node.text();
        }
        println!("{} {}", pr_us, pr_uk);
    }

    fn parse_meaning(&self, doc: &Document) {
        for node1 in doc.find(Attr("class", "lf_area")) {
            for node2 in node1.find(Attr("class", "qdef")) {
                for node3 in node2.find(Name("ul")) {
                    println!("{}", node3.text());
                }
            }
        }
    }
    fn parse_var(&self, doc: &Document) {
        for node1 in doc.find(Attr("class", "lf_area")) {
            for node2 in node1.find(Attr("class", "qdef")) {
                for node3 in node2.find(Attr("class", "hd_if")) {
                    println!("{}", node3.text());
                }
            }
        }
    }

    fn parse_ec(&self, doc: &Document) {
        println!("E-C:");
        for node1 in doc.find(Attr("class", "lf_area")) {
            for node2 in node1.find(Attr("class", "qdef")) {
                for node3 in node2.find(Attr("id", "crossid")) {
                    for node4 in node3.find(Name("tr")) {
                        for node5 in node4.find(Attr("class", "pos pos1")) {
                            println!("{}", node5.text());
                        }
                        for node5 in node4.find(Attr("class", "de_li1 de_li3")) {
                            println!("{}", node5.text());
                        }
                    }
                }
            }
        }
    }

    fn parse_ee(&self, doc: &Document) {
        println!("E-E:");
        for node1 in doc.find(Attr("class", "lf_area")) {
            for node2 in node1.find(Attr("class", "qdef")) {
                for node3 in node2.find(Attr("id", "homoid")) {
                    for node4 in node3.find(Name("tr")) {
                        for node5 in node4.find(Attr("class", "pos pos1")) {
                            println!("{}", node5.text());
                        }
                        for node5 in node4.find(Attr("class", "de_li1 de_li3")) {
                            println!("{}", node5.text());
                        }
                    }
                }
            }
        }
    }

    fn lookup(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "http://cn.bing.com/dict/search?q={}&FORM=BDVSP6&mkt=zh-cn",
            self.key
        );

        let text = reqwest::blocking::get(url)?.text()?;
        let doc = Document::from(text.as_str());
        self.parse_voice(&doc);
        println!();
        self.parse_meaning(&doc);
        println!();
        self.parse_var(&doc);
        println!();
        self.parse_ec(&doc);
        println!();
        self.parse_ee(&doc);
        Ok(())
    }
}
