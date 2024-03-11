use std::{
    cmp::PartialEq,
    fmt::{Display, Formatter, Result},
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Nav {
    Dashboard,
    Presale,
    Calendar,
    Documents,
    Reports,
    Unknown,
}

impl Display for Nav {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Nav::Dashboard => write!(f, "Dashboard"),
            Nav::Presale => write!(f, "Presale"),
            Nav::Calendar => write!(f, "Calendar"),
            Nav::Documents => write!(f, "Documents"),
            Nav::Reports => write!(f, "Reports"),
            Nav::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<Nav> for String {
    fn from(nav: Nav) -> Self {
        match nav {
            Nav::Dashboard => "Dashboard".into(),
            Nav::Presale => "Presale".into(),
            Nav::Calendar => "Calendar".into(),
            Nav::Documents => "Documents".into(),
            Nav::Reports => "Reports".into(),
            Nav::Unknown => "Unknown".into(),
        }
    }
}

impl From<String> for Nav {
    fn from(nav: String) -> Self {
        match nav {
            nav if nav == "Dashboard" => Nav::Dashboard,
            nav if nav == "Presale" => Nav::Presale,
            nav if nav == "Calendar" => Nav::Calendar,
            nav if nav == "Documents" => Nav::Documents,
            nav if nav == "Reports" => Nav::Reports,
            _ => Nav::Unknown,
        }
    }
}
