
/// ```bash
/// $ cargo build --bins
/// $ RUST_LOG=debug maelstrom test -w g-counter --bin target/debug/g_counter --node-count 3 --rate 100 --time-limit 20 --nemesis partition
/// ````
use async_trait::async_trait;
use core::time;
use maelstrom::kv::{seq_kv, Storage, KV};
use maelstrom::protocol::Message;
use maelstrom::{done, Node, Result, Runtime};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::thread;
use tokio_context::context::Context;

pub(crate) fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let runtime = Runtime::new();
    let handler = Arc::new(handler(runtime.clone()));
    runtime.with_handler(handler).run().await
}

#[derive(Clone)]
struct Handler {
    s: Storage,
}

impl Handler {
    async fn confirmed_read(&self, key: String) -> i64 {
        let mut read_value = 0;
        let mut read_correct_value = false;
        while !read_correct_value {
            let (ctx, _handler) = Context::new();
            match self.s.get::<i64>(ctx, key.clone()).await {
                Ok(value) => {
                    let (ctx, _handler) = Context::new();
                    if self
                        .s
                        .cas(ctx, key.clone(), value, value, true)
                        .await
                        .is_ok()
                    {
                        read_value = value;
                        read_correct_value = true;
                    }
                }
                Err(_) => {
                    let (ctx, _handler) = Context::new();
                    if (self.s.cas(ctx, key.clone(), 0, 0, true).await).is_ok() {
                        read_value = 0
                    }
                }
            };
        }
        read_value
    }

    async fn write_and_confirm(&self, key: String, delta: i64) -> bool {
        if delta == 0 {
            return true;
        }
        let value = self.confirmed_read(key.clone()).await;
        let write_value = value + delta;
        let (ctx, _handler) = Context::new();
        if self
            .s
            .cas(ctx, key.clone(), value, write_value, true)
            .await
            .is_ok()
        {
            return true;
        };
        false
    }
}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        let key = String::from("grow_counter");
        let msg: Result<Request> = req.body.as_obj();
        match msg {
            Ok(Request::Init {
                node_id: _,
                node_ids: _,
            }) => {
                let s = self.clone();
                tokio::spawn(async move {
                    loop {
                        thread::sleep(time::Duration::from_millis(100));
                        s.clone().confirmed_read(key.clone()).await;
                    }
                });
                Ok(())
            }
            Ok(Request::Read) => {
                let value = self.confirmed_read(key.clone()).await;
                return runtime.reply(req, Response::ReadOk { value }).await;
            }
            Ok(Request::Add { delta }) => {
                while !self.write_and_confirm(key.clone(), delta).await {}
                return runtime.reply(req, Response::AddOk {}).await;
            }
            _ => done(runtime, req),
        }
    }
}

fn handler(runtime: Runtime) -> Handler {
    Handler { s: seq_kv(runtime) }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Request {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    Read,
    Add {
        delta: i64,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Response {
    ReadOk { value: i64 },
    AddOk {},
}
