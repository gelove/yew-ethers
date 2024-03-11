use yew_router::prelude::Routable;

pub const DASHBOARD: &str = "Dashboard";
pub const AIRDROP: &str = "Airdrop";
pub const PRESALE: &str = "Presale";
pub const CALENDAR: &str = "Calendar";
pub const DOCUMENTS: &str = "Documents";
pub const REPORTS: &str = "Reports";

#[derive(Clone, Debug, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Home,
    #[at("/airdrop")]
    Airdrop,
    #[at("/presale")]
    Presale,
    #[at("/documents")]
    Documents,
    #[at("/calendar")]
    Calendar,
    #[at("/reports")]
    Reports,
    #[at("/post/:id/comment/:cid")]
    Post { id: u32, cid: u32 },
    #[not_found]
    #[at("/404")]
    NotFound,
}
