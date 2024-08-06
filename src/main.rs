use std::net::SocketAddr;
use sqlx::{FromRow, query, query_as};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod api;
mod model; 
mod service;
mod util;

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
// #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
// #[sea_orm(table_name = "user")]
pub struct User {
    // #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
}
//
// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {}
//
// impl ActiveModelBehavior for ActiveModel {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not found");

    // let mut opt = ConnectOptions::new(database_url);
    // opt.max_connections(10);
    //
    // let db = Database::connect(opt).await.unwrap();
    //
    // let vec = User::find().all(&db).await.unwrap();

    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let rows = query("SELECT * FROM public.user").fetch_all(&pool).await.unwrap();
    for row in rows {
        println!("{:?}", row);
    }

    let users = query_as::<_, User>("SELECT * FROM public.user").fetch_all(&pool).await.unwrap();
    for user in users {
        println!("{:?}", user);
    }

    let records = query!("SELECT * FROM public.user").fetch_all(&pool).await.unwrap();
    for record in records {
        println!("{:?}", record);
    }

    let users2 = query_as!(User, "SELECT id, name FROM public.user").fetch_all(&pool).await.unwrap();
    for user in users2 {
        println!("{:?}", user);
    }

    let app = api::load_routers();

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
