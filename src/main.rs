#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

mod auth;
mod models;
mod repositories;
mod schema;

use models::*;
use repositories::*;
use auth::BasicAuth;
use rocket::http::Status;
use rocket::fairing::AdHoc;
use rocket::response::status;
use rocket::serde::json::{Json, Value, json};
//use rusqlite::{Connection, params};

embed_migrations!();

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);


#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        RustaceanRepository::load_all(c)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[post("/rustaceans", format = "json", data="<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, conn: DbConn, new_rustacean: Json<NewRustacean>) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

//----------------------== modified section
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Item {
  row_id: i64,
  id: String,
  name: String,
  phones: Vec<i64>,
  data: Option<Vec<u8>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct NewItem {
  id: String,
  name: String,
  phones: Vec<i64>,
  data: Option<Vec<u8>>,
}

#[derive(serde::Serialize, Debug)]
struct ToDoList {
    items: Vec<Item>,
}

#[derive(serde::Serialize, Debug)]
struct StatusMessage {
    message: String,
}

#[get("/hello")]
fn hello() -> Value {
    json!({"msg":"Hello, world!"})
}//curl 127.0.0.1:8000/hello | jq

#[get("/items/<rid>")]
fn item_get(rid: i64) -> Value {
  let p1 = Item {
    row_id: rid,
    id: "A001".to_string(),
    name: "John Doe".to_string(),
    phones: vec![1111111111, 2222222222],
    data: None,
  };
  json!(p1)
  //json!({"rid":rid,"name":"A001"})
}//curl 127.0.0.1:8000/items/2 | jq

#[get("/items/<name>/<phone>")]
fn items_find(name: &str, phone: i64) -> Value {
  json!({"action":"find item", "name":name, "phone":phone})
  //format!("name: {}, phone: {}\n", name, phone)
}//curl 127.0.0.1:8000/items/A001/1112220001 | jq

#[get("/?items&name=A001")]
fn item_find2() -> Value {
  json!({"action":"find item2", "name":"A001"})
}/* The following GET requests match this endpoint
curl '127.0.0.1:8000/?items&name=A001' | jq
curl '127.0.0.1:8000/?name=A001&items' | jq
curl '127.0.0.1:8000/?size=11&items&there&name=A001' | jq
*/
#[get("/items")]
fn items_get() -> Value {
  json!([
    {"id":"A001","name":"A001"},
    {"id":"A002","name":"A002"},
    {"id":"A003","name":"A003"}])
}//curl 127.0.0.1:8000/items | jq


#[post("/items", format = "json", data="<item>")]
fn item_add(item: Json<NewItem>) -> Value {
  print!("data: {:?}", item);
  json!({"action":"add item-XPOST", "id":item.id, "name":item.name, "phones":item.phones, "data": item.data})
}/*
curl 127.0.0.1:8000/items -XPOST -H "Content-type: application/json" -d '{"id":"A004023","name":"A004","phones":[1112220004,1112220005]}' | jq
*/

#[post("/items_des", format = "json", data = "<item>")]
fn item_add_des(item: Json<NewItem>) -> Result<Json<StatusMessage>, String> {
    println!("/items_des item:{:?}", item);
    let results: Result<i32, String> = Ok(31);
    match results {
        Ok(rows_affected) => Ok(Json(
          StatusMessage {
            message: format!("{} rows inserted, id:{}, name:{}", rows_affected, item.id, item.name),
        })),
        Err(e) => Err(e),
    }
}/*
curl 127.0.0.1:8000/items_des -XPOST -H "Content-type: application/json" -d '{"id":"A004_des","name":"A004_des","phones":[1112220004,1112220005]}' | jq
*/

#[put("/items", format = "json", data="<item>")]
fn item_update(item: Json<NewItem>) -> Value {
  print!("data: {:?}", item);
  json!({"action":"update item-XPUT", "id":item.id, "name":item.name, "phones":item.phones, "data": item.data})
}/*
curl 127.0.0.1:8000/items -XPUT -H "Content-type: application/json" -d '{"id":"A004","name":"A014","phones":[1112220014,1112220015]}' | jq
*/


#[delete("/items/<_id>", format = "json")]
fn item_delete(_id: i32) -> status::NoContent {
  //use rocket::response::status;
  print!("_id: {:?}", _id);
  status::NoContent
}/* Do not add "| jq" because this endpoint does not return value!
curl 127.0.0.1:8000/items/1 -XDELETE -H "Content-type: application/json" -I
*/

// .register("/", catchers![not_found ])
#[catch(404)]
fn not_found() -> Value {
    json!({"msg":"Not found!"})
}//curl 127.0.0.1:8000/xyz | jq


//----------------------==

#[put("/rustaceans/<_id>", format = "json", data="<rustacean>")]
async fn update_rustacean(_id: i32, _auth: BasicAuth, conn: DbConn, rustacean: Json<Rustacean>) -> Result<Value, status::Custom<Value>> {
    conn.run(move |c| {
        RustaceanRepository::save(c, rustacean.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, 
              json!(e.to_string())))
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> Result<status::NoContent, status::Custom<Value>> {
    conn.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

async fn run_db_migrations(rocket: rocket::Rocket<rocket::Build>) -> Result<rocket::Rocket<rocket::Build>, rocket::Rocket<rocket::Build>> {
    DbConn::get_one(&rocket).await
        .expect("failed to retrieve database connection")
        .run(|c| match embedded_migrations::run(c) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                println!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        }).await
}

//------------------------==
#[rocket::main]
async fn main() {
  //let res = make_db();
  //print!("res: {:?}", res);

  let _ = rocket::build()
        .mount("/", routes![
            get_rustaceans,
            create_rustacean,
            view_rustacean,
            update_rustacean,
            delete_rustacean,
            hello, item_find2, items_find, items_get, item_get, item_add, item_update, item_delete, item_add_des
        ])
        .register("/", catchers![
            not_found
        ])
        .attach(DbConn::fairing())
        .attach(AdHoc::try_on_ignite("Database Migrations", run_db_migrations))
        .launch()
        .await;
}
/*#[launch]
fn rocket() -> _ {
  let res = func1();
  print!("res: {:?}", res);
  rocket::build().mount("/", routes![hello, hello2])
}
*/