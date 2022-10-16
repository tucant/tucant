use std::{collections::HashMap, sync::Arc};

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tokio::sync::{mpsc, oneshot};

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

// may store e.g. the open documents
#[derive(Clone)]
pub struct Handler {
    tx: mpsc::Sender<(String, Option<oneshot::Sender<String>>)>,
}

// concurrent by default
impl Handler {
    async fn handle_sending(
        mut rx: mpsc::Receiver<(String, Option<oneshot::Sender<String>>)>,
    ) -> (
        mpsc::Receiver<(String, Option<oneshot::Sender<String>>)>,
        Option<(String, oneshot::Sender<String>)>,
    ) {
        println!("Handle sending start");
        match rx.recv().await {
            Some((request, None)) => {
                // TODO FIXMe actually send
                (rx, None)
            }
            Some((request, Some(response))) => {
                let rand_string: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(30)
                    .map(char::from)
                    .collect();
                // TODO actually send
                println!("Handle sending end");
                (rx, Some((rand_string, response)))
            }
            None => {
                // TODO FIXMe exit
                (rx, None)
            }
        }
    }

    async fn handle_receiving(self: Arc<Self>) -> (String, String) {
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
        // TODO FIXME send the json strings instead so we don't need so many enum stuff
        let (tx, rx) = mpsc::channel(3);

        let the_self = Self { tx };
        let self_arc = Arc::new(the_self);

        let mut pending_requests: HashMap<String, oneshot::Sender<String>> = HashMap::new();

        // I think this still does not work when they're cancelled?
        let receive_handler = self_arc.clone().handle_receiving();
        let send_handler = Handler::handle_sending(rx);
        tokio::pin!(receive_handler);
        tokio::pin!(send_handler);

        loop {
            tokio::select! {
                (_id, _request) = &mut receive_handler => {
                    //pending_requests.remove(&id).unwrap().send(request).unwrap();
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

    // TODO generate this for every request type
    pub async fn handle(&self, _request: String) {
        self.send_dog_request(DogRequest).await;

        //request.send_response(DogResponse);
        /*
        internally:
        self.tx.send((DogResponse, None)).await.unwrap();
        */

        todo!()
    }

    // TODO generate this for every request type
    pub async fn send_dog_request(&self, request: DogRequest) -> DogResponse {
        // this will be implemented automatically
        let (tx, rx) = oneshot::channel();
        //self.tx.send((request, tx)).await.unwrap();
        rx.await.unwrap()
    }

    pub async fn retrieve_next(&self) -> String {
        todo!()
    }
}
