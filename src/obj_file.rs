// A parser for the Wavefront OBJ 3D model format (chapter 15). Only the
// statements the ray tracer needs are recognized: `v` (vertex) and `f`
// (face) so far, with `g` (named group) routing faces into groups. Faces
// with more than three vertices are fan-triangulated, so only convex
// polygons are supported. Everything else is silently ignored.
//
// The tests target this API:
//
// - `parse_obj(source)` parses OBJ statements from a string and returns a
//   `Parser`. (Reading from an actual file can be layered on later, when
//   rendering downloaded models.)
// - `parser.ignored_lines` counts the unrecognized lines.
// - `parser.vertex(i)` returns a vertex by its 1-based OBJ index.
// - `parser.default_group()` and `parser.group(name)` return the triangles
//   collected into the default and named groups, as slices.
// - `parser.into_group()` consumes the parser and assembles the model into
//   a single `Group` shape: each non-empty group (the default group
//   included) becomes a child `Group` of triangles, in file order; empty
//   groups contribute nothing.

#[cfg(test)]
mod obj_file_tests {
    use crate::obj_file;
    use crate::shape;
    use crate::tuple;

    // The book's files/triangles.obj, used by the named-group tests.
    const TRIANGLES_OBJ: &str = "\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4
";

    #[test]
    fn test_ignoring_unrecognized_lines() {
        // The parser handles only a subset of the OBJ format, so it must
        // skip statements it doesn't recognize rather than choke on them.
        let gibberish = "\
There was a young lady named Bright
who traveled much faster than light.
She set out one day
in a relative way,
and came back the previous night.
";

        let parser = obj_file::parse_obj(gibberish);

        assert_eq!(parser.ignored_lines, 5);
    }

    #[test]
    fn test_vertex_records() {
        // A `v` statement is followed by three space-delimited numbers.
        // Note the indices: faces reference vertices 1-based, so the first
        // vertex in the file is vertex 1.
        let file = "\
v -1 1 0
v -1.0000 0.5000 0.0000
v 1 0 0
v 1 1 0
";

        let parser = obj_file::parse_obj(file);

        assert_eq!(parser.vertex(1), tuple::Point::new(-1.0, 1.0, 0.0));
        assert_eq!(parser.vertex(2), tuple::Point::new(-1.0, 0.5, 0.0));
        assert_eq!(parser.vertex(3), tuple::Point::new(1.0, 0.0, 0.0));
        assert_eq!(parser.vertex(4), tuple::Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_parsing_triangle_faces() {
        // An `f` statement names three vertices by their 1-based indices;
        // the resulting triangles land in the parser's default group.
        let file = "\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
f 1 2 3
f 1 3 4
";

        let parser = obj_file::parse_obj(file);

        let triangles = parser.default_group();
        assert_eq!(triangles.len(), 2);
        assert_eq!(
            triangles[0],
            shape::Shape::triangle(parser.vertex(1), parser.vertex(2), parser.vertex(3)),
        );
        assert_eq!(
            triangles[1],
            shape::Shape::triangle(parser.vertex(1), parser.vertex(3), parser.vertex(4)),
        );
    }

    #[test]
    fn test_triangulating_polygons() {
        // A face with more than three vertices describes a convex polygon,
        // which is broken up with a fan triangulation: every triangle
        // starts at the first vertex and takes the next two in the list.
        let file = "\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0
f 1 2 3 4 5
";

        let parser = obj_file::parse_obj(file);

        let triangles = parser.default_group();
        assert_eq!(triangles.len(), 3);
        assert_eq!(
            triangles[0],
            shape::Shape::triangle(parser.vertex(1), parser.vertex(2), parser.vertex(3)),
        );
        assert_eq!(
            triangles[1],
            shape::Shape::triangle(parser.vertex(1), parser.vertex(3), parser.vertex(4)),
        );
        assert_eq!(
            triangles[2],
            shape::Shape::triangle(parser.vertex(1), parser.vertex(4), parser.vertex(5)),
        );
    }

    #[test]
    fn test_triangles_in_named_groups() {
        // A `g` statement names a group; subsequent faces are added to the
        // most recently named group instead of the default group.
        let parser = obj_file::parse_obj(TRIANGLES_OBJ);

        let first = parser.group("FirstGroup");
        let second = parser.group("SecondGroup");
        assert_eq!(first.len(), 1);
        assert_eq!(
            first[0],
            shape::Shape::triangle(parser.vertex(1), parser.vertex(2), parser.vertex(3)),
        );
        assert_eq!(second.len(), 1);
        assert_eq!(
            second[0],
            shape::Shape::triangle(parser.vertex(1), parser.vertex(3), parser.vertex(4)),
        );
    }

    #[test]
    fn test_converting_an_obj_model_to_a_group() {
        // The whole model becomes one `Group` shape that can be added to a
        // scene: one child `Group` per non-empty group, in file order. The
        // default group is empty here (every face follows a `g` statement),
        // so it contributes no child.
        let parser = obj_file::parse_obj(TRIANGLES_OBJ);
        let mut expected_first = shape::Shape::default_group();
        expected_first.add_child(shape::Shape::triangle(
            parser.vertex(1),
            parser.vertex(2),
            parser.vertex(3),
        ));
        let mut expected_second = shape::Shape::default_group();
        expected_second.add_child(shape::Shape::triangle(
            parser.vertex(1),
            parser.vertex(3),
            parser.vertex(4),
        ));
        let mut expected = shape::Shape::default_group();
        expected.add_child(expected_first);
        expected.add_child(expected_second);

        let group = parser.into_group();

        assert_eq!(group, expected);
    }
}
