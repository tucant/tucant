use std::{sync::Arc, collections::HashMap};

use rand::{thread_rng, distributions::Alphanumeric, Rng};
use tokio::{task::spawn_local, spawn, sync::{oneshot, mpsc}};

#[derive(Debug)]
pub struct ElephantRequest;

#[derive(Debug)]
pub struct ElephantResponse;

#[derive(Debug)]
pub struct CatRequest;

#[derive(Debug)]
pub struct CatResponse;

#[derive(Debug)]
pub struct DogRequest;

#[derive(Debug)]
pub struct DogResponse;

// may store e.g. the open documents
#[derive(Clone)]
pub struct Handler {
    tx: mpsc::Sender<(CatRequest, Option<oneshot::Sender<CatResponse>>)>
}

// concurrent by default
impl Handler {
    async fn handle_sending(mut rx: mpsc::Receiver<(CatRequest, Option<oneshot::Sender<CatResponse>>)>) -> mpsc::Receiver<(CatRequest, Option<oneshot::Sender<CatResponse>>)> {
        println!("Handle sending start");
        if let Some((req, res)) = rx.recv().await {
            if let Some(res) = res {
                let rand_string: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(30)
                    .map(char::from)
                    .collect();
                //pending_requests.insert(rand_string, res);

                // TODO actually send
            } else {
                // TODO actually send (also while sending we should be able to receive)
            }
        }
        println!("Handle sending end");
        rx
    }

    async fn handle_receiving(self: Arc<Self>) {
        println!("handle receiving start");
        let request = self.clone().retrieve_next().await;
        let id = "1337".to_string();

        let cloned_self = self.clone();
        spawn(async move {
            cloned_self.handle_dog(DogRequest).await;
        });

        //pending_requests.remove(&id).unwrap().send(request).unwrap();
        println!("handle receiving end");
    }

    // https://smallcultfollowing.com/babysteps/blog/2022/06/13/async-cancellation-a-case-study-of-pub-sub-in-mini-redis/
    // "For example, I think it’d be cool if we could share one &mut across two async fn that are running concurrently, so long as that &mut is not borrowed across an await point. I have thoughts on that but…not for this post."
    // https://blog.yoshuawuyts.com/async-cancellation-1/
    // https://blog.yoshuawuyts.com/futures-concurrency-2/

    // https://tokio.rs/tokio/tutorial/select
    // se there section Loops -> "Resuming an async operation" and "Modifying a branch"
    pub async fn run() {
        let (tx, mut rx) = mpsc::channel(3);

        let the_self = Self {
            tx
        };
        let self_arc = Arc::new(the_self);

        let mut pending_requests: HashMap<String, oneshot::Sender<CatResponse>> = HashMap::new();

        // I think this still does not work when they're cancelled?
        let mut receive_handler = self_arc.clone().handle_receiving();
        let mut send_handler = Handler::handle_sending(rx);
        tokio::pin!(receive_handler);
        tokio::pin!(send_handler);

        loop {
            tokio::select! {
                res = &mut receive_handler => {
                    receive_handler.set(self_arc.clone().handle_receiving());
                }
                rx = &mut send_handler => {
                    send_handler.set(Handler::handle_sending(rx));
                }
            }
        }
    }

    pub async fn handle_elephant(&self, request: ElephantRequest) -> ElephantResponse {
        todo!()
    }

    pub async fn handle_dog(&self, request: DogRequest) -> DogResponse {
        self.send_cat_request(CatRequest).await;
        todo!()
    }

    pub async fn send_cat_request(&self, request: CatRequest) -> CatResponse {
        // this will be implemented automatically
        let (tx, rx) = oneshot::channel::<CatResponse>();
        self.tx.send((CatRequest, Some(tx))).await.unwrap();
        rx.await.unwrap()
    }

    pub async fn retrieve_next(&self) -> CatResponse {
        todo!()
    }
}