use svg::{node::{element::{path::Data, Path, Text}}, Document};
use hexmap::{constants::*, HexOrientation, AxialCoords, axial};


const HEX_SIZE: f32 = 120.0;
const PADDING: f32 = 10.0;
const ORIENTATION: HexOrientation = HexOrientation::PointyTop;


fn main()
{
    let hexes = vec![axial!(0, 0), axial!(1, 0), axial!(-1, 0), axial!(0, 1), axial!(0, -1), axial!(1, 1), axial!(1, -1), axial!(-1, 1), axial!(-1, -1)];
    let mut document = Document::new();
    let mut max_x = 0.0;
    let mut max_y = 0.0;
    for hex in hexes
    {
        // get coords
        let (x, y) = hex.to_world(ORIENTATION);
        // draw hex
        let data = draw_hex((x, y), ORIENTATION);
        let path = Path::new()
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("fill", "white")
            .set("d", data);
        // draw text
        let text = Text::new()
            .set("x", x * HEX_SIZE)
            .set("y", y * HEX_SIZE)
            .set("text-anchor", "middle")
            .set("text-align", "center")
            .set("font-size", 10)
            .add(svg::node::Text::new(format!("x: {:.2}, y: {:.2}, q: {}, r: {}</tspan>", x, y, hex.q, hex.r)));
        document = document.add(path).add(text);
        // update max image dimensions
        if x.abs() > max_x { max_x = x.abs() }
        if y.abs() > max_y { max_y = y.abs() }
    }

    let scaled_width = max_x * HEX_SIZE * 2.0 + HEX_SIZE * 2.0;
    let scaled_height = max_y * HEX_SIZE * 2.0 + HEX_SIZE * 2.0;
    let image_width = scaled_width + PADDING * 2.0;
    let image_height = scaled_height + PADDING * 2.0;
    let document = document.set("viewBox", (-image_width / 2.0, -image_height / 2.0, image_width, image_height));

    svg::save("examples/images/hexes.svg", &document).unwrap();
}


fn draw_hex(center: (f32, f32), orientation: HexOrientation) -> Data
{
    let (cx, cy) = center;
    let cx = cx * HEX_SIZE;
    let cy = cy * HEX_SIZE;
    let points = match orientation
    {
        HexOrientation::FlatTop => FLAT_TOP_CORNERS,
        HexOrientation::PointyTop => POINTY_TOP_CORNERS,
    };
    let mut data = Data::new()
        .move_to((points[5].0 * HEX_SIZE + cx, points[5].1 * HEX_SIZE + cy));
    for (x, y) in points
    {
        let px = x * HEX_SIZE + cx;
        let py = y * HEX_SIZE + cy;
        data = data.line_to((px, py))
    }
    data
}
