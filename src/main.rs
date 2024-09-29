use serde_json::{Value, json};
use std::fs::{self, File};
use std::io::{ Read, Write};
use std::env;
use std::path::Path;


// Return absolut path to the directory where the script is running.
fn get_current_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

// Retrun metadata content from input zarr file.
fn read_metadata(file_path: &str)  -> Value {
    
    let mut zarr_f = File::open(file_path).expect("Failed to open the input <.zarr> file.");
    
    let mut buffer = String::new();
    zarr_f.read_to_string(&mut buffer).expect("Faile to read the input <.zarr> file.");

    let meta: Value = serde_json::from_str(&mut buffer).expect("Failed to parse the input <.zarr> file.");

    meta
}


// Load an existing chunk from a zarr file.
fn load_chunk(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Unable to open chunk file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Unable to read chunk file");
    buffer
}

// Write a chunk of data to the zarr directory.
fn write_chunk(file_path: &str, chunk_data: &[u8]) {
    let mut file = File::create(file_path).expect("Unable to create chunk file");
    file.write_all(chunk_data).expect("Unable to write chunk file");
}

// Update the metadata of zarr file by new chunk size (.zarr file).
fn update_meta(file_path: &str, new_chunk_size: &[usize], metadata: &Value) {
    let new_metadata = json!({
        "chunks": new_chunk_size,
        // Copy other fields from original metadata
        "shape": metadata["shape"],
        "dtype": metadata["dtype"],
        "compressor": metadata["compressor"],
        "fill_value": metadata["fill_value"],
        "order": metadata["order"],
        "filters": metadata["filters"],
        "zarr_format": metadata["zarr_format"]
    });

    let new_metadata_str = serde_json::to_string_pretty(&new_metadata).expect("Unable to serialize new metadata");
    fs::write(file_path, new_metadata_str).expect("Unable to write new metadata");
}

// Main code to re-chunk zarr files by defining data_per_chunk.
fn re_chunk(input_file: &str, data_per_chunk: usize, output_path: &str) {

    let metadata_file_path = format!("{}/.zarray", input_file);
    let zarr_metadata = read_metadata(&metadata_file_path);

    println!("Zarr Metadata: {:#?}", zarr_metadata);

    let zarr_shape_org: Vec<usize> = zarr_metadata["shape"].as_array()
    .expect("Unable to specify the shape from metadata.")
    .iter()
    .map(|x| x.as_u64().expect("Invalid shape value.") as usize)
    .collect();

    let zarr_chunk_size_org: Vec<usize> = zarr_metadata["chunks"].as_array()
    .expect("Unable to specify the chunk size from metadata.")
    .iter()
    .map(|x| x.as_u64().expect("Invalid chunk size value.") as usize)
    .collect();

    let mut zarr_content = Vec::new();
    let chunk_paths = fs::read_dir(input_file).expect("Failed to read zarr files.");

    for el in chunk_paths {
        let file_path = el.unwrap().path();
        if file_path.is_file() && file_path.file_name().unwrap().to_str().unwrap() != ".zarray" {
            let chunk_data = load_chunk(&file_path.to_str().unwrap());
            zarr_content.extend_from_slice(&chunk_data);
        }
    }

    let allowed_size = zarr_shape_org[0] * zarr_chunk_size_org[1];
    let custom_size = data_per_chunk * zarr_chunk_size_org[1];
    
    let mut chunk_suffixe = 0;
    let mut idx  = 0;

    while idx < allowed_size {
        let last_itr = (idx + custom_size).min(allowed_size);
        let new_chunk = &zarr_content[idx..last_itr];
        let new_chunk_name = format!("{}/{}.0", output_path, chunk_suffixe);

        // println!("{}: {} bytes", new_chunk_name, new_chunk.len());

        write_chunk(&new_chunk_name, &new_chunk);

        idx += custom_size;
        chunk_suffixe += 1;
    }

    let new_metadata_path = format!("{}/.zarray", output_path);
    update_meta(
        &new_metadata_path,
        &[data_per_chunk, zarr_chunk_size_org[1]],
        &zarr_metadata);

}

fn main() {
    let in_input_file = "/test/data/potsdam_supermarkets.zarr";
    let in_output_dir: &str = "/test/results";
    let data_per_chunk = 5;

    let current_dir  = get_current_dir();

    let abs_input_file = format!("{}{}", current_dir, in_input_file);
    let abs_output_dir = format!("{}{}", current_dir, in_output_dir);

    let abs_input_file_str = abs_input_file.as_str();
    let abs_output_dir_str = abs_output_dir.as_str();

    println!("\n==============================>");
    println!("Rechunk zarr dataset...");
    println!("Path :: Input file => {}", format!("{}", abs_input_file_str));
    println!("Path :: Output directory => {}", format!("{}", abs_output_dir_str));
    println!("Data per chunk => {}", data_per_chunk);
    

    if !Path::new(abs_input_file_str).exists() {
        println!("File not found: {}", abs_input_file);
        std::process::exit(1)
    }

    if !Path::new(abs_output_dir_str).exists() {
        fs::create_dir_all(abs_output_dir_str)
        .expect("Output dir not found.");
    }

    let output_basename = Path::new(abs_input_file_str)
        .file_stem()
        .expect("No file name!")
        .to_str()
        .unwrap();

    let re_chunked_dir = format!(
        "{}/{}_re_chunk_{}.zarr",
        abs_output_dir_str,
        output_basename,
        data_per_chunk);

    let abs_output_path = &re_chunked_dir;

    if Path::new(abs_output_path).exists() {
        println!("Directory {} already exists", abs_output_path);
        std::process::exit(1)
    } else {
        fs::create_dir_all(abs_output_path)
        .expect("Failed to create re-chunked directory.");
    }
    
    re_chunk(abs_input_file_str, data_per_chunk, abs_output_path);
    println!("Done successfully!");
    println!("Stored in => {}", re_chunked_dir);
    println!("==============================>\n");

}
