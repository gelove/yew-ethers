use defi_wallet_connect::session::SessionInfo;
use defi_wallet_connect::{Client, Metadata, WCMiddleware};
use defi_wallet_connect::{ClientChannelMessage, ClientChannelMessageType};
use ethers::prelude::Address;
use ethers::prelude::Middleware;
use ethers::types::H160;
use eyre::eyre;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::service::wallet;

/// remove session.json to start new session
const G_FILENAME: &str = "session.json";

///  temporary session is stored to session.json
async fn make_client() -> eyre::Result<Client> {
    log::debug!("make_client {}", G_FILENAME);
    let filename = G_FILENAME;
    if let Ok(mut file) = File::open(filename) {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let session: SessionInfo = serde_json::from_str(&contents)?;
        let client = Client::restore(session).await?;
        log::debug!("restored client");
        Ok(client)
    } else {
        let client = Client::new(
            Metadata {
                description: "Defi WalletConnect example.".into(),
                url: "http://localhost:8080/".parse().expect("url"),
                icons: vec![],
                name: "Defi WalletConnect Web3 Example".into(),
            },
            None,
        )
        .await?;
        log::debug!("created client");
        Ok(client)
    }
}

use crate::constant::router::Route;
use crate::layout::{Footer, Header, Sidebar};
use crate::page::airdrop::Airdrop;
use crate::page::home::Home;
use crate::page::model::Model;
use crate::page::not_found::NotFound;
use crate::page::presale::Presale;
use crate::page::upload::Upload;
use crate::types::WalletConnect;

pub(crate) fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Airdrop => html! { <Airdrop/> },
        Route::Presale => html! { <Presale/> },
        Route::Documents => html! { <Model/> },
        Route::Calendar => html! { <Upload/> },
        Route::Reports => {
            html! {<p>{"This is reports manager"}</p>}
        }
        Route::Post { id, cid } => {
            html! {<p>{format!("You are looking at Comment {} of Post {}", id, cid)}</p>}
        }
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_state(|| Rc::new(WalletConnect::new()));

    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let filename = G_FILENAME;

                let mut client = Client::new(
                    Metadata {
                        description: "Defi WalletConnect example.".into(),
                        url: "http://localhost:8080/".parse().expect("url"),
                        icons: vec![],
                        name: "Defi WalletConnect Web3 Example".into(),
                    },
                    None,
                )
                .await
                .unwrap_throw();

                client
                    .run_callback(Box::new(
                        move |message: ClientChannelMessage| -> eyre::Result<()> {
                            match message.state {
                                ClientChannelMessageType::Connected => {
                                    log::debug!("Connected");
                                    if let Some(info) = message.session {
                                        log::debug!("session info: {:?}", info);
                                        wallet::write_session_to_file(&info, filename);
                                        Ok(())
                                    } else {
                                        Err(eyre!("no session info"))
                                    }
                                }
                                ClientChannelMessageType::Disconnected => {
                                    log::debug!("Disconnected");
                                    if let Some(info) = message.session {
                                        log::debug!("session info: {:?}", info);
                                        Ok(())
                                    } else {
                                        Err(eyre!("no session info"))
                                    }
                                }
                                ClientChannelMessageType::Connecting => {
                                    log::debug!("Connecting");
                                    if let Some(info) = &message.session {
                                        log::debug!("Connecting url => {:?}", info.url());
                                        wallet::new_qr(info.url());
                                        // info.uri().print_qr_uri();
                                        // wallet::write_session_to_file(info, filename)
                                        Ok(())
                                    } else {
                                        Err(eyre!("no session info"))
                                    }
                                }
                                ClientChannelMessageType::Updated => {
                                    log::debug!("Updated");
                                    if let Some(info) = &message.session {
                                        log::debug!("Connecting url => {:?}", info.url());
                                        wallet::write_session_to_file(info, filename)
                                    } else {
                                        Err(eyre!("no session info"))
                                    }
                                }
                            }
                        },
                    ))
                    .await
                    .unwrap();
            });
            || ()
        },
        (),
    );

    html! {
        <ContextProvider<Rc<WalletConnect>> context={(*ctx).clone()}>

        <BrowserRouter>
                <div class="flex min-h-screen">
                    <Sidebar />

                    <div class="flex-1 flex flex-col h-screen">
                        <Header />

                        <main class="flex-1">
                            <Switch<Route> render={Switch::render(switch)} />
                        </main>

                        <Footer />
                    </div>
                </div>
        </BrowserRouter>

        </ContextProvider<Rc<WalletConnect>>>
    }
}
