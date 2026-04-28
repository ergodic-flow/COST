extern crate COST;

use std::fs::File;

use COST::graph_iterator::{EdgeMapper, DeltaCompressedReaderMapper, NodesEdgesMemMapper, UpperLowerMemMapper };
use std::io::BufReader;


fn main() {
    if std::env::args().len() != 4 {
        println!("Usage: hits (vertex | hilbert | compressed) <prefix> nodes");
        return;
    }

    let mode = std::env::args().nth(1).expect("mode unavailable");
    let name = std::env::args().nth(2).expect("name unavailable");
    let nodes: u32 = std::env::args().nth(3).expect("nodes unavailable").parse().expect("nodes not parseable");

    match mode.as_str() {
        "vertex" => hits(&NodesEdgesMemMapper::new(&name), nodes),
        "hilbert" => hits(&UpperLowerMemMapper::new(&name), nodes),
        "compressed" => hits(&DeltaCompressedReaderMapper::new(|| BufReader::new(File::open(&name).unwrap())), nodes),
        _ => println!("unrecognized mode: {:?}", mode),
    }
}

fn hits<G: EdgeMapper>(graph: &G, nodes: u32) {
    let timer = std::time::Instant::now();

    let mut hub = vec![1f32; nodes as usize];
    let mut auth = vec![0f32; nodes as usize];

    for iteration in 0..20 {
        println!("Iteration {}:\t{:?}", iteration, timer.elapsed());

        // Reset Authorities
        auth.fill(0.0);
        graph.map_edges(|x, y| unsafe {
            *auth.get_unchecked_mut(y as usize) += *hub.get_unchecked(x as usize);
        });

        // Reset Hubs
        hub.fill(0.0);
        graph.map_edges(|x, y| unsafe {
            *hub.get_unchecked_mut(x as usize) += *auth.get_unchecked(y as usize);
        });

        // Normalization (L-infinity)
        let max_auth = auth.iter().cloned().fold(0f32, f32::max);
        let max_hub = hub.iter().cloned().fold(0f32, f32::max);

        if max_auth > 0.0 {
            for val in auth.iter_mut() { *val /= max_auth; }
        }
        if max_hub > 0.0 {
            for val in hub.iter_mut() { *val /= max_hub; }
        }
    }
}
