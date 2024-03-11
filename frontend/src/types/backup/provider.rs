use std::cmp::PartialEq;
use std::rc::Rc;
use web3::{
    transports::eip_1193::{Eip1193, Provider},
    Web3,
};
use yew::Properties;

#[derive(Debug, Clone)]
pub struct Web3Provider {
    pub web3: Option<Rc<Web3<Eip1193>>>,
}

impl PartialEq for Web3Provider {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
    fn ne(&self, other: &Self) -> bool {
        self != other
    }
}

impl Properties for Web3Provider {
    type Builder = Self;

    fn builder() -> Self::Builder {
        Self::new()
    }
}

impl Web3Provider {
    pub fn new() -> Self {
        // Provider::default() 默认实现调用以下js方法
        // inline_js = "export function get_provider_js() {return window.ethereum}"
        match Provider::default() {
            Ok(Some(p)) => Self {
                web3: Some(Rc::new(Web3::new(Eip1193::new(p)))),
            },
            _ => Self { web3: None },
        }
    }
}
