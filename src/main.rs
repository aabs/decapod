use std::fmt::Display;
use url::Url;

fn main() {
    let mut t = Triple {
        subject: Resource::Iri(Url::parse("urn:12453").expect("invalid subject")),
        object: Resource::Iri(Url::parse("urn:75127").expect("invalid subject")),
        predicate: Resource::Iri(Url::parse("urn:afshafsha").expect("invalid subject")),
    };
    println!("{} .",t);

    t = Triple{
        subject: Resource::Blank("a".to_string()),
        predicate: Resource::Iri(Url::parse("urn:afshafsha").expect("invalid subject")),
        object: Resource::Literal("hello world".to_string()),
    };
    println!("{} .",t);

}

#[derive(Debug)]
pub enum Resource {
    Iri(Url),
    Blank(String),
    Literal(String),
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resource::Iri(u) => write!(f, "<{}>", u),
            Resource::Blank(n) => write!(f, "_:{}", n),
            Resource::Literal(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[derive(Debug)]
pub struct Triple {
    subject: Resource,
    predicate: Resource,
    object: Resource,
}

impl Display for Triple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.subject, self.predicate, self.object)
    }
}
