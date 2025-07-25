//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

use std::fs;

use anyhow::Result;

use sel4_capdl_initializer_types::{Footprint, InputSpec};

mod args;
mod render_elf;
mod reserialize_spec;

use args::Args;

// HACK hardcoded
const GRANULE_SIZE_BITS: usize = 12;

fn main() -> Result<()> {
    let args = Args::parse()?;

    if args.verbose {
        eprintln!("{args:#?}");
    }

    let initializer_elf_buf = fs::read(&args.initializer_elf_path)?;
    let spec_json = fs::read_to_string(&args.spec_json_path)?;
    let fill_dir_path = &args.fill_dir_path;
    let out_file_path = &args.out_file_path;
    let object_names_level = &args.object_names_level;
    let embed_frames = args.embed_frames;

    let input_spec = InputSpec::parse(&spec_json);

    let (final_spec, serialized_spec) = reserialize_spec::reserialize_spec(
        &input_spec,
        fill_dir_path,
        object_names_level,
        embed_frames,
        GRANULE_SIZE_BITS,
        args.verbose,
    );

    let footprint = final_spec.total_footprint();

    // TODO make configurable
    let heap_size = footprint * 2 + 16 * 4096;

    if args.verbose {
        eprintln!("footprint: {footprint}");
        eprintln!("heap size: {}", heap_size / 4096);
    }

    let render_elf_args = render_elf::RenderElfArgs {
        data: &serialized_spec,
        granule_size_bits: GRANULE_SIZE_BITS,
        heap_size,
    };

    let rendered_initializer_elf_buf = match object::File::parse(&*initializer_elf_buf).unwrap() {
        object::File::Elf32(initializer_elf) => render_elf_args.call_with(&initializer_elf),
        object::File::Elf64(initializer_elf) => render_elf_args.call_with(&initializer_elf),
        _ => {
            panic!()
        }
    };

    fs::write(out_file_path, rendered_initializer_elf_buf)?;
    Ok(())
}
