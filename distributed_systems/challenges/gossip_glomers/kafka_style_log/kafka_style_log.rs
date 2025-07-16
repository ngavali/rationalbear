/// ```bash
/// $ cargo build --bins
/// $ RUST_LOG=debug ./maelstrom test -w kafka --bin ~/go/bin/maelstrom-kafka --node-count 1 --concurrency 2n --time-limit 20 --rate 1000
/// ````
use async_trait::async_trait;
use maelstrom::protocol::Message;
use maelstrom::{done, Node, Result, Runtime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let runtime = Runtime::new();
    let handler = Arc::new(handler(runtime.clone()));
    runtime.with_handler(handler).run().await
}

struct LogStorage {
    log: Vec<i64>,
    commit_offset: usize,
}

impl LogStorage {
    fn len(&self) -> usize {
        self.log.len()
    }

    fn append(&mut self, msg: i64) {
        self.log.push(msg);
    }

    fn get_commit_offset(&self) -> usize {
        self.commit_offset
    }

    fn set_commit_offset(&mut self, commit_offset: usize) {
        self.commit_offset = commit_offset;
    }

    fn get_log_from_offset(&self, from_offset: usize) -> Vec<(usize, i64)> {
        let mut log: Vec<(usize, i64)> = Vec::new();
        for i in from_offset..self.len() {
            log.push((i, self.log[i]));
        }
        log
    }
}

#[derive(Clone)]
struct Handler {
    s: Arc<Mutex<HashMap<String, LogStorage>>>,
}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        let message: Result<Request> = req.body.as_obj();
        match message {
            Ok(Request::Init {
                node_id: _,
                node_ids: _,
            }) => Ok(()),
            Ok(Request::Send { key, msg }) => {
                let mut offset = 0;
                let mut stored = false;
                if let Some(ls) = self.s.lock().unwrap().get_mut(&key) {
                    offset = ls.len();
                    ls.append(msg);
                    stored = true;
                }
                if !stored {
                    let ls = LogStorage {
                        log: vec![msg],
                        commit_offset: offset,
                    };
                    self.s.lock().unwrap().insert(key, ls);
                }
                runtime
                    .reply(req, Response::SendOk { offset })
                    .await
                    .unwrap();
                Ok(())
            }
            Ok(Request::Poll { offsets }) => {
                let mut hm: HashMap<String, Vec<(usize, i64)>> = HashMap::new();
                for offset in offsets {
                    if let Some(ls) = self.s.lock().unwrap().get_mut(&offset.0) {
                        hm.insert(offset.0, ls.get_log_from_offset(offset.1));
                    }
                }
                return runtime.reply(req, Response::PollOk { msgs: hm }).await;
            }
            Ok(Request::CommitOffsets { offsets }) => {
                for offset in offsets {
                    if let Some(ls) = self.s.lock().unwrap().get_mut(&offset.0) {
                        ls.set_commit_offset(offset.1);
                    }
                }
                return runtime.reply(req, Response::CommitOffsetsOk {}).await;
            }
            Ok(Request::ListCommittedOffsets { keys }) => {
                let mut commited_offsets: HashMap<String, usize> = HashMap::new();
                for key in keys {
                    if let Some(ls) = self.s.lock().unwrap().get_mut(&key) {
                        commited_offsets.insert(key, ls.get_commit_offset());
                    }
                }
                return runtime
                    .reply(
                        req,
                        Response::ListCommittedOffsetsOk {
                            offsets: commited_offsets,
                        },
                    )
                    .await;
            }
            _ => done(runtime, req),
        }
    }
}

fn handler(_: Runtime) -> Handler {
    Handler {
        s: Arc::new(Mutex::new(HashMap::new())),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Request {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    Send {
        key: String,
        msg: i64,
    },
    Poll {
        offsets: HashMap<String, usize>,
    },
    CommitOffsets {
        offsets: HashMap<String, usize>,
    },
    ListCommittedOffsets {
        keys: Vec<String>,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Response {
    SendOk {
        offset: usize,
    },
    PollOk {
        msgs: HashMap<String, Vec<(usize, i64)>>,
    },
    CommitOffsetsOk {},
    ListCommittedOffsetsOk {
        offsets: HashMap<String, usize>,
    },
}
