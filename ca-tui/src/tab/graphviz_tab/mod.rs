use std::{fs::File, path::Path, process::Command};

use image::ImageReader;
use libca::{Edge, Node};
use ratatui::{layout::Rect, Frame};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol, StatefulImage};

use super::Tab;

pub struct GraphvizTab {
    image: StatefulProtocol,
}

impl GraphvizTab {
    pub fn new(model: &libca::Model) -> anyhow::Result<Self> {
        const DOT: &str = "/tmp/test.dot";
        const PNG: &str = "/tmp/test.png";

        let model = ModelWrapper(model);
        model.render_to(DOT)?;

        dot_to_png(DOT, PNG)?;

        let img = ImageReader::open(PNG)?.decode()?;
        let picker = Picker::from_query_stdio()?;
        let image = picker.new_resize_protocol(img);

        Ok(Self { image })
    }
}

impl Tab for GraphvizTab {
    fn draw(&mut self, _model: &libca::Model, area: Rect, ctx: &mut Frame) {
        ctx.render_stateful_widget(StatefulImage::default(), area, &mut self.image);
    }
}

struct ModelWrapper<'m>(&'m libca::Model);

impl ModelWrapper<'_> {
    pub fn render_to(&self, file_path: impl AsRef<Path>) -> anyhow::Result<()> {
        let mut f = File::create(file_path.as_ref())?;
        dot::render(self, &mut f)?;

        Ok(())
    }
}

impl<'d> dot::GraphWalk<'d, &'d Node, &'d Edge> for ModelWrapper<'d> {
    fn nodes(&'d self) -> dot::Nodes<'d, &'d Node> {
        self.0.nodes().map(|(_, node)| node).collect()
    }

    fn edges(&'d self) -> dot::Edges<'d, &'d Edge> {
        self.0.all_edges().iter().collect()
    }

    fn source(&'d self, edge: &&'d Edge) -> &'d Node {
        let node_id = edge.from_node_id();
        let node = self.0.get_node(node_id).unwrap();
        node
    }

    fn target(&'d self, edge: &&'d Edge) -> &'d Node {
        let node_id = edge.to_node_id();
        let node = self.0.get_node(node_id).unwrap();
        node
    }
}

impl<'d> dot::Labeller<'d, &'d Node, &'d Edge> for ModelWrapper<'d> {
    fn graph_id(&'d self) -> dot::Id<'d> {
        dot::Id::new("TODO").unwrap()
    }

    fn node_id(&'d self, n: &&'d Node) -> dot::Id<'d> {
        dot::Id::new(n.name()).unwrap()
    }

    fn node_label(&'d self, n: &&'d Node) -> dot::LabelText<'d> {
        dot::LabelText::label(n.name())
    }

    fn edge_label(&'d self, e: &&'d Edge) -> dot::LabelText<'d> {
        dot::LabelText::label(e.name())
    }
}

fn dot_to_png(dot: &str, png: &str) -> anyhow::Result<()> {
    Command::new("dot")
        .args(["-Tpng", "-o", png, dot])
        .spawn()?
        .wait()?;

    Ok(())
}
