macro_rules! impl_attrs {
    () => {
        pub fn get_attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|s| s.as_str())
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect();
            self
        }
    };
}

pub mod graph {
    type Attribute = std::collections::HashMap<String, String>;

    pub mod graph_items {
        pub mod node {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: crate::graph::Attribute,
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_owned(),
                        attrs: crate::graph::Attribute::new(),
                    }
                }
                impl_attrs!();
            }
        }
        pub mod edge {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                pub src: String,
                pub sink: String,
                pub attrs: crate::graph::Attribute,
            }
            impl Edge {
                pub fn new(src: &str, sink: &str) -> Self {
                    Self {
                        src: src.to_owned(),
                        sink: sink.to_owned(),
                        attrs: crate::graph::Attribute::new(),
                    }
                }
                impl_attrs!();
            }
        }
    }
    use self::graph_items::{edge::Edge, node::Node};

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: Attribute,
    }
    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|&(a, b)| (a.to_owned(), b.to_owned()))
                .collect();
            self
        }
        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|x| x.name == name)
        }
    }
}
