use geo::prelude::{BoundingRect, Contains};
use gre::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(seed: u8) -> Vec<Group> {
    let voronoi_size = 900;
    let max_samples = 80;
    let samples_r = 0.04;
    let res = 80;
    let width = 210.;
    let height = 297.;
    let poly_threshold = 0.5;

    let project =
        |(x, y): (f64, f64)| (x * width, y * height);

    let mut rng = SmallRng::from_seed([
        seed, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    let candidates = sample_2d_candidates_f64(
        &|p| {
            let d = euclidian_dist(p, (0.5, 0.5));
            1. - 1.5 * d
        },
        800,
        voronoi_size,
        &mut rng,
    );

    let mut polys =
        sample_square_voronoi_polys(candidates, 0.02);

    // filter out big polygons (by their "squared" bounds)
    polys.retain(|poly| {
        poly_bounding_square_edge(poly) < poly_threshold
    });

    let get_color =
        image_get_color("images/bitcoin_portrait.png")
            .unwrap();

    let get = |p| {
        let c = get_color(p);
        smoothstep(1.0, 0.0, c.1)
    };

    let mut data = Data::new();

    let routes: Vec<Vec<(f64, f64)>> = polys
        .par_iter()
        .map(|poly| {
            let mut rng = SmallRng::from_seed([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0,
            ]);
            let bounds = poly.bounding_rect().unwrap();
            let min = bounds.min();
            let width = bounds.width();
            let height = bounds.height();
            let map_p = |(lx, ly)| {
                (min.x + width * lx, min.y + height * ly)
            };
            let mut candidates = sample_2d_candidates_f64(
                &|p| {
                    let ap = map_p(p);
                    if poly.contains(&geo::Point::new(
                        ap.0, ap.1,
                    )) {
                        samples_r * get(ap)
                    } else {
                        0.0
                    }
                },
                res,
                max_samples,
                &mut rng,
            );
            candidates = candidates
                .iter()
                .map(|&p| project(map_p(p)))
                .collect();
            if candidates.len() < 5 {
                vec![]
            } else {
                candidates
            }
        })
        .collect();

    let should_draw_line =
        |a: (f64, f64), b: (f64, f64)| {
            let dx = a.0 - b.0;
            let dy = a.1 - b.1;
            (dx * dx + dy * dy).sqrt() < 20.0
        };

    for route in routes {
        data = render_route_when(
            data,
            route,
            &should_draw_line,
        );
    }

    vec![Group::new().add(
        layer("black").add(base_path("black", 0.2, data)),
    )]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(10);
    let groups = art(seed);
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (175.0, 280.0),
        "black",
    ));
    svg::save("image.svg", &document).unwrap();
}
