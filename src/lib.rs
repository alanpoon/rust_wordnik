#[macro_use] extern crate serde_derive;
extern crate reqwest;
extern crate url;
use url::Url;
use reqwest::StatusCode;
#[derive(Deserialize, Debug,Clone)]
pub struct Definition {
    pub text_prons: Vec<String>,
    pub source_dictionary: String,
    pub example_uses:Vec<String>,
    pub related_words:Vec<String>,
    pub labels:Vec<String>,
   pub citations:Vec<Cit>,
   pub word:String,
   pub part_of_speech:String,
   pub attribution_text:String,
   pub sequence:String,
   pub seq_string:String,
   pub text:String,
   pub score:f32
}

#[derive(Deserialize,Debug,Clone)]
pub struct Cit {
   pub source: String,
   pub cite:String
}

pub fn get_definitions(word: &str,api:&str)
->Result<Vec<Definition>, reqwest::Error> 
{
       let url = format!("http://api.wordnik.com/v4/word.json/{}/definitions",  word);
       let mut url2 = Url::parse(&url).unwrap();
       url2.query_pairs_mut()
       .append_pair("api_key",api)
       .append_pair("sourceDictionaries","webster")
       .append_pair("includeRelated","false")
       .append_pair("useCanonical","false")
       .append_pair("includeTags","false")
       .append_pair("limit","1");
let client = reqwest::Client::new();
let mut res = client.get(&url2.into_string())
    .send().unwrap();
    match res.status() {
    StatusCode::OK => println!("success!"),
    StatusCode::PAYLOAD_TOO_LARGE => {
        println!("Request payload is too large!");
    }
    s => println!("Received response status: {:?}", s),
}

    let decoded:Vec<Definition> = res.json().unwrap();
  Ok(decoded)

}
