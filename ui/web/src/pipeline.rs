#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Pipeline {
    pub(crate) name: &'static str,
    pub(crate) inputs: PipelineInputs,
    pub(crate) nodes: &'static [PipelineNode],
    pub(crate) edges: &'static [PipelineEdge],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PipelineInputs {
    pub(crate) n: &'static str,
    pub(crate) e: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PipelineNode {
    pub(crate) id: &'static str,
    pub(crate) label: &'static str,
    pub(crate) kind: &'static str,
    pub(crate) position: PipelinePosition,
    pub(crate) inputs: &'static [PipelinePort],
    pub(crate) outputs: &'static [PipelinePort],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PipelinePosition {
    pub(crate) x: u8,
    pub(crate) y: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PipelinePort {
    pub(crate) id: &'static str,
    pub(crate) label: &'static str,
    pub(crate) value_type: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PipelineEdge {
    pub(crate) from_node: &'static str,
    pub(crate) from_port: &'static str,
    pub(crate) to_node: &'static str,
    pub(crate) to_port: &'static str,
}

const POWER_INPUTS: [PipelinePort; 2] = [
    PipelinePort {
        id: "n",
        label: "n",
        value_type: "Z",
    },
    PipelinePort {
        id: "e",
        label: "e",
        value_type: "Z",
    },
];
const POWER_OUTPUTS: [PipelinePort; 1] = [PipelinePort {
    id: "p",
    label: "p",
    value_type: "Z",
}];
const OUTPUT_INPUTS: [PipelinePort; 1] = [PipelinePort {
    id: "value",
    label: "value",
    value_type: "Z",
}];
const OUTPUT_OUTPUTS: [PipelinePort; 0] = [];

const POWER_OUTPUT_NODES: [PipelineNode; 2] = [
    PipelineNode {
        id: "power",
        label: "Power",
        kind: "numeric",
        position: PipelinePosition { x: 0, y: 0 },
        inputs: &POWER_INPUTS,
        outputs: &POWER_OUTPUTS,
    },
    PipelineNode {
        id: "output",
        label: "Output",
        kind: "sink",
        position: PipelinePosition { x: 1, y: 0 },
        inputs: &OUTPUT_INPUTS,
        outputs: &OUTPUT_OUTPUTS,
    },
];

const POWER_OUTPUT_EDGES: [PipelineEdge; 1] = [PipelineEdge {
    from_node: "power",
    from_port: "p",
    to_node: "output",
    to_port: "value",
}];

pub(crate) const POWER_OUTPUT_PIPELINE: Pipeline = Pipeline {
    name: "Power to Output",
    inputs: PipelineInputs { n: "2", e: "8" },
    nodes: &POWER_OUTPUT_NODES,
    edges: &POWER_OUTPUT_EDGES,
};

pub fn pipeline_power_output_html_snapshot() -> String {
    let pipeline = POWER_OUTPUT_PIPELINE;
    format!(
        r#"<section class="pipeline-canvas" aria-label="Pipeline">
  <header class="pipeline-header"><span>Pipeline</span><strong>{}</strong></header>
  <output class="pipeline-validation" aria-label="Pipeline validation">Pending: pipeline canvas is not rendered yet.</output>
</section>
"#,
        pipeline.name
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_output_fixture_has_deterministic_shape() {
        assert_eq!(POWER_OUTPUT_PIPELINE.name, "Power to Output");
        assert_eq!(POWER_OUTPUT_PIPELINE.inputs, PipelineInputs { n: "2", e: "8" });
        assert_eq!(
            POWER_OUTPUT_PIPELINE
                .nodes
                .iter()
                .map(|node| node.id)
                .collect::<Vec<_>>(),
            ["power", "output"]
        );
        assert_eq!(POWER_OUTPUT_PIPELINE.edges.len(), 1);
    }

    #[test]
    fn power_output_fixture_connects_power_to_output() {
        assert_eq!(
            POWER_OUTPUT_PIPELINE.edges[0],
            PipelineEdge {
                from_node: "power",
                from_port: "p",
                to_node: "output",
                to_port: "value",
            }
        );
        assert_eq!(POWER_OUTPUT_PIPELINE.nodes[0].outputs[0].value_type, "Z");
        assert_eq!(POWER_OUTPUT_PIPELINE.nodes[1].inputs[0].value_type, "Z");
    }
}
