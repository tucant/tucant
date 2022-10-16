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
    pub async fn new() -> Self {
        let (tx, mut rx) = mpsc::channel(3);

        let the_self = Self {
            tx
        };

        let mut pending_requests: HashMap<String, oneshot::Sender<CatResponse>> = HashMap::new();

        while let Some((req, res)) = rx.recv().await {
            if let Some(res) = res {
                let rand_string: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(30)
                    .map(char::from)
                    .collect();
                pending_requests.insert(rand_string, res);
            } else {

            }
        }

        loop {
            let request = the_self.clone().retrieve_next().await;
            let id = "1337".to_string();

            let cloned_self = the_self.clone();
            spawn(async move {
                cloned_self.handle_dog(DogRequest).await;
            });

            pending_requests.remove(&id).unwrap().send(request).unwrap();
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
        self.tx.send((CatRequest, tx)).await.unwrap();
        rx.await.unwrap()
    }

    pub async fn retrieve_next(&self) -> CatResponse {
        todo!()
    }
}