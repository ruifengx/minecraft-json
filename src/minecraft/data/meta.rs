/*
 * minecraft-json: processing Minecraft JSON data
 * Copyright (C) 2021  Xie Ruifeng
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

//! `pack.mcmeta` files.

use serde::{Serialize, Deserialize};

use crate::minecraft::text::TextComponent;

/// A data pack is identified by Minecraft based on the presence of the `pack.mcmeta` file in the
/// root directory of the data pack, which contains data in JSON format.
#[derive(Eq, PartialEq, Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum McMeta {
    /// Holds the data pack information.
    Pack {
        /// A [JSON text](TextComponent) that appears when hovering over the data pack's name in
        /// the list given by the `/datapack list` command, or when viewing the pack in the Create
        /// World screen.
        description: TextComponent,
        /// Pack version: If this number does not match the current required number, the data
        /// pack displays a warning and requires additional confirmation to load the pack.
        pack_format: i64,
    }
}

/// Known `pack_format` field for [`McMeta`].
pub mod pack_format {
    /// Minecraft 1.13–1.14.4.
    pub const VER_4: i64 = 4;
    /// Minecraft 1.15–1.16.1.
    pub const VER_5: i64 = 5;
    /// Minecraft 1.16.2–1.16.5.
    pub const VER_6: i64 = 6;
    /// Minecraft 1.17.
    pub const VER_7: i64 = 7;
}
