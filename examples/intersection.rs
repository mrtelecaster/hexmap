use hexmap::{AxialCoords, axial, HexOrientation, constants::POINTY_TOP_CORNERS};
use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use svg::{node::element::{Circle, path::Data, Path}, Document};

const SCALE: f32 = 400.0;
const SHOTS_WIDTH: f32 = 2.0;
const SHOTS: usize = 20000;
const ORIENTATION: HexOrientation = HexOrientation::PointyTop;


fn main()
{
    let hex_corners: Vec<(f32, f32)> = POINTY_TOP_CORNERS.iter().map(|(x, y)| { (x * SCALE, y * SCALE) }).collect();
    let mut rng = thread_rng();
    let hex_data = Data::new()
        .move_to(hex_corners[0])
        .line_to(hex_corners[1])
        .line_to(hex_corners[2])
        .line_to(hex_corners[3])
        .line_to(hex_corners[4])
        .line_to(hex_corners[5])
        .line_to(hex_corners[0]);
    let hex_path = Path::new()
        .set("d", hex_data)
        .set("stroke", "white")
        .set("stroke-width", "1px")
        .set("fill", "none");
    let distr = Normal::new(0.0, SHOTS_WIDTH).unwrap();
    let circle = Circle::new()
        .set("cx", 0)
        .set("cy", 0)
        .set("r", SCALE)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", "1px");
    let mut document = Document::new()
        .set("viewBox", (-SCALE * 3.0, -SCALE * 3.0, SCALE * 2.0 * 3.0, SCALE * 2.0 * 3.0))
        .add(circle)
        .add(hex_path);
    for _ in 0..SHOTS
    {
        let x = distr.sample(&mut rng);
        let y = distr.sample(&mut rng);
        let coords = AxialCoords::from_world(x, y, ORIENTATION);
        let color = if coords == axial!(0, 0) {
            "white"
        } else if coords == axial!(1, 0) {
            "red"
        } else if coords == axial!(0, 1) {
            "cyan"
        } else if coords == axial!(-1, 1) {
            "green"
        } else if coords == axial!(1, -1) {
            "magenta"
        } else if coords == axial!(0, -1) {
            "blue"
        } else if coords == axial!(-1, 0) {
            "yellow"
        } else {
            "black"
        };
        let cx = x * SCALE;
        let cy = y * SCALE;
        let r = 1.0;
        let circle = Circle::new()
            .set("cx", cx)
            .set("cy", cy)
            .set("r", r)
            .set("fill", color)
            .set("stroke", "none");
        document = document.add(circle);
    }
    svg::save("examples/images/bullseye.svg", &document);
}