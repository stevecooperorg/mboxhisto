use anyhow::Result;
use clap::Parser;
use mailbox::stream::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
struct Args {
    #[clap(short = 'i', long = "input-file")]
    input_file: std::path::PathBuf,
    #[clap(short = 'o', long = "output-file")]
    output_file: std::path::PathBuf,
}

fn main() -> Result<()> {
    let Args {
        input_file,
        output_file,
    } = Args::parse();
    eprintln!("{}", env!("CARGO_BIN_NAME"));
    eprintln!("reading mbox file '{}'", input_file.display());

    let file = File::open(&input_file).unwrap();

    let mut histo = HashMap::new();

    for entry in mailbox::stream::entries(file).flatten() {
        match entry {
            Entry::Header(h) if &h.key().to_string() == "From" => {
                let value = h.value().owner().to_string();
                let email = extract_email_from_header(&value);
                histo.entry(email).and_modify(|e| *e += 1).or_insert(1);
            }
            _ => {}
        }
    }

    let mut file = File::create(output_file).expect("Unable to create file");

    writeln!(file, "count, email").expect("Unable to write to file");
    for (k, v) in histo.iter() {
        writeln!(file, "{},\"{}\"", v, k.replace('"', "\\\"")).expect("Unable to write to file");
    }

    Ok(())
}

fn extract_email_from_header(input: &str) -> String {
    // write regex to extract email from, eg, "j <a.b@foo.com>"
    let rx = regex::Regex::new(r#"<([^>]+)>"#).unwrap();
    if let Some(caps) = rx.captures(input) {
        caps.get(1).unwrap().as_str().to_string()
    } else {
        input.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let actual = extract_email_from_header("j <a.b@foo.com>");
        let expected = "a.b@foo.com";
        assert_eq!(actual, expected);

        let actual = extract_email_from_header("a.b@foo.com");
        let expected = "a.b@foo.com";
        assert_eq!(actual, expected);
    }
}
