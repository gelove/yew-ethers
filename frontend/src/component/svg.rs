use yew::{
    prelude::*,
    virtual_dom::{vtag::SVG_NAMESPACE, VTag},
};

fn nav_tag(activated: bool, child: Html) -> VTag {
    let mut svg = VTag::new("svg");
    let class = if activated {
        "active"
    } else {
        "group-hover:text-gray-300"
    };
    svg.add_attribute("class", class.to_owned());
    svg.add_attribute("xmlns", SVG_NAMESPACE);
    svg.add_attribute("fill", "none".to_owned());
    svg.add_attribute("stroke", "currentColor".to_owned());
    svg.add_attribute("aria-hidden", "true".to_owned());
    svg.add_child(child);
    svg
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub activated: bool,
}

// #[function_component(HomeSVG)]
pub fn home(activated: bool) -> VTag {
    // let Props { activated } = props;
    nav_tag(
        activated,
        html! {
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
        },
    )
}

pub fn users(activated: bool) -> VTag {
    nav_tag(
        activated,
        html! {
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
        },
    )
}

pub fn folder(activated: bool) -> VTag {
    nav_tag(
        activated,
        html! {
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
        },
    )
}

pub fn calendar(activated: bool) -> VTag {
    nav_tag(
        activated,
        html! {
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
        },
    )
}

pub fn inbox(activated: bool) -> VTag {
    nav_tag(
        activated,
        html! {
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
        },
    )
}

pub fn chart_bar(activated: bool) -> VTag {
    nav_tag(
        activated,
        html! {
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
        },
    )
}
