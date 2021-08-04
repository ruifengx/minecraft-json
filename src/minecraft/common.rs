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

//! Common data types for use in Minecraft JSON.

use derivative::Derivative;
use serde::{Serialize, Deserialize};

/// A 3D vector, for positions, velocities, etc.
#[derive(Eq, PartialEq, Debug)]
#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Default(bound = ""))]
#[allow(missing_docs)]
pub struct Vector3d<I> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<I>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<I>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<I>,
}

/// Ranged values for use in conditions.
#[derive(Eq, PartialEq, Debug)]
#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Default)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Ranged<I> {
    #[derivative(Default)]
    Exact(I),
    Range {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min: Option<I>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<I>,
    },
}

/// Plain values for use in block states or fluid states.
#[derive(Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum PlainValue {
    Boolean(bool),
    Integer(Ranged<isize>),
    String(String),
}
