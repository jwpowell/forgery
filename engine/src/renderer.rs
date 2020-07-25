use std::convert::From;
use std::error::Error;
use std::fmt;

use crate::cell::Cell;
use crate::layout::Layout;
use crate::log;
use crate::world::WorldMap;

use web_sys::{Document, Element};

fn get_document() -> Result<Document, RenderError> {
    // Use `web_sys`'s global `window` function to get a handle on the global window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    //let body = document.body().expect("document should have a body");

    Ok(document)
}

fn get_target(document: &Document, target_id: &str) -> Result<Element, RenderError> {
    Ok(document
        .get_element_by_id(target_id)
        .expect(format!("target_id does not exist: {}", target_id).as_str()))
}

#[derive(Debug)]
pub struct RenderError {
    details: String,
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error rendering: {}", self.details)
    }
}

impl Error for RenderError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<wasm_bindgen::JsValue> for RenderError {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        let details = match value.as_string() {
            Some(v) => v,
            None => "could not stringify JsValue".into(),
        };
        RenderError { details }
    }
}

impl From<RenderError> for wasm_bindgen::JsValue {
    fn from(err: RenderError) -> Self {
        wasm_bindgen::JsValue::from_str(err.details.as_str())
    }
}

pub trait Renderer {
    fn render(&self, target_id: &str) -> Result<(), RenderError>;
}

impl<T: Cell, L: Layout<T = T>> Renderer for WorldMap<T, L> {
    fn render(&self, target_id: &str) -> Result<(), RenderError> {
        log::debug(format!("rendering WorldMap to {:?}", target_id));

        let document = get_document()?;
        let target = get_target(&document, target_id)?;

        // Clear the target of any existing elements.
        target.set_inner_html("");

        // Add the root svg container.
        let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
        svg.set_attribute("viewBox", "-250 -250 500 500")?;

        for (_, cell) in &self.map {
            self.render_cell(&document, &svg, &cell)?;
        }

        target.append_child(&svg)?;

        Ok(())
    }
}

impl<T: Cell, L: Layout<T = T>> WorldMap<T, L> {
    fn render_cell(
        &self,
        document: &Document,
        target: &Element,
        cell: &T,
    ) -> Result<(), RenderError> {
        //let g = document.create_element("g");
        //let pixel = layout.hexToPixel(hex);
        //g.setAttribute("transform", "translate(" + pixel.x + "," + pixel.y + ")");

        let polygon = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "polygon")?;
        let corners = self.layout.polygon_corners(cell);

        let mut corners_string: String = "".to_owned();
        for corner in corners {
            corners_string.push_str(String::from(corner).as_str());
            corners_string.push_str(" ");
        }

        polygon.set_attribute("points", corners_string.as_str())?;
        polygon.set_attribute("style", "fill:lime;stroke:purple;stroke-width:1")?;

        let title = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "title")?;
        title.set_inner_html(String::from(cell.coords()).as_str());

        //polygon.setAttribute("transform", "translate(" + pixel.x + "," + pixel.y + ")");

        polygon.append_child(&title)?;

        target.append_child(&polygon)?;

        Ok(())
    }
}
/*
class HexMapRenderer {
    constructor(target) {
        READONLY(this, "target", target);
    }

    draw(layout, hexMap) {
        DEBUG && ASSERT_INSTANCE_OF(hexMap, HexMap);

        const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
        svg.setAttribute("viewBox", "-250 -250 500 500");

        const hexRenderer = new HexRenderer(svg);

        hexMap.forEach(function (hex) {
            hexRenderer.draw(layout, hex);
        });


        this.target.innerHTML = "";
        this.target.appendChild(svg);
    }
}

class HexRenderer {

    constructor(target) {
        READONLY(this, "target", target);
    }

    draw(layout, hex) {
        DEBUG && ASSERT_INSTANCE_OF(hex, Hex);

        const g = document.createElement("g");
        const pixel = layout.hexToPixel(hex);
        //g.setAttribute("transform", "translate(" + pixel.x + "," + pixel.y + ")");

        const polygon = document.createElementNS("http://www.w3.org/2000/svg", "polygon");
        const corners = layout.polygonCorners(hex);
        let points = "";
        for (let i = 0; i < corners.length; ++i) {
            points += corners[i].x + "," + corners[i].y + " ";
        }
        polygon.setAttribute("points", points);
        polygon.setAttribute("style", "fill:lime;stroke:purple;stroke-width:1");

        const title = document.createElementNS("http://www.w3.org/2000/svg", "title");
        title.innerHTML = hex.hashCode();

        //polygon.setAttribute("transform", "translate(" + pixel.x + "," + pixel.y + ")");

        polygon.appendChild(title);


        this.target.appendChild(polygon);
    }

}
*/
