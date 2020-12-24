#[macro_use]
extern crate serde_derive; 
use chrono::prelude::*;

#[derive(Serialize,Deserialize, Debug)]
struct CasesResponses (pub Vec<CasesResponse> );

#[derive(Serialize,Deserialize, Debug)]
struct CasesResponse {
    data: String,
    resultatcoviddescripcio: String,
}

// API call with no parameter (e.g. "https://analisi.transparenciacatalunya.cat/resource/")
impl restson::RestPath<()> for CasesResponses {
    fn get_path(_: ()) -> Result<String,restson::Error> { Ok(String::from("resource/jj6z-iyrp.json")) }
}


fn main() {
    println!("Hello, world!");

    let mut client = restson::RestClient::new("https://analisi.transparenciacatalunya.cat").unwrap();
    // Gets http://httpbin.org/anything/1234?a=2&b=abcd
    let query = vec![("MunicipiCodi","08260")];
    let data: CasesResponses = client.get_with((), &query).unwrap();

    //println!("RESPONSE: {:?}", data);
    println!("Read {} registers",data.0.len());

    //TODAY:
    let today = Local::now().date().pred();
    println!("The date is: {}", today.format("%F"));

    //iteration pior to filter
    let todayCases : Vec<CasesResponse> = data.0.into_iter()
                                                    .filter(|c| c.resultatcoviddescripcio.contains("Positiu"))
                                                    //.filter(|c| c.data.starts_with(&today.to_string()))
                                                    .collect();

    println!("filtered {} registers",todayCases.len());

}