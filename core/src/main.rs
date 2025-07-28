/*
 * This file is part of Musmatic Core
 *
 * Musmatic Core is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Musmatic Core is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with Musmatic Core. If not, see <https://www.gnu.org/licenses/>.
 *
 * Copyright (c) 2019-2025 - Musmatic Core authors
 */

use std::io;
use std::io::Read;
use musmatic::render_mei_to_svg;


fn main(){
    println!("Loading");
    let mut buffer = Vec::new();

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_end(&mut buffer).unwrap();

    let mei_data = String::from_utf8(buffer).unwrap();
    println!("{}", render_mei_to_svg(mei_data));
}
