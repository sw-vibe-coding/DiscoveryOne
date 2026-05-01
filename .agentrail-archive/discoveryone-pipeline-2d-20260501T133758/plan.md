# Saga: discoveryone-pipeline-2d

Goal: implement M9: a 2D pipeline editor demo where bundled definitions can be placed on a canvas, connected with type-checked edges, and run through a small browser pipeline baseline.

Milestones:
1. Add a RED acceptance baseline for a small pipeline, initially Power -> output, with stable HTML/snapshot output.
2. Model pipeline nodes and edges as a small serializable data structure, without adding persistence yet.
3. Render the 2D pipeline canvas in the Web UI with nodes, ports, and deterministic layout.
4. Add narrow connection validation and inline error output for incompatible or missing edges.
5. Execute the small pipeline path in the Web runtime and make the M9 acceptance baseline green.
6. Update docs/status for the M9 demo and close the saga.