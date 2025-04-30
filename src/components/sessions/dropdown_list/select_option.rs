use termio::cli::constant::ProtocolType;
use tmui::{
    icons::svg_dom::SvgDom,
    prelude::Align,
    views::{
        cell::{
            cell_render::{SvgCellRender, TextCellRender},
            Cell,
        },
        list_view::list_view_object::ListViewObject,
        node::node_render::NodeRender,
    },
};

use crate::{assets::Asset, config::TermdotConfig};

pub struct SelectOption {
    protocol_type: ProtocolType,
}

impl ListViewObject for SelectOption {
    #[inline]
    fn cells(&self) -> Vec<Cell> {
        let dom = match self.protocol_type {
            ProtocolType::Cmd => None,
            ProtocolType::PowerShell => Some(SvgDom::from_bytes(
                Asset::get("icons/powershell.svg").unwrap().data.as_ref(),
            )),
            ProtocolType::Custom => Some(SvgDom::from_bytes(
                Asset::get("icons/godotengine.svg").unwrap().data.as_ref(),
            )),
            _ => None,
        };
        vec![
            Cell::value_cell().value(self.protocol_type).build(),
            Cell::svg()
                .cell_render(SvgCellRender::builder().dom(dom).width(20).build())
                .build(),
            Cell::string()
                .value(self.protocol_type.as_str().to_string())
                .cell_render(
                    TextCellRender::builder()
                        .color(TermdotConfig::foreground())
                        .valign(Align::End)
                        .build(),
                )
                .build(),
        ]
    }

    #[inline]
    fn node_render(&self) -> NodeRender {
        NodeRender::builder()
            .selection_color(TermdotConfig::selection())
            .hover_color(TermdotConfig::selection())
            .build()
    }
}

impl SelectOption {
    #[inline]
    pub fn new(protocol_type: ProtocolType) -> Self {
        Self { protocol_type }
    }
}
