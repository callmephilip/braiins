// Copyright (C) 2019  Braiins Systems s.r.o.
//
// This file is part of Braiins Open-Source Initiative (BOSI).
//
// BOSI is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// Please, keep in mind that we may also license BOSI or any part thereof
// under a proprietary license. For more information on the terms and conditions
// of such proprietary license or if you have any other questions, please
// contact us at opensource@braiins.com.

pub mod client;
pub mod config;
pub mod entry;
pub mod error;
pub mod hal;
pub mod hub;
pub mod job;
pub mod node;
pub mod runtime_config;
pub mod shutdown;
pub mod stats;
pub mod work;

pub mod test_utils;

// reexport main function from `entry` module
pub use entry::main;
// reexport clap which is needed in `hal::Backend::add_args`
pub use clap;

use std::fmt;
use std::sync::Arc;

use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Frontend;

impl fmt::Display for Frontend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bOSminer")
    }
}

impl node::Info for Frontend {}

lazy_static! {
    /// Shared (global) configuration structure
    pub static ref BOSMINER: Arc<Frontend> = Arc::new(Frontend);
}