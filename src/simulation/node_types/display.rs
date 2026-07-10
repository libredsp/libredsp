use crate::simulation::{Node, Packet};
use std::fs::File;
use std::io::Write;

pub struct Display {
    id: Option<usize>,
    x: Vec<f64>,
    y: Vec<f64>,
    file: Option<File>,
}

impl Display {
    pub fn new() -> Self {
        Display {
            id: None,
            x: Vec::new(),
            y: Vec::new(),
            file: None,
        }
    }
    
    pub fn set_output_file(&mut self, path: &str) {
        self.file = Some(File::create(path).expect("Failed to create output file"));
        println!("Created the file {}", path);
    }
}

impl Node for Display {
    fn execute(&mut self, input: &[Packet]) -> Option<Packet> {
        let received_val: f64 = input.first().map(|p| p.output).unwrap_or(0.0);        

        self.y.push(received_val);
        if self.x.is_empty() {
            self.x.push(0.0);
        } else {
            self.x.push(*self.x.last().unwrap_or(&0.0) + 0.01);  // Increment time
        }
        
        /* Write to file */
        if let Some(ref mut file) = self.file {
            writeln!(file, "{}", received_val).expect("Failed to write to file");
        }
        
        None
    }
    
    fn get_display_name(&self) -> &str {
        "Display"
    }
    
    fn get_id(&self) -> usize {
        self.id.expect("ID not set.")
    }
    
    fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }
}