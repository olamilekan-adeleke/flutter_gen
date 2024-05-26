// Copyright (c) 2022, the Dart project authors.  Please see the AUTHORS file
// for details. All rights reserved. Use of this source code is governed by a
// BSD-style license that can be found in the LICENSE file.
//

// use crate::features;

use crate::features::bloc_gen::bloc_file_gen;

#[path = "../features/mod.rs"]
mod features;

fn main() {
    bloc_file_gen::init_bloc_gen();

    println!("Hello, world!");
}
