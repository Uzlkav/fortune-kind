// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use std::io;

mod cli;
mod file;
mod fortune;
mod random;

fn main() -> io::Result<()> {
    let matches = cli::build_cli().get_matches();

    // Could be collapsed a little more, but leaving expanded for readability
    if let Some(pattern) = matches.get_one::<String>("find") {
        fortune::search_fortunes(pattern);
    } else if let Some(short) = matches.get_one::<u8>("short") {
        fortune::get_quote(short, matches.get_flag("equal"));
    } else if matches.get_flag("equal") {
        fortune::get_quote(&0, true);
    } else {
        fortune::get_quote(&0, false);
    }

    Ok(())
}
