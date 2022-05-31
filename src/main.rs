#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};
const CSV_PATH: &str = "src/assets/cepsp.csv";

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
    let file = std::fs::File::open(CSV_PATH).expect("Could not open CEP .csv db");

    return match csv::Reader::from_reader(file)
        .deserialize::<CEPInfo>()
        .map(|x| x.expect("Could not read db line"))
        .find(|x| x.cep == cep.parse::<i32>().expect("Could not parse CEP to i32"))
    {
        Some(cep_info) => Some(Json(cep_info)),
        None => None,
    };
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, cep])
}
