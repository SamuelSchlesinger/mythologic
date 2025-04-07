use mythologic::examples::{
    create_greek_ontology, create_norse_ontology, 
    create_egyptian_ontology, create_celtic_ontology, 
    create_hindu_ontology, create_artifacts_ontology,
    create_heroes_ontology, create_creatures_ontology,
    create_locations_ontology, create_concepts_ontology
};
use mythologic::utils::generate_html_visualization;
use std::env;
use std::process;
use std::path::Path;

fn print_usage() {
    println!("Mythologic Explorer - A tool to visualize mythological ontologies");
    println!("\nUsage:");
    println!("  mythologic_explorer <ontology_name> [output_path]");
    println!("\nAvailable ontologies:");
    println!("  greek      - Greek mythology");
    println!("  norse      - Norse mythology");
    println!("  egyptian   - Egyptian mythology");
    println!("  celtic     - Celtic mythology");
    println!("  hindu      - Hindu mythology");
    println!("  artifacts  - Notable mythological artifacts");
    println!("  heroes     - Heroes across mythologies");
    println!("  creatures  - Mythological creatures");
    println!("  locations  - Mythological locations");
    println!("  concepts   - Abstract mythological concepts");
    println!("  all        - Generate visualizations for all ontologies");
    println!("\nExamples:");
    println!("  mythologic_explorer greek");
    println!("  mythologic_explorer norse ./norse_myths.html");
    println!("  mythologic_explorer all ./visualizations/");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage();
        process::exit(if args.len() < 2 { 1 } else { 0 });
    }
    
    let ontology_name = &args[1].to_lowercase();
    
    // Determine output path
    let output_path = if args.len() > 2 {
        args[2].clone()
    } else {
        format!("./{}_ontology.html", ontology_name)
    };
    
    // Check if we should generate all ontologies
    if ontology_name == "all" {
        println!("Generating visualizations for all ontologies...");
        
        // Create output directory if specified and doesn't exist
        if args.len() > 2 {
            use std::fs;
            if let Err(e) = fs::create_dir_all(&output_path) {
                eprintln!("Error creating output directory: {}", e);
                process::exit(1);
            }
        }
        
        let ontologies = [
            ("greek", create_greek_ontology()),
            ("norse", create_norse_ontology()),
            ("egyptian", create_egyptian_ontology()),
            ("celtic", create_celtic_ontology()),
            ("hindu", create_hindu_ontology()),
            ("artifacts", create_artifacts_ontology()),
            ("heroes", create_heroes_ontology()),
            ("creatures", create_creatures_ontology()),
            ("locations", create_locations_ontology()),
            ("concepts", create_concepts_ontology()),
        ];
        
        for (name, ontology) in ontologies.iter() {
            // If output_path is a directory, put files inside it
            // Otherwise, append the ontology name to the path
            let file_path = if args.len() > 2 {
                format!("{}/{}_ontology.html", output_path, name)
            } else {
                format!("./{}_ontology.html", name)
            };
            
            println!("  Generating {} ontology visualization at {}", name, file_path);
            
            if let Err(e) = generate_html_visualization(ontology, Path::new(&file_path)) {
                eprintln!("Error generating visualization for {} ontology: {}", name, e);
            }
        }
        
        println!("All visualizations generated successfully!");
        return;
    }
    
    // Generate a single ontology visualization
    println!("Generating {} ontology visualization at {}", ontology_name, output_path);
    
    let ontology = match ontology_name.as_str() {
        "greek" => create_greek_ontology(),
        "norse" => create_norse_ontology(),
        "egyptian" => create_egyptian_ontology(),
        "celtic" => create_celtic_ontology(),
        "hindu" => create_hindu_ontology(),
        "artifacts" => create_artifacts_ontology(),
        "heroes" => create_heroes_ontology(),
        "creatures" => create_creatures_ontology(),
        "locations" => create_locations_ontology(),
        "concepts" => create_concepts_ontology(),
        _ => {
            eprintln!("Unknown ontology: {}", ontology_name);
            print_usage();
            process::exit(1);
        }
    };
    
    match generate_html_visualization(&ontology, Path::new(&output_path)) {
        Ok(_) => println!("Visualization generated successfully!"),
        Err(e) => {
            eprintln!("Error generating visualization: {}", e);
            process::exit(1);
        }
    }
}