#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate url;
extern crate serialize;
extern crate getopts;

use getopts::{reqopt,getopts};
use regex::Regex;
use url::Url;
use serialize::json;
use serialize::json::ToJson;
use std::fmt::Show;
use std::fmt;
use std::os;
use std::io::fs::File;
use std::path::posix::Path;
use std::io::IoResult;
use std::io::InvalidInput;
use std::collections::TreeMap;

enum RewriteFlag {
  Redirect301,
  Redirect302,
  Last,
  Fail,
  QueryStringAppend
}

impl Show for RewriteFlag {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Redirect301 => write!(f, "R=301"),
      &Redirect302 => write!(f, "R=302"),
      &Last => write!(f, "L"),
      &Fail => write!(f, "F"),
      &QueryStringAppend => write!(f, "QSA")
    }
  }
}

impl ToJson for RewriteFlag {
  fn to_json(&self) -> json::Json {
    json::String(match self {
      &Redirect301 => "R=301".to_string(),
      &Redirect302 => "R=302".to_string(),
      &Last => "L".to_string(),
      &Fail => "F".to_string(),
      &QueryStringAppend => "QSA".to_string()
    })
  }
}

#[deriving(Show)]
struct RewriteRule {
  domain: String,
  pattern: Regex,
  flags: Vec<RewriteFlag>,
  dest: Url
}

impl ToJson for RewriteRule {
  fn to_json(&self) -> json::Json {
    let mut d = TreeMap::new();
    d.insert("domain".to_string(), self.domain.to_json());
    d.insert("pattern".to_string(), self.pattern.as_str().to_string().to_json());
    d.insert("flags".to_string(), self.flags.to_json());
    d.insert("dest".to_string(), self.dest.serialize().to_json());
    json::Object(d)
  }
}

#[deriving(Show)]
enum ParseError {
  BadFlag,
  InvalidCondition,
  InvalidDestination,
  MissingDestination,
  MissingPattern,
  UnexpectedWhiteSpace
}

struct Context {
  output: String,
  input: String,
  domain: String
}

fn validate_input(args:Vec<String>) -> IoResult<Context> {
  let opts = [
    reqopt("o", "", "output file", "OUTPUT"),
    reqopt("i", "", "input file", "INPUT"),
    reqopt("d", "domain", "domain name rewrites are valid within", "DOMAIN")
  ];
  let matches = match getopts(args.tail(), opts) {
    Ok(m) => m,
    Err(f) => {
      println!("{}", f)
      return Err(std::io::IoError {
        kind: InvalidInput,
        desc: "options were not correctly passed to program",
        detail: None
      });
    }
  };

  Ok(Context {
    output: matches.opt_str("o").unwrap(),
    input: matches.opt_str("i").unwrap(),
    domain: matches.opt_str("domain").unwrap()
  })
}

fn parse_flags(rawflags: String) -> Result<Vec<RewriteFlag>, ParseError> {
  let re = regex!(r"[^],\[]+");
  let mut flags: Vec<RewriteFlag> = vec!();
  for flag in re.captures_iter(rawflags.as_slice()) {
    flags.push(match flag.at(0) {
      "R=301" => Redirect301,
      "R=302" => Redirect302,
      "L" => Last,
      "QSA" => QueryStringAppend,
      _ => return Err(BadFlag)
    });
  }
  Ok(flags)
}


// TODO: Break this out into a separate module/file
fn parse_rewrite_rules(domain: String, file: String) -> Result<(Vec<RewriteRule>, uint), ParseError> {
  let contents = File::open(&Path::new(file.as_slice())).read_to_string().unwrap();
  let lines: Vec<&str> = contents.as_slice().lines_any().collect();
  let mut in_condition = false;
  let whitespace  = regex!(r"[ \t]+");
  let mut rules: Vec<RewriteRule> = Vec::with_capacity(lines.len());
  let mut l = 1u;
  let mut skipped = 0u;

  for line in lines.iter() {
    let parts: Vec<&str> = whitespace.split(*line).collect();
    if parts.len() > 0 {
      match parts[0] {
        "RewriteRule" => {
          if in_condition {
            in_condition = false;
            skipped = skipped + 1;
          } else {
            // TODO: Make this less icky...
            match parts.len() {
              1 => return Err(MissingPattern),
              2 => return Err(MissingDestination),
              3 => {
                rules.push(RewriteRule {
                  domain: domain.clone(),
                  pattern: Regex::new(parts[1]).unwrap(),
                  flags: vec!(),
                  dest: Url::parse(parts[2]).ok().expect(format!("Invalid destination url on line {}", l).as_slice())
                })
              },
              4 => {
                rules.push(RewriteRule {
                  domain: domain.clone(),
                  pattern: Regex::new(parts[1]).unwrap(),
                  flags: try!(parse_flags(parts[3].to_string())),
                  dest: Url::parse(parts[2]).ok().expect(format!("Invalid destination url on line {}", l).as_slice())
                })
              },
              _ => return Err(UnexpectedWhiteSpace)
            };
          }
        },
        "RewriteCond" => {
          in_condition = true;
        },
        _ => ()
      }
    }
    l = l + 1;
  }

  Ok((rules, skipped))
}

fn write_data(data: &Vec<RewriteRule>, output: String) -> IoResult<()> {
  let json_obj: json::Json = data.to_json();
  let json_str: String = json_obj.to_string();
  File::create(&Path::new(output)).write_str(json_str.as_slice())
}

fn main() {
  match validate_input(os::args()) {
    Err(e) => println!("Invalid input: {}", e),
    Ok(context) => {
      match parse_rewrite_rules(context.domain, context.input) {
        Err(e) => println!("Error parsing htaccess: {}", e),
        Ok((ref rules, skipped)) => {
          match write_data(rules, context.output) {
            Err(e) => println!("Unable to write data: {}", e),
            Ok(_) => println!("Saved to disk. {} rules captured, {} rules skipped", rules.len(), skipped)
          };
        }
      }
    }
  };
}

// TODO: Add Tests
