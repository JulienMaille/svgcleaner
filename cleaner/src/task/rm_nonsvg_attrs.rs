// svgcleaner could help you to clean up your SVG files
// from unnecessary data.
// Copyright (C) 2012-2018 Evgeniy Reizner
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use svgdom::{
    Attribute,
    Document,
    Name,
    NodeType,
};

static PRESERVED_CUSTOM_ATTRIBUTES: &[&str] = &[
    // `vector-effect` is a valid presentation attribute, but svgdom 0.10.5
    // treats it as an unknown attribute. Keep it explicitly so users can rely
    // on `non-scaling-stroke` and friends even when removing non-SVG attrs.
    "vector-effect",
];

pub fn remove_nonsvg_attributes(doc: &Document) {
    for mut node in doc.descendants() {
        if node.node_type() != NodeType::Element {
            continue;
        }

        node.attributes_mut().retain(|attr| should_keep_attribute(attr));
    }
}

fn should_keep_attribute(attr: &Attribute) -> bool {
    attr.is_svg() || is_preserved_custom_attribute(attr)
}

fn is_preserved_custom_attribute(attr: &Attribute) -> bool {
    match attr.name {
        Name::Name(ref name) => PRESERVED_CUSTOM_ATTRIBUTES
            .iter()
            .any(|allowed| name.eq_ignore_ascii_case(allowed)),
        Name::Id(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svgdom::{Document, ToStringWithOptions};

    #[test]
    fn removes_unknown_attributes() {
        let doc = Document::from_str(
            "<svg custom='1'>\n    <g inkscape:label='test' custom='2'/>\n</svg>",
        ).unwrap();

        remove_nonsvg_attributes(&doc);

        assert_eq_text!(
            doc.to_string_with_opt(&write_opt_for_tests!()),
            "<svg>\n    <g/>\n</svg>\n"
        );
    }

    #[test]
    fn keeps_vector_effect_attribute() {
        let doc = Document::from_str(
            "<svg>\n    <path vector-effect='non-scaling-stroke'/>\n</svg>",
        ).unwrap();

        remove_nonsvg_attributes(&doc);

        assert_eq_text!(
            doc.to_string_with_opt(&write_opt_for_tests!()),
            "<svg>\n    <path vector-effect='non-scaling-stroke'/>\n</svg>\n"
        );
    }
}
