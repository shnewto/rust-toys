fn main() {
    println!("Hello, world!");
}


// Description:

// The problem

// In this kata, you're going write a function called pointInPoly to test if a point is inside a polygon.

// Points will be represented as [x,y] arrays.

// The polygon will be an array of points which are the polygon's vertices. The last point in the array connects back to the first point.

// You can assume:

// The polygon will be a valid simple polygon. That is, it will have at least three points, none of its edges will cross each other, and exactly two edges will meet at each vertex.
// In the tests, the point will never fall exactly on an edge of the polygon.

type Point = (f32, f32);

fn point_in_poly(poly: &[Point], point: Point) -> bool {

    let nvert = poly.len();
    let mut c = false;

    let mut j = nvert - 1;

    for i in 0..nvert {

        if (poly[i].1 >= point.1 ) != (poly[j].1 >= point.1) &&
        (point.0 <= (poly[j].0 - poly[i].0) * (point.1 - poly[i].1) / (poly[j].1 - poly[i].1) + poly[i].0) {
            c = !c;
        }

        j = i;
    }

  c

}

#[test]
fn simple_square() {
  let poly = [(-5., -5.), (5., -5.),
              (5., 5.), (-5., 5.)];
  show_and_test(&poly, (-6., 0.), false);
  show_and_test(&poly, (-1., 1.), true);
}