use std::collections::HashMap;

use eyre::ContextCompat;
use itertools::Itertools;

use super::{Graph, GraphNodeId};

pub type GraphModuleId = usize;
pub type GraphModulePortId = usize;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum GraphModulePortType {
    #[default]
    InputNet,
    OutputNet,
    InputReg,
    OutputReg,
    InOut,
}

#[derive(Clone, Debug)]
pub enum GraphModulePortTarget {
    Node(GraphNodeId),
    Module(GraphModuleId, GraphModulePortId),
}

impl Default for GraphModulePortTarget {
    fn default() -> Self {
        Self::Node(0)
    }
}

#[derive(Default, Clone, Debug)]
pub struct GraphModulePort {
    pub id: GraphModulePortId,
    pub name: String,
    pub port_type: GraphModulePortType,
    pub target: GraphModulePortTarget,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum GraphModuleVariableType {
    #[default]
    Wire,
    Reg,
}

#[derive(Default, Clone, Debug)]
pub struct GraphModuleVariable {
    pub var_type: GraphModulePortType,
    pub source: GraphModulePortId,
    pub targe: GraphModulePortId,
}

#[derive(Default, Clone, Debug)]
enum GraphModuleState {
    #[default]
    UnInitialized,
    Initialized,
}

#[derive(Default, Clone, Debug)]
pub struct GraphModule {
    state: GraphModuleState,
    // only root GraphModule have context
    context: Option<GraphModuleContext>,
    pub id: GraphModuleId,
    pub graph: Option<GraphId>,
    pub instances: Vec<GraphModuleId>,
    pub vars: Vec<GraphModuleVariable>,
    pub ports: Vec<GraphModulePort>,
}

pub type GraphId = usize;

#[derive(Default, Clone, Debug)]
pub struct UniqueGraph(GraphId, Graph);

#[derive(Default, Clone, Debug)]
struct GraphModuleContext {
    pub modules: Vec<Box<GraphModule>>,
    pub module_index: HashMap<GraphModuleId, usize>,
    pub graphs: Vec<Box<UniqueGraph>>,
    pub graph_index: HashMap<GraphId, usize>,
}

impl GraphModule {
    pub fn from_instances(instances: Vec<Box<GraphModule>>) -> Self {
        todo!()
    }

    pub fn numbering_ports(&mut self, base: usize) -> usize {
        self.ports
            .iter_mut()
            .enumerate()
            .for_each(|(index, port)| port.id = base + index);

        self.state = GraphModuleState::Initialized;

        base + self.ports.len()
    }

    pub fn check_init(&self) -> eyre::Result<()> {
        match self.state {
            GraphModuleState::UnInitialized => {
                eyre::bail!("You cannot use unitialized graph module!")
            }
            GraphModuleState::Initialized => Ok(()),
        }
    }

    pub fn port_by_name(&self, name: &str) -> eyre::Result<&GraphModulePort> {
        // TODO: makes this fast
        self.ports
            .iter()
            .find(|port| port.name == name)
            .context("Port not found!")
    }
}

impl From<&Graph> for GraphModule {
    fn from(value: &Graph) -> Self {
        value.clone().into()
    }
}

impl From<Graph> for GraphModule {
    fn from(value: Graph) -> Self {
        Self {
            id: 0,
            ports: value
                .inputs()
                .iter()
                .map(|input| GraphModulePort {
                    port_type: GraphModulePortType::InputNet,
                    target: GraphModulePortTarget::Node(*input),
                    name: value
                        .find_node_by_id(*input)
                        .unwrap()
                        .kind
                        .as_input()
                        .clone(),
                    ..Default::default()
                })
                .chain(value.outputs().iter().map(|output| {
                    GraphModulePort {
                        port_type: GraphModulePortType::OutputNet,
                        target: GraphModulePortTarget::Node(*output),
                        name: value
                            .find_node_by_id(*output)
                            .unwrap()
                            .kind
                            .as_output()
                            .clone(),
                        ..Default::default()
                    }
                }))
                .collect_vec(),
            graph: Some(0),
            context: Some(GraphModuleContext {
                graphs: vec![Box::new(UniqueGraph(0, value))],
                graph_index: HashMap::from_iter(vec![(0, 0)]),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

impl From<&GraphModule> for Graph {
    fn from(value: &GraphModule) -> Self {
        todo!()
    }
}
