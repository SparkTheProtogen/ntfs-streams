use std::fs::{File, OpenOptions, metadata};
use std::io::{Read, Write};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Mia", version = "TWO", about = "The possible modes are:\nf2s (file to stream)\ns2f (stream to file)\nms2f (mass stream to file, contents of --stream irrelevant)\nPUT ONLY STREAM NAMES in index.txt", long_about = "This is a utility to read and write to and from NTFS alternate file streams, use -h to see long about")]
struct Arguments {
	#[arg(short, long)]
	mode: String,
	
	#[arg(short, long)]
	stream: String,
	
	#[arg(short, long)]
	file: String,
}
fn main() {
 let args = Arguments::parse();

 match args.mode.as_str() {
	"f2s" => {file2stream(args.file, args.stream)},
	"s2f" => {stream2file(args.stream, args.file)},
	"ms2f" => {mass_decompress(args.file)},
	_ => {println!("You are an idiot!")}
	}
}
fn mass_decompress(filename: String) {
	// Check if the file exists
	let path = format!(".\\index.txt");
    // If the metadata is not None, the file exists
	
	
    if !metadata(path.clone()).is_ok() {
        println!("index.txt NOT present, creating now, put ONLY THE STREAM NAMES, seperated by newlines");
		if let Err(error) = File::create(path.clone()) {
			println!("Error: {}", error)
		};
	}
		let mut file = match File::open(path.clone()) {
		 Err(why) => panic!("oh no error: {}", why),
		 Ok(file) => file,
		};
		
	 let mut s = String::new();
     let _contents = match file.read_to_string(&mut s) {
         Err(why) => panic!("ERROR: cant read file: {}", why),
         Ok(cont) => cont,
		};
		
    let split_file = s.lines();
    let stream_vec: Vec<&str> = split_file.collect();
	
	
	for stream in stream_vec {
		let source = format!("{}:{}", filename, stream);
		stream2file(source, stream.to_string());
	}
	println!("FINISHED!!!!!");
}
// AI-written code below
fn file2stream(source_file: String, destination_stream: String) {
    

    // Read data from the source file
    let mut source_data = Vec::new();
    let mut file = File::open(source_file).expect("Failed to open source file");
    file.read_to_end(&mut source_data)
        .expect("Failed to read source file");

    // Write data to the destination file as an alternate file stream
    let mut destination_stream = OpenOptions::new()
        .create(true)
        .write(true)
        .open(destination_stream)
        .expect("Failed to open destination file");

    destination_stream
        .write_all(&source_data)
        .expect("Failed to write to destination file");

    println!("Data written to alternate file stream successfully.");
}
fn stream2file(source_stream: String, destination_file: String) {
	
	let dest_filename = destination_file.clone();
    // Read data from the source file as an alternate file stream
    let mut source_data = Vec::new();
    let mut file = OpenOptions::new()
        .read(true)
        .open(source_stream)
        .expect("Failed to open source file");

    file.read_to_end(&mut source_data)
        .expect("Failed to read source file");

    // Write data to the destination file
    let mut destination_file = File::create(destination_file)
        .expect("Failed to create destination file");

    destination_file
        .write_all(&source_data)
        .expect("Failed to write to destination file");

    println!("Data written to destination file ({}) successfully.", dest_filename);
}