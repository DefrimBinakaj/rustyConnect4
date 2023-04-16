// https://www.freecodecamp.org/news/mongodb-in-rust/
// https://www.mongodb.com/try/download/community
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, unused_mut, unused_variables, unused_imports, dead_code, unused_parens)]
use mongodb::{Client, bson::*, options::{ClientOptions, ResolverConfig, FindOptions}};
use std::env;
use std::error::Error;
use tokio;
use tokio_stream::StreamExt;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde_json::{json, to_string};

async fn create_database(client: &Client, db_name: &str) -> Result<(), Box<dyn Error>> {
   let _db = client.database(db_name);
   println!("Database {} created", db_name);
   Ok(())
}

async fn create_collection(client: &Client, db_name: &str, coll_name: &str) -> Result<(), Box<dyn Error>> {
   let db = client.database(db_name);
   let coll_names = db.list_collection_names(None).await?;

   if coll_names.contains(&coll_name.to_string()) {
       println!("Collection {} already exists in {}", coll_name, db_name);
   } else {
       let _coll = db.create_collection(coll_name, None).await?;
       println!("Collection {} created in {}", coll_name, db_name);
   }

   Ok(())
}

async fn insert_document(client: &Client, db_name: &str, coll_name: &str, doc_name: &str, doc_password: &str) -> Result<(), Box<dyn Error>> {
   let db = client.database(db_name);
   let coll = db.collection(coll_name);

   let doc = doc! { 
      "name": doc_name, 
      "password": doc_password,
      "c4gamesplayed": 0,
      "c4gameswon": 0,
      "tootgamesplayed": 0,
      "tootgameswon": 0,
   };

   coll.insert_one(doc, None).await.unwrap();

   println!("Document inserted with \nname:{}, \npassword:{} \n", doc_name, doc_password);
   println!("");
   Ok(())

}

async fn delete_document(client: &Client, db_name: &str, coll_name: &str, doc_name: &str) -> Result<(), Box<dyn Error>> {
   let db = client.database(db_name);
   let coll = db.collection::<Document>(coll_name);
   let filter = doc! { 
      "name": bson::Bson::String(doc_name.to_string()) 
   };

   // to delete only one entry with a specific name:
   // coll.delete_one(filter, None).await?;

   // to delete all entries with a specific name:
   coll.delete_many(filter, None).await?;
   
   println!("Document deleted!");
   Ok(())
}















#[derive(serde::Deserialize)]
struct signInData {
   username: String,
   password: String,
}


async fn create_account(client: web::Data<Client>, signInData: web::Json<signInData>) -> impl Responder {
   println!("create account for: {}", signInData.username);
   let db = client.database("tempFromCompass");
   let coll = db.collection::<Document>("players");
   let filter = doc! {
      "name": signInData.username.clone(),
  };

  match coll.find_one(filter, None).await {
      Ok(existing_user) => {
          if existing_user.is_none() {
              let doc = doc! {
                  "name": signInData.username.clone(),
                  "password": signInData.password.clone(),
                  "c4gamesplayed": 0,
                  "c4gameswon": 0,
                  "tootgamesplayed": 0,
                  "tootgameswon": 0,
              };

              match coll.insert_one(doc, None).await {
                  Ok(_) => {
                      println!("Account created successfully");
                      HttpResponse::Ok().body("Account created successfully")
                  }
                  Err(e) => {
                      eprintln!("Error creating account: {}", e);
                      HttpResponse::InternalServerError().body("Error creating account")
                  }
              }
          } else {
              println!("Username already exists");
              HttpResponse::BadRequest().body("Username already exists")
          }
      }
      Err(e) => {
          eprintln!("Error checking for existing user: {}", e);
          HttpResponse::InternalServerError().body("Error checking for existing user")
      }
  }


}




async fn sign_into_game(client: web::Data<Client>, signInData: web::Json<signInData>) -> impl Responder {
   // println!("sign in triggered");
   let db = client.database("tempFromCompass");
   let coll = db.collection::<Document>("players");
   let filter = doc! {
       "name": bson::Bson::String(signInData.username.to_string()),
       "password": bson::Bson::String(signInData.password.to_string()),
   };
   let result = coll.find_one(filter, None).await;

   let mut isSigned = "nothing";

   if let Ok(Some(_)) = result {
      // signed in
      isSigned = "true";
   } else {
      isSigned = "false";
   }

   HttpResponse::Ok().json(isSigned)

}

















async fn top_n(client: web::Data<Client>) -> impl Responder  {
   let db = client.database("tempFromCompass");
   let coll = db.collection::<Document>("players");
   let mut orderedTopDocs = "".to_owned();

   orderedTopDocs.push_str("Connect4 champion players (top 5): \n");

   let filter = doc! {};
   let find_options = FindOptions::builder()
       .sort(doc! { "c4gameswon": -1, "c4gamesplayed": 1 })
       .limit(5)
       .build();

   let mut cursor = coll.find(filter, find_options).await.unwrap();
   let mut top_n_documents = Vec::new();

   while let Some(result) = cursor.next().await {
       match result {
           Ok(document) => top_n_documents.push(document),
        Err(_) => todo!(),
         
       }
   }



   for (index, document) in top_n_documents.iter().enumerate() {
      if let (Some(name), Some(games_played), Some(games_won)) = (
         document.get("name").and_then(bson::Bson::as_str),
         document.get("c4gamesplayed").and_then(bson::Bson::as_i32),
         document.get("c4gameswon").and_then(bson::Bson::as_i32),
      )
      {
         let winrate = (games_won as f64 / games_played as f64) * 100.0;
         if winrate > 0.0 {
            let strEntry = format!("{}) {}:  WINS-{} PLAYED-{}  WINRATE-{:.0}% \n ", index + 1, name, games_won, games_played, (games_won as f64 / games_played as f64) * 100.0 );
            orderedTopDocs.push_str(&strEntry);
         }
         else {
            let strEntry = format!("{}) {}:  WINS-{} PLAYED-{}  WINRATE-{:.0}% \n ", index + 1, name, games_won, games_played, 0.0 );
            orderedTopDocs.push_str(&strEntry);
         }
      }
   }



   orderedTopDocs.push_str("\nTOOT champion players (top 5): \n");

   let filter_toot = doc! {};
   let find_options_toot = FindOptions::builder()
       .sort(doc! { "tootgameswon": -1, "tootgamesplayed": 1 })
       .limit(5)
       .build();

   let mut cursor_toot = coll.find(filter_toot, find_options_toot).await.unwrap();
   let mut top_n_documents_toot = Vec::new();

   while let Some(result) = cursor_toot.next().await {
       match result {
           Ok(document) => top_n_documents_toot.push(document),
        Err(_) => todo!(),
         
       }
   }


   for (index, document) in top_n_documents_toot.iter().enumerate() {
      if let (Some(name), Some(games_played), Some(games_won)) = (
         document.get("name").and_then(bson::Bson::as_str),
         document.get("tootgamesplayed").and_then(bson::Bson::as_i32),
         document.get("tootgameswon").and_then(bson::Bson::as_i32),
      )
      {
         let winrate_toot = (games_won as f64 / games_played as f64) * 100.0;
         if winrate_toot > 0.0 {
            let strEntry = format!("{}) {}:  WINS-{} PLAYED-{}  WINRATE-{:.0}% \n ", index + 1, name, games_won, games_played, (games_won as f64 / games_played as f64) * 100.0 );
            orderedTopDocs.push_str(&strEntry);
         }
         else {
            let strEntry = format!("{}) {}:  WINS-{} PLAYED-{}  WINRATE-{:.0}% \n ", index + 1, name, games_won, games_played, 0.0 );
            orderedTopDocs.push_str(&strEntry);
         }
      }
   }



   HttpResponse::Ok().json(orderedTopDocs)

}




async fn win_c4(client: web::Data<Client>, name: web::Json<String>) -> impl Responder  {
   let db = client.database("tempFromCompass");
   let coll = db.collection::<Document>("players");
   let filter = doc! {"name": bson::Bson::String(name.to_string())};
   let update = doc! {
      "$inc": {
         "c4gamesplayed": 1,
         "c4gameswon": 1,
      },
   };

   
   let result = coll.update_many(filter, update, None).await;

   HttpResponse::Ok()
}

async fn lose_c4(client: web::Data<Client>, name: web::Json<String>) -> impl Responder {
   let db = client.database("tempFromCompass");
   let coll = db.collection::<Document>("players");
   let filter = doc! {"name": bson::Bson::String(name.to_string())};
   let update = doc! {
      "$inc": {
         "c4gamesplayed": 1,
      },
   };
   let result = coll.update_many(filter, update, None).await;

   HttpResponse::Ok()
}

async fn win_toot(client: web::Data<Client>, name: web::Json<String>) -> impl Responder  {
   let db = client.database("tempFromCompass");
   let coll = db.collection::<Document>("players");
   let filter = doc! {"name": bson::Bson::String(name.to_string())};
   let update = doc! {
      "$inc": {
         "tootgamesplayed": 1,
         "tootgameswon": 1,
      },
   };
   let result = coll.update_many(filter, update, None).await;

   HttpResponse::Ok()
}


async fn lose_toot(client: web::Data<Client>, name: web::Json<String>) -> impl Responder  {
   let db = client.database("tempFromCompass");
   let coll = db.collection::<Document>("players");
   let filter = doc! {"name": bson::Bson::String(name.to_string())};
   let update = doc! {
      "$inc": {
         "tootgamesplayed": 1,
      },
   };
   let result = coll.update_many(filter, update, None).await;

   HttpResponse::Ok()
}


















#[actix_web::main]
async fn main() -> std::io::Result<()> {
   println!("running server...");
   env::set_var("RUST_LOG", "actix_web=info");
   env::set_var("RUST_BACKTRACE", "1");

   let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
   let client = Client::with_options(client_options).unwrap();

   HttpServer::new(move || {
      let cors = Cors::permissive(); // You can further customize the CORS settings if needed

      App::new()
         .wrap(cors)
         .app_data(web::Data::new(client.clone()))
   .route("/data", web::get().to(top_n))
   .route("/winc4", web::post().to(win_c4))
   .route("/losec4", web::post().to(lose_c4))
   .route("/wintoot", web::post().to(win_toot))
   .route("/losetoot", web::post().to(lose_toot))
   .route("/signin", web::post().to(sign_into_game))
   .route("/signup", web::post().to(create_account))
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}




// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {

//    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
//    let client = Client::with_options(client_options).unwrap();

//    let db_name = "tempFromCompass";
//    let coll_name = "players";

//    // Print the databases in our MongoDB cluster:
//    println!("Databases:");
//    for name in client.list_database_names(None, None).await? {
//       println!("- {}", name);
//    }

   

//    loop {

//       println!("enter cmd: [done to exit]");
//       println!("1 - insert doc");
//       println!("2 - delete doc");
//       println!("3 - sign in");
//       println!("4 - leaderboards");

//       let mut cmdNum = String::new();
//       println!("choose command:");
//       let mut cmdNumWrap = std::io::stdin().read_line(&mut cmdNum).unwrap();
//       let mut cmdNumToString = cmdNum.trim();

//       if cmdNumToString == "1" {
//          println!("---");

//          let mut usernameInput = String::new();
//          println!("username:");
//          let mut usernameInputWrap = std::io::stdin().read_line(&mut usernameInput).unwrap();
//          let mut usernameInputTrim = usernameInput.trim();

//          let mut passwordInput = String::new();
//          println!("password:");
//          let mut passwordInputWrap = std::io::stdin().read_line(&mut passwordInput).unwrap();
//          let mut passwordInputTrim = passwordInput.trim();

//          insert_document(&client, db_name, coll_name, usernameInputTrim, passwordInputTrim).await?;
//          println!("---");
//       }
//       else if cmdNumToString == "2" {
//          println!("---");

//          let mut usernameInput = String::new();
//          println!("username to delete:");
//          let mut usernameInputWrap = std::io::stdin().read_line(&mut usernameInput).unwrap();
//          let mut usernameInputTrim = usernameInput.trim();

//          delete_document(&client, db_name, coll_name, usernameInputTrim).await?;
//          println!("---");
//       }
//       else if cmdNumToString == "3" {
//          println!("---");
//          println!("what is your:");

//          let mut usernameInput = String::new();
//          println!(" - username:");
//          let mut usernameInputWrap = std::io::stdin().read_line(&mut usernameInput).unwrap();
//          let mut usernameInputTrim = usernameInput.trim();

//          let mut passwordInput = String::new();
//          println!(" - password:");
//          let mut passwordInputWrap = std::io::stdin().read_line(&mut passwordInput).unwrap();
//          let mut passwordInputTrim = passwordInput.trim();

//          sign_in(&client, db_name, coll_name, usernameInputTrim, passwordInputTrim).await?;
//          println!("---");
//       }
//       else if cmdNumToString == "4" {
//          println!("---");
//          top_n_by_c4_games_played(&client, db_name, coll_name, 4).await?;
//          println!("---");
//          top_n_by_toot_games_played(&client, db_name, coll_name, 3).await?;
//          println!("---");
//       }
//       else if cmdNumToString == "done" {
//          break;
//       }
//       else {
//          println!("---");
//          println!("invalid command");
//          println!("---");
//       }

//    }





//    // create_database(&client, "newDatabase").await?;
//    // create_collection(&client, db_name, "alluserstemp").await?;
   
   

//    Ok(())

// }