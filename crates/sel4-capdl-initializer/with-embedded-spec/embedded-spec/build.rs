//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

use std::env;
use std::fs;
use std::path::PathBuf;

use sel4_capdl_initializer_with_embedded_spec_build_env::get_embedding;

fn main() {
    // TODO
    // let deflate_fill = cfg!(feature = "deflate");

    let (embedding, footprint) = get_embedding();

    footprint.tell_cargo();

    let (embedded_spec, aux_files) = embedding.embed();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    {
        let formatted = prettyplease::unparse(&syn::parse2(embedded_spec).unwrap());
        let spec_out_path = out_dir.join("spec.rs");
        fs::write(&spec_out_path, formatted).unwrap();
    }

    for (fname, content) in &aux_files {
        fs::write(out_dir.join(fname), content).unwrap();
    }
}
