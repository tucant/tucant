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

#[derive(Debug)]
pub enum Requests {
    ElephantRequest(ElephantRequest),
    CatRequest(CatRequest),
    DogRequest(DogRequest),
}

#[derive(Debug)]
pub enum Responses {
    ElephantResponse(ElephantResponse),
    CatResponse(CatResponse),
    DogResponse(DogResponse),
}

// concurrent by default
impl Handler {
    async fn handle_sending(mut rx: mpsc::Receiver<(CatRequest, Option<oneshot::Sender<CatResponse>>)>) -> (mpsc::Receiver<(CatRequest, Option<oneshot::Sender<CatResponse>>)>, Option<(String, oneshot::Sender<CatResponse>)>) {
        println!("Handle sending start");
        if let Some((req, res)) = rx.recv().await {
            if let Some(res) = res {
                let rand_string: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(30)
                    .map(char::from)
                    .collect();
                // TODO actually send
                println!("Handle sending end");
                (rx, Some((rand_string, res)))
            } else {
                // TODO actually send (also while sending we should be able to receive)
                println!("Handle sending end");
                (rx, None)
            }
        } else {
            // TODO FIXME
            println!("DONE!!!!");
            (rx, None)
        }
    }

    async fn handle_receiving(self: Arc<Self>) -> (String, Requests) {
        println!("handle receiving start");
        let request = self.clone().retrieve_next().await;
        let id = "1337".to_string();

        let cloned_self = self.clone();
        spawn(async move {
            cloned_self.handle(Requests::DogRequest(DogRequest)).await;
        });

        println!("handle receiving end");
        (id, request)
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

        let mut pending_requests: HashMap<String, oneshot::Sender<Requests>> = HashMap::new();

        // I think this still does not work when they're cancelled?
        let mut receive_handler = self_arc.clone().handle_receiving();
        let mut send_handler = Handler::handle_sending(rx);
        tokio::pin!(receive_handler);
        tokio::pin!(send_handler);

        loop {
            tokio::select! {
                (id, request) = &mut receive_handler => {
                    pending_requests.remove(&id).unwrap().send(request).unwrap();
                    receive_handler.set(self_arc.clone().handle_receiving());
                }
                (rx, hashmap_insert) = &mut send_handler => {
                    if let Some((id, res)) = hashmap_insert {
                        pending_requests.insert(id, res);
                    }
                    send_handler.set(Handler::handle_sending(rx));
                }
            }
        }
    }

    pub async fn handle(&self, request: Requests) {
        self.send_cat_request(CatRequest).await;

        //request.send_response(DogResponse);
        /*
        internally:
        self.tx.send((DogResponse, None)).await.unwrap();
        */

        todo!()
    }

    pub async fn send_cat_request(&self, request: CatRequest) -> CatResponse {
        // this will be implemented automatically
        let (tx, rx) = oneshot::channel::<CatResponse>();
        self.tx.send((CatRequest, Some(tx))).await.unwrap();
        rx.await.unwrap()
    }

    pub async fn retrieve_next(&self) -> Requests {
        todo!()
    }
}