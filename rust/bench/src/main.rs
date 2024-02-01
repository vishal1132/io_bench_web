use axum::{routing::get, Router, extract::State};
use redis::aio::MultiplexedConnection;
use std::sync::{Arc,Mutex};
use rand::Rng;
use tokio::time::{sleep,Duration};

#[derive(Clone)]
struct AppState {
    conn: MultiplexedConnection,
    count: Arc<Mutex<i32>>,
}

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();

    let con = client.get_multiplexed_tokio_connection().await.unwrap();
    let app_state = AppState {
        conn: con,
        count: Arc::new(Mutex::new(0))
    };
    let router= Router::new()
        .route("/data", get(data))
        .route("/bench", get(bench))
        .route("/test", get(test))
        .with_state(app_state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn bench(){
    sleep(Duration::from_millis(10)).await;
}

async fn test(State(state): State<AppState>) -> String {
    if rand::thread_rng().gen_range(1..=100)<6{
        {
            let mut counter=state.count.lock().unwrap();
            *counter+=1;
        }
        sleep(Duration::from_secs(3)).await;
    }
    let mut conn=state.conn.clone();
    let val:String=redis::cmd("GET").arg("test").query_async(&mut conn).await.unwrap();
    val
}


async fn data(State(state): State<AppState>)-> String{
    let counter=state.count.lock().unwrap();
    format!("expensive requests {}\n", *counter)
}
