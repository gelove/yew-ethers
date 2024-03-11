use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::{future, pin_mut};
use futures::{sink::SinkExt, stream::StreamExt};
use log::info;
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::{Message, WebSocketError};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

#[derive(Clone)]
pub struct Context {
    sender: UnboundedSender<Message>,
    receiver: Rc<RefCell<UnboundedReceiver<Result<Message, WebSocketError>>>>,
}

impl PartialEq for Context {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[function_component(Consumer)]
fn consumer() -> Html {
    let ctx: Context = use_context().unwrap_throw();
    {
        use_effect_with_deps(
            move |_| {
                let mut tx = ctx.sender;
                let rx = ctx.receiver;

                wasm_bindgen_futures::spawn_local(async move {
                    tx.send(Message::Text("message".to_string()))
                        .await
                        .unwrap_throw();
                });

                wasm_bindgen_futures::spawn_local(async move {
                    while let Some(m) = rx.borrow_mut().deref_mut().next().await {
                        info!("{:?}", m);
                    }
                });

                || ()
            },
            (),
        )
    }
    html! {"hello world"}
}

#[function_component(Listener)]
fn listener() -> Html {
    let (mut read_tx, read_rx) = futures::channel::mpsc::unbounded();
    let (write_tx, write_rx) = futures::channel::mpsc::unbounded();

    let context = Context {
        sender: write_tx,
        receiver: Rc::new(RefCell::new(read_rx)),
    };

    {
        use_effect_with_deps(
            move |_| {
                let ws = WebSocket::open("ws://localhost:8080/api/ws").unwrap_throw();
                let (write, mut read) = ws.split();

                let fwd_writes = write_rx.map(Ok).forward(write);

                let fwd_reads = async move {
                    while let Some(m) = read.next().await {
                        read_tx.send(m).await.unwrap_throw()
                    }
                };

                wasm_bindgen_futures::spawn_local(async move {
                    pin_mut!(fwd_writes, fwd_reads);
                    future::select(fwd_writes, fwd_reads).await;
                });

                || {}
            },
            (),
        );
    }

    html! {
        <ContextProvider<Context> {context}>
            <Consumer />
        </ContextProvider<Context>>
    }
}
