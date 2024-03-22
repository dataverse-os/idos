use crate::PayloadHandler;
use dataverse_ceramic::{kubo, StreamOperator};
use dataverse_file_system as dfs;
use idos_types::Proof;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    kubo_path: String,
    dsn: String,
    queue: Option<QueueConfig>,
    cache_size: usize,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    pub dsn: String,
    pub pool: u32,
    pub worker: u32,
}

pub struct Client {
    cfg: Config,

    kubo: Arc<kubo::Client>,
    fs_client: Arc<dfs::file::Client>,
}

#[async_trait]
impl PayloadHandler for Client {
    async fn new_with_config(cfg: serde_json::Value) -> anyhow::Result<Self> {
        let cfg: Config = serde_json::from_value(cfg)?;
        let kubo: Arc<kubo::Client> = Arc::new(kubo::new(&cfg.kubo_path));
        let operator: Arc<dyn StreamOperator> = match &cfg.queue {
            Some(queue) => {
                // init kubo client for kubo task queue
                kubo::task::init_kubo(&cfg.kubo_path);
                let queue = Arc::new(Mutex::new(task_queue(&queue).await?));

                Arc::new(kubo::Cached::new(kubo.clone(), queue, cfg.cache_size)?)
            }
            None => kubo.clone(),
        };

        let pg_store = Arc::new(dataverse_pgsql_store::Client::new(operator, &cfg.dsn)?);
        let fs_client = Arc::new(dfs::file::Client::new(pg_store.clone(), pg_store));
        Ok(Client {
            cfg,
            kubo,
            fs_client,
        })
    }

    async fn handle_payload(&self, value: &mut serde_json::Value) -> anyhow::Result<Proof> {
        todo!()
    }
}

pub async fn task_queue(cfg: &QueueConfig) -> anyhow::Result<dfs::task::Queue> {
    let queue: dfs::task::Queue = dfs::task::new_queue(&cfg.dsn, cfg.pool).await?;
    let mut pool = dfs::task::build_pool(queue.clone(), cfg.worker);
    info!("starting queue");
    pool.start().await;
    return Ok(queue);
}
