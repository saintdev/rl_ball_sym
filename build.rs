use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

fn read_ids(data: &[u8]) -> Vec<i32> {
    data.chunks_exact(std::mem::size_of::<i32>()).map(|buf| i32::from_le_bytes(buf.try_into().unwrap())).collect()
}

fn read_vertices(data: &[u8]) -> Vec<f32> {
    data.chunks_exact(std::mem::size_of::<f32>()).map(|buf| f32::from_le_bytes(buf.try_into().unwrap())).collect()
}

fn write_table_i32(f: &mut impl Write, name: &str, table: &[i32]) {
    write!(f, "pub(crate) static {}: &[i32] = &[", name.to_uppercase()).unwrap();
    for (i, value) in table.iter().enumerate() {
        if i % 20 == 0 {
            write!(f, "\n    ").unwrap();
        }
        write!(f, "{:#X}i32, ", value).unwrap();
    }
    writeln!(f).unwrap();
    writeln!(f, "];").unwrap();
}

fn write_table_f32(f: &mut impl Write, name: &str, table: &[f32]) {
    write!(f, "pub(crate) static {}: &[f32] = &[", name.to_uppercase()).unwrap();
    for (i, value) in table.iter().enumerate() {
        if i % 20 == 0 {
            write!(f, "\n    ").unwrap();
        }
        write!(f, "{}f32,", value).unwrap();
    }
    writeln!(f).unwrap();
    writeln!(f, "];").unwrap();
}

pub fn main() {
    let mut mesh_bins = HashMap::new();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("mesh_tables.rs");
    let dest_file = File::create(dest_path).unwrap();
    let mut dest_file = BufWriter::new(dest_file);
    let asset_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/"));

    for mode_dir in asset_dir.read_dir().unwrap() {
        let mode_dir = mode_dir.unwrap().path();
        if !mode_dir.is_dir() {
            continue;
        }

        for bin_file in mode_dir.read_dir().unwrap() {
            let bin_file = bin_file.unwrap().path();
            if !bin_file.is_file() {
                continue;
            }
            if bin_file.extension().map(|ext| ext != "bin").unwrap_or(true) {
                continue;
            }

            println!("cargo:rerun-if-changed={}", bin_file.strip_prefix(env!("CARGO_MANIFEST_DIR")).unwrap().to_string_lossy());
            let base_name = format!("{}", bin_file.file_stem().unwrap().to_string_lossy());
            mesh_bins.entry(base_name).or_insert_with(|| std::fs::read(&bin_file).unwrap());
        }
    }

    for (k, data) in &mesh_bins {
        if k.ends_with("_ids") {
            let ids = read_ids(data);
            write_table_i32(&mut dest_file, k, &ids)
        }
        if k.ends_with("_vertices") {
            let vertices = read_vertices(data);
            write_table_f32(&mut dest_file, k, &vertices);
        }
    }
    dest_file.flush().unwrap();
}
