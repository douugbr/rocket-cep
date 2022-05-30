#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize)]
struct Message {
    message: &'static str,
}

#[derive(Serialize, Deserialize)]
struct CEPInfo {
    cep: i32,
    address: String,
    neighbourhood: String,
    number_range: String,
    city_id: i32,
    state_id: i32,
}

#[get("/")]
fn index() -> Json<Message> {
    Json(Message {
        message: "Bem-vindo à Rocket CEP Api",
    })
}

#[get("/<cep>")]
fn cep(cep: String) -> Option<Json<CEPInfo>> {
    let file_path = "src/assets/cepsp.csv";
    let file = std::fs::File::open(file_path).expect("Could not open file");

    return match csv::Reader::from_reader(file)
        .deserialize::<CEPInfo>()
        .map(|x| {
            let desserialized = x.unwrap();
            desserialized
        })
        .find(|x| x.cep == cep.parse::<i32>().expect("Could not parse int"))
    {
        Some(cep_info) => Some(Json(cep_info)),
        None => None,
    };
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, cep])
}
