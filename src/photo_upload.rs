use self::rusqlite::params;
use rocket::data::{Data, ToByteUnit};
use rocket::fairing::AdHoc;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Build, Rocket};
use rocket_sync_db_pools::{database, rusqlite};
use std::path::Path;
use uuid::Uuid;

#[database("rusqlite")]
struct Db(rusqlite::Connection);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Post {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    title: String,
    text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Photo {
    uuid: Uuid,
    filename: String,
}

type Result<T, E = Debug<rusqlite::Error>> = std::result::Result<T, E>;
type UploadResult<T, E = Debug<std::io::Error>> = std::result::Result<T, E>;

#[post("/", data = "<post>")]
async fn create(db: Db, post: Json<Post>) -> Result<Created<Json<Post>>> {
    let item = post.clone();
    db.run(move |conn| {
        conn.execute(
            "INSERT INTO posts (title, text) VALUES (?1, ?2)",
            params![item.title, item.text],
        )
    })
    .await?;

    Ok(Created::new("/").body(post))
}

#[get("/")]
async fn list(db: Db) -> Result<Json<Vec<i64>>> {
    let ids = db
        .run(|conn| {
            conn.prepare("SELECT id FROM posts")?
                .query_map(params![], |row| row.get(0))?
                .collect::<Result<Vec<i64>, _>>()
        })
        .await?;

    Ok(Json(ids))
}

#[get("/<id>")]
async fn read(db: Db, id: i64) -> Option<Json<Post>> {
    let post = db
        .run(move |conn| {
            conn.query_row(
                "SELECT id, title, text FROM posts WHERE id = ?1",
                params![id],
                |r| {
                    Ok(Post {
                        id: Some(r.get(0)?),
                        title: r.get(1)?,
                        text: r.get(2)?,
                    })
                },
            )
        })
        .await
        .ok()?;

    Some(Json(post))
}

#[delete("/<id>")]
async fn delete(db: Db, id: i64) -> Result<Option<()>> {
    let affected = db
        .run(move |conn| conn.execute("DELETE FROM posts WHERE id = ?1", params![id]))
        .await?;

    Ok((affected == 1).then(|| ()))
}

#[delete("/")]
async fn destroy(db: Db) -> Result<()> {
    db.run(move |conn| conn.execute("DELETE FROM posts", params![]))
        .await?;

    Ok(())
}

async fn init_db(rocket: Rocket<Build>) -> Rocket<Build> {
    Db::get_one(&rocket)
        .await
        .expect("database mounted")
        .run(|conn| {
            conn.execute(
                r#"
                CREATE TABLE posts (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title VARCHAR NOT NULL,
                    text VARCHAR NOT NULL,
                    published BOOLEAN NOT NULL DEFAULT 0
                )"#,
                params![],
            )
        })
        .await
        .expect("can init rusqlite DB");

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("photo_upload stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .attach(AdHoc::on_ignite("init db", init_db))
            .mount(
                "/",
                routes![photo_upload, list, create, read, delete, destroy],
            )
    })
}

#[post("/img/upload/<filename>", data = "<file>")]
async fn photo_upload(filename: &str, file: Data<'_>) -> UploadResult<String> {
    let _uuid = Uuid::new_v4();
    file.open(100.megabytes())
        .into_file(Path::new("./uploads/img/").join(filename))
        .await?;
    Ok(format!("test"))
}
