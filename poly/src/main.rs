fn main() {
    println!("Hello, world!");
}


// bool PointInPolygon(Point point, Polygon polygon) {
//   vector<Point> points = polygon.getPoints();
//   int i, j, nvert = points.size();
//   bool c = false;

//   for(i = 0, j = nvert - 1; i < nvert; j = i++) {
//     if( ( (points[i].y >= point.y ) != (points[j].y >= point.y) ) &&
//         (point.x <= (points[j].x - points[i].x) * (point.y - points[i].y) / (points[j].y - points[i].y) + points[i].x)
//       )
//       c = !c;
//   }

//   return c;
// }

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