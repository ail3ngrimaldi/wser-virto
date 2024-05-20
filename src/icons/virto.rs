use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct Virto;
impl IconShape for Virto {
    fn view_box(&self) -> String {
        String::from("0 0 177 177")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            circle { cx:"88.5", cy:"88.5", r:"88.5", fill:"#10352B" }
            path { d: "M119.65 88.8L104.55 114.9H72.85L57.75 88.8L72.85 62.6H104.55L119.65 88.8ZM112.45 39H65.05C61.45 39 58.15 40.9 56.35 44L33.35 83.8C31.55 86.9 31.55 90.7 33.35 93.8L56.35 133.6C58.15 136.7 61.45 138.6 65.05 138.6H112.45C116.05 138.6 119.35 136.7 121.15 133.6L144.15 93.8C145.95 90.7 145.95 86.9 144.15 83.8L121.05 44C119.25 40.9 115.95 39 112.45 39Z", fill:"#DECFF1" }
        )
    }
}
