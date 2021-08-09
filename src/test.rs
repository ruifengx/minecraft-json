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

/// Assert that a JSON string and some data is equivalent, i.e. [`serde_json::from_str`] and
/// [`serde_json::to_string`] converts them back and forth.
#[macro_export]
macro_rules! assert_equiv {
    ($lhs: expr, $rhs: expr) => {
        let __lhs = $lhs;
        let __rhs = &$rhs;
        assert_eq!(__lhs, serde_json::to_string(__rhs).unwrap());
        assert_eq!(*__rhs, serde_json::from_str(__lhs).unwrap());
    }
}

/// Assert that a JSON string and some data is equivalent, i.e. [`serde_json::from_str`] and
/// [`serde_json::to_string_pretty`] converts them back and forth.
#[macro_export]
macro_rules! assert_equiv_pretty {
    ($lhs: expr, $rhs: expr) => {
        let __lhs = $lhs;
        let __rhs = &$rhs;
        assert_eq!(__lhs, serde_json::to_string_pretty(__rhs).unwrap());
        assert_eq!(*__rhs, serde_json::from_str(__lhs).unwrap());
    }
}

/// Assert that a JSON string and some data is equivalent.
///
/// This version protects against `arbitrary_precision` by going via [`serde_json::Value`].
/// i.e. this macro asserts that (a) [`serde_json::from_value`] after [`serde_json::from_str`] and
/// (b) [`serde_json::to_string_pretty`] converts them back and forth.
#[macro_export]
macro_rules! assert_equiv_pretty_protected {
    ($lhs: expr, $rhs: expr) => {
        let __lhs = $lhs;
        let __rhs = &$rhs;
        assert_eq!(__lhs, serde_json::to_string_pretty(__rhs).unwrap());
        assert_eq!(*__rhs, serde_json::from_value(serde_json::from_str(__lhs).unwrap()).unwrap());
    }
}

/// Assert that a JSON string is NOT deserializable.
#[macro_export]
macro_rules! assert_cannot_deserialize {
    ($lhs: expr => $t: ty) => {
        assert!(serde_json::from_str::<$t>($lhs).is_err());
    }
}
