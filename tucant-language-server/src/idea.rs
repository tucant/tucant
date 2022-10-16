use std::{sync::Arc, collections::HashMap};

use rand::{thread_rng, distributions::Alphanumeric, Rng};
use tokio::{task::spawn_local, spawn, sync::{oneshot, mpsc}};

#[derive(Debug)]
pub struct ElephantRequest;

#[derive(Debug)]
pub struct ElephantResponse;

#[derive(Debug)]
pub struct CatNotification;

#[derive(Debug)]
pub struct DogRequest;

#[derive(Debug)]
pub struct DogResponse;

#[derive(Debug)]
pub enum Receiving {
    CatNotification(CatNotification),
    DogRequest(DogRequest),
}

#[derive(Debug)]
pub enum Sending {
    SendingElephantRequest((ElephantRequest, oneshot::Sender<ElephantResponse>)),
    SendingCatNotification(CatNotification), // cat has no response
    SendingDogRequest((DogRequest, oneshot::Sender<DogResponse>)),
}

#[derive(Debug)]
pub enum SendingReceiveEnd {
    SendingReceiveEndElephantRequest(oneshot::Sender<ElephantResponse>),
    SendingReceiveEndDogRequest(oneshot::Sender<DogResponse>),
}

// may store e.g. the open documents
#[derive(Clone)]
pub struct Handler {
    tx: mpsc::Sender<Sending>
}

fn handle_sending_generic<Response>(res: Response) -> (String, Response) {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    // TODO actually send
    println!("Handle sending end");
    (rand_string, res)
}

// concurrent by default
impl Handler {
    async fn handle_sending(mut rx: mpsc::Receiver<Sending>) -> (mpsc::Receiver<Sending>, Option<(String, SendingReceiveEnd)>) {
        println!("Handle sending start");
        match rx.recv().await {
            Some(Sending::SendingCatNotification(req)) => {
                // TODO FIXMe actually send
                (rx, None)
            },
            Some(Sending::SendingDogRequest((req, res))) => {
                (rx, Some(handle_sending_generic(SendingReceiveEnd::SendingReceiveEndDogRequest(res))))
            },
            Some(Sending::SendingElephantRequest((req, res))) => {
                (rx, Some(handle_sending_generic(SendingReceiveEnd::SendingReceiveEndElephantRequest(res))))
            },
            None => {
                // TODO FIXME
                println!("DONE!!!!");
                (rx, None)
            },
        }
    }

    async fn handle_receiving(self: Arc<Self>) -> (String, Receiving) {
        println!("handle receiving start");
        let request = self.clone().retrieve_next().await;
        let id = "1337".to_string();

        /*
        let cloned_self = self.clone();
        spawn(async move {
            cloned_self.handle(request).await;
        });
        */

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

        let mut pending_requests: HashMap<String, SendingReceiveEnd> = HashMap::new();

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

    pub async fn handle(&self, request: Receiving) {
        self.send_dog_request(DogRequest).await;

        //request.send_response(DogResponse);
        /*
        internally:
        self.tx.send((DogResponse, None)).await.unwrap();
        */

        todo!()
    }

    pub async fn send_dog_request(&self, request: DogRequest) -> DogResponse {
        // this will be implemented automatically
        let (tx, rx) = oneshot::channel();
        self.tx.send(Sending::SendingDogRequest((request, tx))).await.unwrap();
        rx.await.unwrap()
    }

    pub async fn retrieve_next(&self) -> Receiving {
        todo!()
    }
}