use svg::{Document, node::element::{Path, path::Data}};
use hexmap::constants::*;


pub const PADDING: isize = 30;
pub const SIZE: isize = 200;

pub fn main()
{
    draw_flat_top_hex();
    draw_pointy_top_hex();
}


/// Draws an SVG image of a flat-topped hexagon with some guidelines to make sure the corners all align correctly
fn draw_flat_top_hex()
{
    let corners: Vec<(f32, f32)> = FLAT_TOP_CORNERS
        .iter()
        .map(|(x, y)| { (x * SIZE as f32, y * SIZE as f32) })
        .collect();

    let hex_data = Data::new()
        .move_to(corners[0])
        .line_to(corners[1])
        .line_to(corners[2])
        .line_to(corners[3])
        .line_to(corners[4])
        .line_to(corners[5])
        .line_to(corners[0])
        .close();

    let hex_path = Path::new()
        .set("fill", "white")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", hex_data);

    let image_width = (FLAT_TOP_WIDTH * SIZE as f32) as isize + PADDING;
    let image_height = (FLAT_TOP_HEIGHT * SIZE as f32) as isize + PADDING;

    let grid_data = Data::new()
        // vertical lines
        .move_to((-FLAT_TOP_WIDTH * SIZE as f32 / 2.0, image_height as f32 / 2.0))
        .line_to((-FLAT_TOP_WIDTH * SIZE as f32 / 2.0, -image_height as f32 / 2.0))
        .move_to((-FLAT_TOP_WIDTH * SIZE as f32 / 4.0, image_height as f32 / 2.0))
        .line_to((-FLAT_TOP_WIDTH * SIZE as f32 / 4.0, -image_height as f32 / 2.0))
        .move_to((0, image_height as f32 / 2.0))
        .line_to((0, -image_height as f32 / 2.0))
        .move_to((FLAT_TOP_WIDTH * SIZE as f32 / 4.0, image_height as f32 / 2.0))
        .line_to((FLAT_TOP_WIDTH * SIZE as f32 / 4.0, -image_height as f32 / 2.0))
        .move_to((FLAT_TOP_WIDTH * SIZE as f32 / 2.0, image_height as f32 / 2.0))
        .line_to((FLAT_TOP_WIDTH * SIZE as f32 / 2.0, -image_height as f32 / 2.0))
        // horizontal lines
        .move_to((image_width as f32 / 2.0, -FLAT_TOP_HEIGHT * SIZE as f32 / 2.0))
        .line_to((-image_width as f32 / 2.0, -FLAT_TOP_HEIGHT * SIZE as f32 / 2.0))
        .move_to((image_width as f32 / 2.0, -FLAT_TOP_HEIGHT * SIZE as f32 / 4.0))
        .line_to((-image_width as f32 / 2.0, -FLAT_TOP_HEIGHT * SIZE as f32 / 4.0))
        .move_to((image_width as f32 / 2.0, 0.0))
        .line_to((-image_width as f32 / 2.0, 0.0))
        .move_to((image_width as f32 / 2.0, FLAT_TOP_HEIGHT * SIZE as f32 / 4.0))
        .line_to((-image_width as f32 / 2.0, FLAT_TOP_HEIGHT * SIZE as f32 / 4.0))
        .move_to((image_width as f32 / 2.0, FLAT_TOP_HEIGHT * SIZE as f32 / 2.0))
        .line_to((-image_width as f32 / 2.0, FLAT_TOP_HEIGHT * SIZE as f32 / 2.0));

    let grid_path = Path::new()
        .set("d", grid_data)
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("stroke-dasharray", (6, 3));

    let document = Document::new()
        .set("viewBox", (-image_width / 2, -image_height / 2, image_width, image_height))
        .add(hex_path)
        .add(grid_path);

    svg::save("examples/images/flat_top.svg", &document).unwrap();
}


/// Draws an SVG image of a pointy-topped hexagon with some guide lines to ensure that the corners all line up as expected.
fn draw_pointy_top_hex()
{
    let corners: Vec<(f32, f32)> = POINTY_TOP_CORNERS
        .iter()
        .map(|(x, y)| { (x * SIZE as f32, y * SIZE as f32) })
        .collect();

    let hex_data = Data::new()
        .move_to(corners[0])
        .line_to(corners[1])
        .line_to(corners[2])
        .line_to(corners[3])
        .line_to(corners[4])
        .line_to(corners[5])
        .line_to(corners[0])
        .close();

    let hex_path = Path::new()
        .set("fill", "white")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", hex_data);

    let image_width = (POINTY_TOP_WIDTH * SIZE as f32) as isize + PADDING;
    let image_height = (POINTY_TOP_HEIGHT * SIZE as f32) as isize + PADDING;

    let grid_data = Data::new()
        // vertical lines
        .move_to((-POINTY_TOP_WIDTH * SIZE as f32 / 2.0, image_height as f32 / 2.0))
        .line_to((-POINTY_TOP_WIDTH * SIZE as f32 / 2.0, -image_height as f32 / 2.0))
        .move_to((-POINTY_TOP_WIDTH * SIZE as f32 / 4.0, image_height as f32 / 2.0))
        .line_to((-POINTY_TOP_WIDTH * SIZE as f32 / 4.0, -image_height as f32 / 2.0))
        .move_to((0, image_height as f32 / 2.0))
        .line_to((0, -image_height as f32 / 2.0))
        .move_to((POINTY_TOP_WIDTH * SIZE as f32 / 4.0, image_height as f32 / 2.0))
        .line_to((POINTY_TOP_WIDTH * SIZE as f32 / 4.0, -image_height as f32 / 2.0))
        .move_to((POINTY_TOP_WIDTH * SIZE as f32 / 2.0, image_height as f32 / 2.0))
        .line_to((POINTY_TOP_WIDTH * SIZE as f32 / 2.0, -image_height as f32 / 2.0))
        // horizontal lines
        .move_to((image_width as f32 / 2.0, -POINTY_TOP_HEIGHT * SIZE as f32 / 2.0))
        .line_to((-image_width as f32 / 2.0, -POINTY_TOP_HEIGHT * SIZE as f32 / 2.0))
        .move_to((image_width as f32 / 2.0, -POINTY_TOP_HEIGHT * SIZE as f32 / 4.0))
        .line_to((-image_width as f32 / 2.0, -POINTY_TOP_HEIGHT * SIZE as f32 / 4.0))
        .move_to((image_width as f32 / 2.0, 0.0))
        .line_to((-image_width as f32 / 2.0, 0.0))
        .move_to((image_width as f32 / 2.0, POINTY_TOP_HEIGHT * SIZE as f32 / 4.0))
        .line_to((-image_width as f32 / 2.0, POINTY_TOP_HEIGHT * SIZE as f32 / 4.0))
        .move_to((image_width as f32 / 2.0, POINTY_TOP_HEIGHT * SIZE as f32 / 2.0))
        .line_to((-image_width as f32 / 2.0, POINTY_TOP_HEIGHT * SIZE as f32 / 2.0));

    let grid_path = Path::new()
        .set("d", grid_data)
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("stroke-dasharray", (6, 3));

    let document = Document::new()
        .set("viewBox", (-image_width / 2, -image_height / 2, image_width, image_height))
        .add(hex_path)
        .add(grid_path);

    svg::save("examples/images/pointy_top.svg", &document).unwrap();
}