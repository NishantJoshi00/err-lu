use std::collections::HashMap;

use anyhow::Context;

#[derive(serde::Deserialize, serde::Serialize)]
struct Record {
    connector: String,
    error_code: String,
    category: String,
}

fn main() -> anyhow::Result<()> {
    let source_map = std::env::var("SOURCE_MAP").context("`SOURCE_MAP` env var not set")?;
    println!("cargo:rerun-if-changed={}", source_map);
    let source_map = std::path::PathBuf::from(source_map);

    let out_dir = std::env::var("OUT_DIR").context("Failed to get `OUT_DIR` from build system")?;
    let out_dir = std::path::PathBuf::from(out_dir);

    let io_reader = std::fs::File::open(source_map).context("Failed to open source map file")?;

    let mut csv_reader = csv::Reader::from_reader(io_reader);

    let mut iter = csv_reader.deserialize::<Record>();

    let output = out_dir.join("mapping.bin");

    let io_writer = std::fs::File::create(output).context("Failed to create output file")?;

    let mapping = iter.try_fold(HashMap::new(), |mut map, element| {
        let element = element.context("Failed to deserialize record")?;

        let inner_map: &mut HashMap<_, _> = map.entry(element.connector).or_default();

        inner_map.insert(element.error_code, element.category);

        Ok::<_, anyhow::Error>(map)
    })?;

    bincode::serialize_into(io_writer, &mapping).context("Failed to serialize mapping")?;

    Ok(())
}
