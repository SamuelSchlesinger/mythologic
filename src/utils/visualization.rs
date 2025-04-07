use crate::core::MythOntology;
use crate::core::MythEntity;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Generate an HTML visualization of a MythOntology
pub fn generate_html_visualization(ontology: &MythOntology, output_path: &Path) -> std::io::Result<()> {
    let entities = ontology.all_entities();
    let mut file = File::create(output_path)?;
    
    // Count entity types
    let mut entity_type_counts: HashMap<&'static str, usize> = HashMap::new();
    for entity in &entities {
        *entity_type_counts.entry(entity.entity_type()).or_insert(0) += 1;
    }
    
    // Create connections for graph
    let mut connections: Vec<(String, String, String)> = Vec::new();
    let mut processed_entities: HashSet<String> = HashSet::new();
    
    // First pass: collect all entities and their relationships
    for entity in &entities {
        let entity_id = entity.id().to_string();
        processed_entities.insert(entity_id.clone());
        
        // Check for relationship vectors in entity
        for related_id in entity.relationships() {
            if let Some(related_entity) = ontology.get_entity(&related_id) {
                connections.push((
                    entity.id().to_string(),
                    related_entity.id().to_string(),
                    format!("{} → {}", entity.entity_type(), related_entity.entity_type())
                ));
            }
        }
    }
    
    // Second pass: extract relationships directly from relationship entities
    for entity in &entities {
        match entity {
            MythEntity::FamilyRelationship(rel) => {
                let source_id = rel.relationship.source_id.clone();
                let target_id = rel.relationship.target_id.clone();
                
                if let (Some(source), Some(target)) = (ontology.get_entity(&source_id), ontology.get_entity(&target_id)) {
                    connections.push((
                        source.id().to_string(),
                        target.id().to_string(),
                        format!("{} → Family", entity.name())
                    ));
                    
                    // If bidirectional, add the reverse relationship too
                    if rel.relationship.bidirectional {
                        connections.push((
                            target.id().to_string(),
                            source.id().to_string(),
                            format!("{} → Family", entity.name())
                        ));
                    }
                }
            },
            MythEntity::AllianceRelationship(rel) => {
                let source_id = rel.relationship.source_id.clone();
                let target_id = rel.relationship.target_id.clone();
                
                if let (Some(source), Some(target)) = (ontology.get_entity(&source_id), ontology.get_entity(&target_id)) {
                    connections.push((
                        source.id().to_string(),
                        target.id().to_string(),
                        format!("{} → Alliance", entity.name())
                    ));
                    
                    // If bidirectional, add the reverse relationship too
                    if rel.relationship.bidirectional {
                        connections.push((
                            target.id().to_string(),
                            source.id().to_string(),
                            format!("{} → Alliance", entity.name())
                        ));
                    }
                }
            },
            MythEntity::ConflictRelationship(rel) => {
                let source_id = rel.relationship.source_id.clone();
                let target_id = rel.relationship.target_id.clone();
                
                if let (Some(source), Some(target)) = (ontology.get_entity(&source_id), ontology.get_entity(&target_id)) {
                    connections.push((
                        source.id().to_string(),
                        target.id().to_string(),
                        format!("{} → Conflict", entity.name())
                    ));
                }
            },
            MythEntity::TransformationRelationship(rel) => {
                let source_id = rel.relationship.source_id.clone();
                let target_id = rel.relationship.target_id.clone();
                
                if let (Some(source), Some(target)) = (ontology.get_entity(&source_id), ontology.get_entity(&target_id)) {
                    connections.push((
                        source.id().to_string(),
                        target.id().to_string(),
                        format!("{} → Transformation", entity.name())
                    ));
                }
            },
            _ => {}
        }
    }
    
    // Generate HTML content
    let html_content = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Myth Ontology Visualization</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 0;
            color: #333;
            background-color: #f9f9f9;
        }}
        .container {{
            width: 95%;
            margin: 0 auto;
            padding: 20px;
        }}
        h1, h2, h3 {{
            color: #2c3e50;
        }}
        .summary-card {{
            background: white;
            border-radius: 8px;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
            padding: 20px;
            margin-bottom: 20px;
        }}
        .entity-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
            gap: 20px;
            margin-top: 20px;
        }}
        .entity-card {{
            background: white;
            border-radius: 8px;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
            padding: 15px;
            transition: transform 0.3s ease;
        }}
        .entity-card:hover {{
            transform: translateY(-5px);
        }}
        .entity-type {{
            font-size: 0.8em;
            color: #7f8c8d;
            text-transform: uppercase;
            letter-spacing: 1px;
            margin-bottom: 5px;
        }}
        .entity-name {{
            font-size: 1.2em;
            font-weight: bold;
            margin-bottom: 10px;
            color: #2980b9;
        }}
        .entity-description {{
            font-size: 0.9em;
            line-height: 1.5;
        }}
        .entity-metadata {{
            font-size: 0.8em;
            margin-top: 10px;
            color: #7f8c8d;
        }}
        .chart-container {{
            width: 100%;
            height: 300px;
            margin-bottom: 20px;
        }}
        .tabs {{
            display: flex;
            border-bottom: 1px solid #ddd;
            margin-bottom: 20px;
        }}
        .tab {{
            padding: 10px 20px;
            cursor: pointer;
            border: 1px solid transparent;
        }}
        .tab.active {{
            border: 1px solid #ddd;
            border-bottom: 1px solid white;
            border-radius: 5px 5px 0 0;
            margin-bottom: -1px;
            background-color: white;
        }}
        .tab-content {{
            display: none;
        }}
        .tab-content.active {{
            display: block;
        }}
        .relationship-list {{
            list-style-type: none;
            padding: 0;
        }}
        .relationship-list li {{
            padding: 5px 0;
            border-bottom: 1px dashed #eee;
        }}
        .relationship-list li:last-child {{
            border-bottom: none;
        }}
        .search-container {{
            margin-bottom: 20px;
        }}
        #searchInput {{
            width: 100%;
            padding: 10px;
            border: 1px solid #ddd;
            border-radius: 4px;
            font-size: 16px;
        }}
        .culture-pill {{
            display: inline-block;
            background-color: #3498db;
            color: white;
            padding: 2px 8px;
            border-radius: 12px;
            font-size: 0.75em;
            margin-right: 5px;
        }}
        svg {{
            width: 100%;
            height: 600px;
            border: 1px solid #ddd;
            border-radius: 8px;
            background-color: white;
        }}
        .node {{
            cursor: pointer;
        }}
        .node text {{
            font-size: 12px;
        }}
        .link {{
            stroke: #999;
            stroke-opacity: 0.6;
        }}
        .node-deity {{ fill: #e74c3c; }}
        .node-hero {{ fill: #3498db; }}
        .node-creature {{ fill: #2ecc71; }}
        .node-artifact {{ fill: #f39c12; }}
        .node-location {{ fill: #9b59b6; }}
        .node-concept {{ fill: #1abc9c; }}
        .node-culture {{ fill: #34495e; }}
        .node-pantheon {{ fill: #16a085; }}
        .node-relationship {{ fill: #95a5a6; }}
        .legend {{
            display: flex;
            flex-wrap: wrap;
            margin-top: 10px;
            gap: 10px;
        }}
        .legend-item {{
            display: flex;
            align-items: center;
            margin-right: 15px;
        }}
        .legend-color {{
            width: 15px;
            height: 15px;
            border-radius: 50%;
            margin-right: 5px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Mythological Ontology Visualization</h1>
        
        <div class="summary-card">
            <h2>Ontology Summary</h2>
            <p>Total entities: <strong>{}</strong></p>
            <div class="legend">
                {}
            </div>
        </div>

        <div class="tabs">
            <div class="tab active" onclick="openTab(event, 'graph')">Relationship Graph</div>
            <div class="tab" onclick="openTab(event, 'entities')">Entity List</div>
        </div>

        <div id="graph" class="tab-content active">
            <div class="search-container">
                <input type="text" id="searchInput" placeholder="Search entities..." onkeyup="filterGraph()">
            </div>
            <svg id="ontology-graph"></svg>
        </div>

        <div id="entities" class="tab-content">
            <div class="search-container">
                <input type="text" id="entitySearchInput" placeholder="Search entities..." onkeyup="filterEntities()">
            </div>
            <div class="entity-grid" id="entityGrid">
                {}
            </div>
        </div>
    </div>

    <script>
        // Tab functionality
        function openTab(evt, tabName) {{
            var i, tabcontent, tablinks;
            tabcontent = document.getElementsByClassName("tab-content");
            for (i = 0; i < tabcontent.length; i++) {{
                tabcontent[i].className = tabcontent[i].className.replace(" active", "");
            }}
            tablinks = document.getElementsByClassName("tab");
            for (i = 0; i < tablinks.length; i++) {{
                tablinks[i].className = tablinks[i].className.replace(" active", "");
            }}
            document.getElementById(tabName).className += " active";
            evt.currentTarget.className += " active";
            
            if (tabName === 'graph') {{
                renderGraph();
            }}
        }}

        // Entity filtering
        function filterEntities() {{
            var input, filter, grid, cards, name, i;
            input = document.getElementById("entitySearchInput");
            filter = input.value.toUpperCase();
            grid = document.getElementById("entityGrid");
            cards = grid.getElementsByClassName("entity-card");
            
            for (i = 0; i < cards.length; i++) {{
                name = cards[i].getElementsByClassName("entity-name")[0];
                if (name.innerHTML.toUpperCase().indexOf(filter) > -1) {{
                    cards[i].style.display = "";
                }} else {{
                    cards[i].style.display = "none";
                }}
            }}
        }}

        // Graph filtering
        function filterGraph() {{
            var input = document.getElementById("searchInput");
            var filter = input.value.toUpperCase();
            var nodes = document.querySelectorAll('.node');
            
            if (filter === '') {{
                // Show all nodes and links
                nodes.forEach(function(node) {{
                    node.style.opacity = 1;
                }});
                document.querySelectorAll('.link').forEach(function(link) {{
                    link.style.opacity = 0.6;
                }});
                return;
            }}
            
            // First, make all nodes and links semi-transparent
            nodes.forEach(function(node) {{
                node.style.opacity = 0.2;
            }});
            document.querySelectorAll('.link').forEach(function(link) {{
                link.style.opacity = 0.1;
            }});
            
            // Then highlight matching nodes and their connections
            nodes.forEach(function(node) {{
                var name = node.querySelector('text').textContent;
                if (name.toUpperCase().indexOf(filter) > -1) {{
                    node.style.opacity = 1;
                    
                    // Highlight connected nodes and links
                    var nodeId = node.id;
                    document.querySelectorAll('.link[data-source="' + nodeId + '"], .link[data-target="' + nodeId + '"]').forEach(function(link) {{
                        link.style.opacity = 0.8;
                        var sourceId = link.getAttribute('data-source');
                        var targetId = link.getAttribute('data-target');
                        document.getElementById(sourceId).style.opacity = 1;
                        document.getElementById(targetId).style.opacity = 1;
                    }});
                }}
            }});
        }}

        // D3.js graph visualization
        function renderGraph() {{
            const nodes = {{}};
            const links = {{}};
            
            // Wait for D3.js to load
            setTimeout(function() {{
                const svg = document.getElementById('ontology-graph');
                
                // Clear existing content
                while (svg.firstChild) {{
                    svg.removeChild(svg.firstChild);
                }}
                
                const width = svg.clientWidth;
                const height = svg.clientHeight;
                const centerX = width / 2;
                const centerY = height / 2;
                
                // Create nodes
                {}
                
                // Create links
                {}
                
                // Initialize simulation
                const simulation = d3.forceSimulation(Object.values(nodes))
                    .force("link", d3.forceLink(Object.values(links)).id(function(d) {{ return d.id; }}).distance(100))
                    .force("charge", d3.forceManyBody().strength(-200))
                    .force("center", d3.forceCenter(centerX, centerY))
                    .force("collide", d3.forceCollide().radius(30));
                
                // Create links
                const link = d3.select(svg)
                    .selectAll("line")
                    .data(Object.values(links))
                    .enter()
                    .append("line")
                    .attr("class", "link")
                    .attr("stroke-width", 1.5)
                    .attr("data-source", function(d) {{ return d.source.id; }})
                    .attr("data-target", function(d) {{ return d.target.id; }});
                
                // Create node groups
                const node = d3.select(svg)
                    .selectAll(".node")
                    .data(Object.values(nodes))
                    .enter()
                    .append("g")
                    .attr("class", "node")
                    .attr("id", function(d) {{ return d.id; }})
                    .call(d3.drag()
                        .on("start", dragstarted)
                        .on("drag", dragged)
                        .on("end", dragended));
                
                // Add circles to nodes
                node.append("circle")
                    .attr("r", function(d) {{ return getNodeSize(d.type); }})
                    .attr("class", function(d) {{ return "node-" + d.type.toLowerCase(); }})
                    .append("title")
                    .text(function(d) {{ return d.label; }});
                
                // Add text labels to nodes
                node.append("text")
                    .attr("dy", ".35em")
                    .attr("x", function(d) {{ return getNodeSize(d.type) + 5; }})
                    .text(function(d) {{ return d.label; }});
                
                // Update positions on each tick
                simulation.on("tick", function() {{
                    link
                        .attr("x1", function(d) {{ return d.source.x; }})
                        .attr("y1", function(d) {{ return d.source.y; }})
                        .attr("x2", function(d) {{ return d.target.x; }})
                        .attr("y2", function(d) {{ return d.target.y; }});
                    
                    node
                        .attr("transform", function(d) {{ return "translate(" + d.x + "," + d.y + ")"; }});
                }});
                
                // Drag functions
                function dragstarted(event, d) {{
                    if (!event.active) simulation.alphaTarget(0.3).restart();
                    d.fx = d.x;
                    d.fy = d.y;
                }}
                
                function dragged(event, d) {{
                    d.fx = event.x;
                    d.fy = event.y;
                }}
                
                function dragended(event, d) {{
                    if (!event.active) simulation.alphaTarget(0);
                    d.fx = null;
                    d.fy = null;
                }}
                
                // Node size based on type
                function getNodeSize(type) {{
                    switch(type.toLowerCase()) {{
                        case 'deity': return 15;
                        case 'hero': return 12;
                        case 'creature': return 10;
                        case 'artifact': return 10;
                        case 'location': return 12;
                        case 'concept': return 8;
                        case 'culture': return 15;
                        case 'pantheon': return 12;
                        default: return 8;
                    }}
                }}
            }}, 100);
        }}
        
        // Load D3.js dynamically
        function loadD3() {{
            return new Promise(function(resolve, reject) {{
                const script = document.createElement('script');
                script.src = 'https://d3js.org/d3.v7.min.js';
                script.onload = resolve;
                script.onerror = reject;
                document.head.appendChild(script);
            }});
        }}
        
        // Initialize on page load
        window.onload = function() {{
            loadD3().then(function() {{
                renderGraph();
            }});
        }};
    </script>
</body>
</html>
"#,
        // Total entities
        entities.len(),
        
        // Entity type legend
        entity_type_counts.iter()
            .map(|(entity_type, count)| {
                let color_class = format!("node-{}", entity_type.to_lowercase());
                format!(
                    r#"<div class="legend-item">
                        <div class="legend-color {}" style="background-color: var(--{}, #999);"></div>
                        <span>{}: {}</span>
                    </div>"#,
                    color_class, color_class, entity_type, count
                )
            })
            .collect::<Vec<_>>()
            .join("\n                "),
        
        // Entity cards
        entities.iter()
            .map(|entity| {
                let culture_display = entity.culture_name()
                    .map(|c| format!(r#"<span class="culture-pill">{}</span>"#, c))
                    .unwrap_or_default();
                
                format!(
                    r#"<div class="entity-card" data-type="{}">
                        <div class="entity-type">{} {}</div>
                        <div class="entity-name">{}</div>
                        <div class="entity-description">{}</div>
                        <div class="entity-metadata">
                            <div>ID: {}</div>
                            <div>Relationships: {}</div>
                        </div>
                    </div>"#,
                    entity.entity_type().to_lowercase(),
                    entity.entity_type(),
                    culture_display,
                    entity.name(),
                    entity.metadata().attributes.get("description")
                        .unwrap_or(&String::from("No description available.")).clone(),
                    entity.id(),
                    entity.relationships().len()
                )
            })
            .collect::<Vec<_>>()
            .join("\n                "),
        
        // D3 Node Creation
        entities.iter()
            .map(|entity| {
                format!(
                    r#"nodes['{}'] = {{ 
                        id: '{}', 
                        name: '{}', 
                        label: '{}',
                        type: '{}',
                        relationships: {}
                    }};"#,
                    entity.id(),
                    entity.id(),
                    entity.name().replace("'", "\\'"),
                    entity.name().replace("'", "\\'"),
                    entity.entity_type(),
                    entity.relationships().len()
                )
            })
            .collect::<Vec<_>>()
            .join("\n                "),
        
        // D3 Link Creation
        connections.iter()
            .enumerate()
            .map(|(idx, (source, target, _))| {
                format!(
                    r#"links['link{}'] = {{ 
                        source: '{}', 
                        target: '{}',
                        value: 1
                    }};"#,
                    idx,
                    source,
                    target
                )
            })
            .collect::<Vec<_>>()
            .join("\n                ")
    );
    
    file.write_all(html_content.as_bytes())?;
    Ok(())
}