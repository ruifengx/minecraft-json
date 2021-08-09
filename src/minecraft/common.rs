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
use serde_json::Number;

/// Union of two JSON deserializable objects.
#[derive(Eq, PartialEq, Debug)]
#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Default(bound = "L: Default"))]
#[serde(untagged)]
pub enum Either<L, R> {
    /// Left branch. Defaulted.
    #[derivative(Default)]
    Left(L),
    /// Right branch.
    Right(R),
}

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
pub type Ranged<I> = Ranged2<I, I>;

/// Ranged values for use in conditions, with possibly different types for exact and ranged values.
#[derive(Eq, PartialEq, Debug)]
#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Default)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Ranged2<I, N> {
    #[derivative(Default)]
    Exact(I),
    Range {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min: Option<N>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<N>,
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

/// Constant number provider as raw number or JSON object.
pub type NumberProviderValue<I> = Either<I, Box<NumberProvider<I>>>;

/// Loot tables use number providers in some places that accept an int or float. They can either
/// be defined as a constant value or as an object.
#[derive(Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NumberProvider<I> {
    /// A constant value.
    Constant {
        /// The exact value.
        value: I,
    },
    /// A random number following a uniform distribution between two values (inclusive).
    Uniform {
        /// Number provider. The minimum value.
        min: Box<NumberProviderValue<I>>,
        /// Number provider. The maximum value.
        max: Box<NumberProviderValue<I>>,
    },
    /// A random number following a binomial distribution.
    Binomial {
        /// Number provider. The amount of trials.
        min: Box<NumberProviderValue<isize>>,
        /// Number provider. The probability of success on an individual trial.
        max: Box<NumberProviderValue<Number>>,
    },
    /// A scoreboard value.
    Score {
        /// Scoreboard name provider.
        target: ScoreboardNameProvider,
        /// The scoreboard objective.
        score: String,
        /// Optional. Scale to multiply the score before returning it.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scale: Option<Number>,
    },
}

/// Scoreboard name provider.
pub type ScoreboardNameProvider = Either<ScoreboardName, ScoreboardSelector>;

/// Scoreboard name provider.
#[derive(Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum ScoreboardName {
    This,
    Killer,
    DirectKiller,
    PlayerKiller,
}

/// Scoreboard name provider.
#[derive(Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum ScoreboardSelector {
    Fixed {
        /// A UUID or playername.
        name: String,
    },
    Context {
        /// Scoreboard name provider.
        target: ScoreboardName,
    },
}
