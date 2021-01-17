use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use crate::engine::{debug, error, Cell, CellCoord, Layout, Point};

use web_sys::{Document, Element, Event, KeyEvent, MouseEvent, SvgsvgElement};

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;

// ------------ DOM ----------------------

thread_local! {
    static DOCUMENT: RefCell<Document> = RefCell::new(get_document().expect("failed to get document"));
}

const SVG_NS: Option<&'static str> = Some("http://www.w3.org/2000/svg");

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum UserEvent {
    MouseClick,
    MouseDown,
    MouseUp,
    MouseMove,
    MouseOver,
    MouseOut,
    KeyDown,
    KeyUp,
}

impl From<&UserEvent> for &str {
    fn from(event: &UserEvent) -> Self {
        match event {
            UserEvent::MouseClick => "click",
            UserEvent::MouseDown => "mousedown",
            UserEvent::MouseUp => "mouseup",
            UserEvent::MouseMove => "mousemove",
            UserEvent::MouseOver => "mouseover",
            UserEvent::MouseOut => "mouseout",
            UserEvent::KeyDown => "keydown",
            UserEvent::KeyUp => "keyup",
        }
    }
}

pub fn add_event<H>(el: &Element, user_event: &UserEvent, event_listener: H)
where
    H: 'static + FnMut(Event),
{
    let cl = Closure::wrap(Box::new(event_listener) as Box<dyn FnMut(_)>);
    el.add_event_listener_with_callback(user_event.into(), cl.as_ref().unchecked_ref())
        .unwrap();
    cl.forget();
}

pub fn add_mouse_event<H>(el: &Element, user_event: &UserEvent, event_listener: H)
where
    H: 'static + FnMut(MouseEvent),
{
    let cl = Closure::wrap(Box::new(event_listener) as Box<dyn FnMut(_)>);
    el.add_event_listener_with_callback(user_event.into(), cl.as_ref().unchecked_ref())
        .unwrap();
    cl.forget();
}

pub fn add_key_event<H>(el: &Element, user_event: &UserEvent, event_listener: H)
where
    H: 'static + FnMut(KeyEvent),
{
    let cl = Closure::wrap(Box::new(event_listener) as Box<dyn FnMut(_)>);
    el.add_event_listener_with_callback(user_event.into(), cl.as_ref().unchecked_ref())
        .unwrap();
    cl.forget();
}

fn remove_event<H>(el: &Element, user_event: &UserEvent, event_listener: H)
where
    H: 'static + FnMut(Event),
{
    let cl = Closure::wrap(Box::new(event_listener) as Box<dyn FnMut(_)>);
    el.remove_event_listener_with_callback(user_event.into(), cl.as_ref().unchecked_ref())
        .unwrap();
    cl.forget();
}

fn get_document() -> Result<Document, RenderError> {
    // Use `web_sys`'s global `window` function to get a handle on the global window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    //let body = document.body().expect("document should have a body");

    Ok(document)
}

pub fn get_target(target_id: &str) -> Result<Element, RenderError> {
    DOCUMENT.with(|doc| -> Result<Element, RenderError> {
        match doc.borrow().get_element_by_id(target_id) {
            Some(e) => Ok(e),
            None => Err(RenderError::new(
                format!("element {:?} does not exist", target_id).as_str(),
            )),
        }
    })
}

pub fn get_body() -> Element {
    DOCUMENT.with(|doc| -> Element { doc.borrow().body().expect("body does not exist").into() })
}

pub fn create_svg(camera: &Camera) -> Result<Element, RenderError> {
    let svg_view = DOCUMENT.with(|doc| -> Result<Element, JsValue> {
        doc.borrow().create_element_ns(SVG_NS, "svg")
    })?;

    svg_view.set_attribute(
        "viewBox",
        format!(
            "{:?} {:?} {:?} {:?}",
            camera.min_x(),
            camera.min_y(),
            camera.width,
            camera.height
        )
        .as_str(),
    )?;

    Ok(svg_view)
}

// -----------------------------------------------

pub struct Viewport {
    target_id: String,
    svg_view: Element,
    layers: Vec<Layer>,
}

impl Viewport {
    pub fn new(target_id: &str, width: i32, height: i32) -> Result<Viewport, RenderError> {
        // Setup the world render target. This must be done only once.
        let target = get_target(target_id)?;
        let render_camera = Camera::new(width, height);
        let svg_view = create_svg(&render_camera)?;
        svg_view.set_attribute("id", "svg_view")?;
        target.append_child(&svg_view)?;
        Ok(Viewport {
            target_id: target_id.to_owned(),
            svg_view: svg_view,
            layers: Vec::new(),
        })
    }

    pub fn look_at(&mut self, camera: &Camera) -> Result<(), RenderError> {
        self.svg_view.set_attribute(
            "viewBox",
            format!(
                "{:?} {:?} {:?} {:?}",
                camera.min_x(),
                camera.min_y(),
                camera.width,
                camera.height,
            )
            .as_str(),
        )?;

        Ok(())
    }

    pub fn clear_layer(&mut self, layer_name: &str) -> Result<(), RenderError> {
        match self.layer_mut(layer_name) {
            Some(layer) => {
                layer.clear()?;
            }
            None => {}
        };

        Ok(())
    }

    pub fn layer(&self, layer_name: &str) -> Option<&Layer> {
        for layer in &self.layers {
            if layer.name == layer_name {
                return Some(&layer);
            }
        }
        None
    }

    pub fn layer_mut(&mut self, layer_name: &str) -> Option<&mut Layer> {
        for layer in &mut self.layers {
            if layer.name == layer_name {
                return Some(layer);
            }
        }
        None
    }

    pub fn insert_layer(&mut self, order: usize, layer: Layer) {
        self.layers.insert(order, layer);
    }

    pub fn remove_layer(&mut self, layer_name: &str) {
        self.layers.retain(|layer| layer.name != layer_name);
    }

    pub fn event_point(&self, event: &MouseEvent) -> Result<Point, RenderError> {
        // Get point in global SVG space
        let svg_matrix = self
            .svg_view
            .clone()
            .dyn_into::<SvgsvgElement>()?
            .get_screen_ctm()
            .expect("failed to get screen ctm")
            .inverse()
            .expect("failed to get inverse");
        let svg_point = self
            .svg_view
            .clone()
            .dyn_into::<SvgsvgElement>()?
            .create_svg_point();
        svg_point.set_x(event.client_x() as f32);
        svg_point.set_y(event.client_y() as f32);
        let svg_point = svg_point.matrix_transform(&svg_matrix);

        Ok(Point::new(svg_point.x(), svg_point.y()))
    }
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub width: i32,
    pub height: i32,
    pub position: Point,
}

impl Camera {
    pub fn new(width: i32, height: i32) -> Camera {
        Camera {
            width: width,
            height: height,
            position: Point::origin(),
        }
    }
    pub fn look_at(&mut self, position: &Point) {
        self.position = position.clone();
    }

    fn min_x(&self) -> i32 {
        self.position.x as i32 - (self.width / 2)
    }

    fn min_y(&self) -> i32 {
        self.position.y as i32 - (self.height / 2)
    }
}

#[derive(Debug)]
pub struct RenderError {
    details: String,
}

impl RenderError {
    pub fn new(details: &str) -> RenderError {
        RenderError {
            details: details.to_owned(),
        }
    }
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

impl From<web_sys::Element> for RenderError {
    fn from(value: web_sys::Element) -> Self {
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

#[derive(Debug, Clone)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub fn new(width: i32, height: i32) -> Size {
        Size { width, height }
    }
}

#[derive(Debug, Clone)]
pub struct TextureBorder {
    width: i32,
    color: String,
}

impl TextureBorder {
    pub fn new(width: i32, color: &str) -> TextureBorder {
        TextureBorder {
            width: width,
            color: color.to_owned(),
        }
    }

    fn style_str(&self) -> String {
        // Note: Do not use {:?} or else it prints quotes around the color string.
        format!(
            "stroke:{};stroke-width:{:?};",
            self.color.as_str(),
            self.width
        )
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub image: Option<String>,
    pub background_color: Option<String>,
    pub border: Option<TextureBorder>,
}

impl Texture {
    pub fn new() -> Texture {
        Texture {
            image: None,
            background_color: None,
            border: None,
        }
    }

    fn style_str(&self) -> String {
        let mut style = "".to_owned();
        if let Some(bg_color) = &self.background_color {
            style = style + "fill:" + bg_color.as_str() + ";";
        } else {
            style = style + "fill-opacity: 0.0;";
        }
        if let Some(border) = &self.border {
            style = style + border.style_str().as_str();
        }
        style
    }
}
#[derive(Debug, Clone)]
pub enum Shape {
    Cell,
    Rectangle { width: i32, height: i32 },
    Circle { radius: i32 },
}

impl Shape {
    fn svg_name(&self) -> &str {
        match self {
            Shape::Cell => "polygon",
            Shape::Rectangle {
                width: _,
                height: _,
            } => "rect",
            Shape::Circle { radius: _ } => "circle",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sprite {
    id: String,
    shape: Shape,
    position: Point,
    texture: Texture,
    visible: bool,
}

impl Sprite {
    pub fn new(id: &str, shape: &Shape, position: &Point, texture: &Texture) -> Sprite {
        Sprite {
            id: id.to_owned(),
            shape: shape.clone(),
            position: position.clone(),
            texture: texture.clone(),
            visible: true,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn on<H>(id: &str, event: UserEvent, handler: H) -> Result<(), JsValue>
    where
        H: 'static + FnMut(Event),
    {
        let sprite_element = get_target(id)?;
        add_event(&sprite_element, &event, handler);

        Ok(())
    }

    pub fn remove_listener<H>(id: &str, event: UserEvent, handler: H) -> Result<(), JsValue>
    where
        H: 'static + FnMut(Event),
    {
        let sprite_element = get_target(id)?;
        remove_event(&sprite_element, &event, handler);

        Ok(())
    }
}

pub struct Layer {
    pub name: String,
    pub sprites: HashMap<CellCoord, Sprite>,
}

impl Layer {
    pub fn new(name: &str) -> Layer {
        Layer {
            name: name.to_owned(),
            sprites: HashMap::new(),
        }
    }

    pub fn add_sprite(&mut self, coord: CellCoord, sprite: Sprite) {
        self.sprites.insert(coord, sprite);
    }

    pub fn remove_sprite(&mut self, sprite_id: String) {
        // FIXME: Not performant. Must iterate all sprites.
        self.sprites.retain(|_, sprite| sprite.id != sprite_id);
    }
}

// ------- Renderable --------

pub trait Renderable {
    fn clear(&mut self) -> Result<(), RenderError>;

    fn render<C, L>(&self, layout: &L) -> Result<Element, RenderError>
    where
        C: Cell,
        L: Layout<C = C>;
}

impl Renderable for Viewport {
    fn clear(&mut self) -> Result<(), RenderError> {
        for layer in &mut self.layers {
            layer.clear()?;
        }

        Ok(())
    }

    fn render<C, L>(&self, layout: &L) -> Result<Element, RenderError>
    where
        C: Cell,
        L: Layout<C = C>,
    {
        debug("rendering viewport".to_owned());

        for layer in &self.layers {
            let layer_view = layer.render(layout)?;
            debug(format!("appending layer view: {}", layer.name));
            self.svg_view
                .append_child(&layer_view)
                .expect("failed to append child");
        }

        debug("viewport rendered".to_owned());

        Ok(self.svg_view.clone())
    }
}

impl Renderable for Layer {
    fn clear(&mut self) -> Result<(), RenderError> {
        let layer_view = match get_target(&self.name) {
            Ok(e) => e,
            Err(_) => {
                error(format!("failed to clear layer: {}", self.name));
                return Ok(()); // Ignore errors on clear. The elements might not exist.
            }
        };

        for (_, sprite) in &mut self.sprites {
            sprite.clear()?;
        }

        // Remove all sprite objects.
        self.sprites.clear();

        // Clear everything in this layer.
        layer_view.set_inner_html("");

        Ok(())
    }

    fn render<C, L>(&self, layout: &L) -> Result<Element, RenderError>
    where
        C: Cell,
        L: Layout<C = C>,
    {
        debug(format!("rendering layer: {}", self.name));

        let layer_view = match get_target(&self.name) {
            Ok(e) => e,
            Err(_) => DOCUMENT.with(|doc| -> Result<Element, JsValue> {
                let layer_view = doc.borrow().create_element_ns(SVG_NS, "g")?;

                layer_view.set_attribute("id", &self.name)?;

                Ok(layer_view)
            })?,
        };

        for (_, sprite) in &self.sprites {
            let sprite_view = sprite.render(layout)?;
            // Add the sprite to the layer.
            layer_view.append_child(&sprite_view)?;
        }

        debug(format!("layer rendered: {}", self.name));

        Ok(layer_view)
    }
}

impl Renderable for Sprite {
    fn clear(&mut self) -> Result<(), RenderError> {
        match get_target(self.id()) {
            Ok(sprite_view) => {
                // Remove the sprite from the DOM.
                sprite_view.remove();
                Ok(())
            }
            Err(_) => Ok(()), // Ignore errors on clear. The elements might not exist.
        }
    }

    fn render<C, L>(&self, layout: &L) -> Result<Element, RenderError>
    where
        C: Cell,
        L: Layout<C = C>,
    {
        debug(format!("rendering sprite: {}", self.id()));

        // Group all of a sprites data together.
        let sprite_view = DOCUMENT.with(|doc| -> Result<Element, JsValue> {
            doc.borrow().create_element_ns(SVG_NS, "g")
        })?;

        sprite_view.set_attribute("id", self.id())?;

        // A sprite is defined as a polygon of any shape.
        let width;
        let height;

        let svg_element = DOCUMENT.with(|doc| -> Result<Element, JsValue> {
            doc.borrow()
                .create_element_ns(SVG_NS, self.shape.svg_name())
        })?;

        let sprite_polygon = match self.shape {
            Shape::Cell => {
                let cell = layout.pixel_to_cell(&self.position);

                let mut corners_string: String = "".to_owned();
                for corner in layout.polygon_corners(&cell) {
                    corners_string.push_str(String::from(&corner).as_str());
                    corners_string.push_str(" ");
                }

                svg_element.set_attribute("points", corners_string.as_str())?;

                let cell_size: f32 = layout
                    .cell_corner_offset(0)
                    .x
                    .max(layout.cell_corner_offset(0).y)
                    .abs();

                width = cell_size;
                height = cell_size;

                svg_element
            }
            Shape::Rectangle {
                width: rect_width,
                height: rect_height,
            } => {
                svg_element.set_attribute("width", rect_width.to_string().as_str())?;
                svg_element.set_attribute("height", rect_height.to_string().as_str())?;
                svg_element.set_attribute("x", (-rect_width as f32 / 2.0).to_string().as_str())?;
                svg_element.set_attribute("y", (-rect_height as f32 / 2.0).to_string().as_str())?;

                width = rect_width as f32;
                height = rect_height as f32;

                svg_element
            }
            Shape::Circle { radius } => {
                svg_element.set_attribute("cx", "0")?;
                svg_element.set_attribute("cy", "0")?;
                svg_element.set_attribute("r", radius.to_string().as_str())?;

                width = radius as f32;
                height = radius as f32;

                svg_element
            }
        };

        sprite_polygon.set_attribute("style", self.texture.style_str().as_str())?;

        // Add the polygon shape to the sprite group.
        sprite_view.append_child(&sprite_polygon)?;

        // Set any texture for the sprite as an <image> child of the sprite group.
        if let Some(image) = &self.texture.image {
            let sprite_image = DOCUMENT.with(|doc| -> Result<Element, JsValue> {
                doc.borrow().create_element_ns(SVG_NS, "image")
            })?;

            sprite_image.set_attribute("href", &image)?;
            sprite_image.set_attribute("width", width.to_string().as_str())?;
            sprite_image.set_attribute("height", height.to_string().as_str())?;
            sprite_image.set_attribute("x", (-width / 2.0).to_string().as_str())?;
            sprite_image.set_attribute("y", (-height / 2.0).to_string().as_str())?;

            sprite_view.append_child(&sprite_image)?;
        }

        // All sprite data is defined about the origin.
        // Move the sprite to the correct location.
        // FIXME: There is a collision between world space and screen space that needs to be fixed. The sprites are working inside screen space,
        //        but ideally it needs to work inside world space. The camera needs to operate in world space as well.
        sprite_view.set_attribute(
            "transform",
            format!("translate({},{})", self.position.x, self.position.y).as_str(),
        )?;

        debug(format!("sprite rendered: {}", self.id()));

        Ok(sprite_view)
    }
}
/*
pub struct RenderRect {
    point: Point,
    width: f32,
    height: f32,
}

pub struct RenderCircle {
    point: Point,
    radius: f32,
}

pub trait Renderable {
    fn render<H>(
        &self,
        document: &Document,
        target: &Element,
        id: Option<String>,
        style: Option<String>,
        listeners: HashMap<UserEvent, H>,
    ) -> Result<(), RenderError>
    where
        H: 'static + FnMut(Event),
    {
        Ok(())
    }
}
*/
/*
pub struct RenderCell<C: Cell, H>
where
    H: 'static + FnMut(Event),
{
    cell: C,
    id: Option<String>,
    style: Option<String>,
    listeners: HashMap<UserEvent, H>,
}

pub trait Renderer<C: Cell, L: Layout<C = C>> {
    fn render(
        &self,
        layout: L,
        document: &Document,
        target: &Element,
        style: &str,
    ) -> Result<(), RenderError>;
}
*/
/*
impl<C: Cell, L: Layout<C = C>> Renderer<C, L> for C {
    fn render(
        &self,
        layout: L,
        document: &Document,
        target: &Element,
        style: &str,
    ) -> Result<(), RenderError> {
        let cell_view = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

        let polygon = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "polygon")?;
        let corners = layout.polygon_corners(self);

        let mut corners_string: String = "".to_owned();
        for corner in corners {
            corners_string.push_str(String::from(corner).as_str());
            corners_string.push_str(" ");
        }

        cell_view.set_attribute("points", corners_string.as_str())?;
        cell_view.set_attribute("style", style)?; //"fill:lime;stroke:purple;stroke-width:1"

        // Set mouseover text to coordinate.
        // let title = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "title")?;
        // title.set_inner_html(
        //     format!(
        //         "Cell:{:?} Point:{:?}",
        //         String::from(&self.coord()),
        //         layout.cell_to_pixel(self)
        //     )
        //     .as_str(),
        // );
        // cell_view.append_child(&title)?;

        target.append_child(&cell_view)?;

        Ok(())
    }
}

impl<C: Cell, L: Layout<C = C>> Renderer for WorldMap<C, L> {
    fn render(&self, target_id: &str) -> Result<(), RenderError> {
        engine::debug(format!("rendering WorldMap to {:?}", target_id));

        let document = get_document()?;
        let target = get_target(&document, target_id)?;

        // Clear the target of any existing elements.
        target.set_inner_html("");

        // Add the root svg container.
        let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
        svg.set_attribute("id", "world_view")?;
        let view_box_min_x = -200;
        let view_box_min_y = -200;
        svg.set_attribute(
            "viewBox",
            format!("{:?} {:?} 500 500", view_box_min_x, view_box_min_y).as_str(),
        )?;

        let layer_background =
            document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;
        let layer_sprites = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;
        let layer_overlay = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

        for (_, cell) in &self.map {
            self.render_background(&document, &layer_background, &cell)?;
            self.render_sprites(&document, &layer_sprites, &cell)?;
            self.render_overlay(&document, &layer_overlay, &cell)?;
        }

        svg.append_child(&layer_background)?;
        svg.append_child(&layer_sprites)?;
        svg.append_child(&layer_overlay)?;

        let cell_size: f32 = self
            .layout
            .cell_corner_offset(0)
            .x
            .max(self.layout.cell_corner_offset(0).y)
            .abs();

        if let Some(path) = self.shortest_path(
            &C::new(-3.0, -2.0, 5.0),
            &C::new(0.0, -5.0, 5.0),
            &game_state().collision_set,
        ) {
            for entry_cell in path {
                let path_preview =
                    document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
                path_preview.set_attribute("cx", "0")?;
                path_preview.set_attribute("cy", "0")?;
                path_preview.set_attribute("r", (cell_size * 0.5).to_string().as_str())?;
                path_preview.set_attribute("style", "fill:green;stroke:black;stroke-width:1")?;

                let cell_center = self.layout.cell_to_pixel(&entry_cell);
                path_preview.set_attribute(
                    "transform",
                    format!("translate({},{})", cell_center.x, cell_center.y).as_str(),
                )?;

                svg.append_child(&path_preview)?;
            }
        }

        target.append_child(&svg)?;

        // TODO: cannot borrow world because the closure can outlive it. world needs to be a &'static so JS callback closures and operate on it.
        add_event(&svg, "click", move |e: Event| {
            e.prevent_default();
            let btn = e.clone().dyn_into::<MouseEvent>().unwrap().button();
            if btn == 0 {
                //alert(format!("left click").as_str());
            } else if btn == 2 {
                //alert(format!("right click").as_str());
            }

            let event = e.clone().dyn_into::<MouseEvent>().unwrap();

            //alert(format!("mousedown: {:?},{:?}", event.screen_x(), event.screen_y()).as_str());

            // // Create an SVGPoint for future math
            // var pt = svg.createSVGPoint();

            // // Get point in global SVG space
            // function cursorPoint(evt){
            //   pt.x = evt.clientX; pt.y = evt.clientY;
            //   return pt.matrixTransform(svg.getScreenCTM().inverse());
            // }

            let svg_target = event
                .current_target()
                .unwrap()
                .dyn_into::<SvgsvgElement>()
                .unwrap();
            let svg_point = svg_target.create_svg_point();
            svg_point.set_x(event.client_x() as f32); // + view_box_min_x as f32);
            svg_point.set_y(event.client_y() as f32); //+ view_box_min_y as f32);
            let svg_matrix = svg_target.get_screen_ctm().unwrap().inverse().unwrap();
            let svg_point = svg_point.matrix_transform(&svg_matrix);

            let point = Point::new(svg_point.x(), svg_point.y());
            WORLD.with(|w| {
                let clicked_cell = w.borrow().layout.pixel_to_cell(&point);
                alert(
                    format!(
                        "mousedown: client {:?},{:?} svg {:?},{:?} cell {:?},{:?},{:?}",
                        event.client_x(),
                        event.client_y(),
                        svg_point.x(),
                        svg_point.y(),
                        clicked_cell.coord().x,
                        clicked_cell.coord().y,
                        clicked_cell.coord().z,
                    )
                    .as_str(),
                );
            });
        });

        Ok(())
    }
}

impl<C: Cell, L: Layout<C = C>> WorldMap<C, L> {
    fn render_background(
        &self,
        document: &Document,
        target: &Element,
        cell: &C,
    ) -> Result<(), RenderError> {
        let polygon = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "polygon")?;
        let corners = self.layout.polygon_corners(&cell);

        let mut corners_string: String = "".to_owned();
        for corner in corners {
            corners_string.push_str(String::from(corner).as_str());
            corners_string.push_str(" ");
        }

        // Create cell view.

        polygon.set_attribute("points", corners_string.as_str())?;
        polygon.set_attribute("style", "fill:lime;stroke:purple;stroke-width:1")?;

        // Set mouseover text to coordinate.
        let title = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "title")?;
        title.set_inner_html(
            format!(
                "Cell:{:?} Point:{:?}",
                String::from(&cell.coord()),
                self.layout.cell_to_pixel(&cell)
            )
            .as_str(),
        );
        polygon.append_child(&title)?;
        /*
                let cell_center = self.layout.cell_to_pixel(cell);
                polygon.set_attribute(
                    "transform",
                    format!("translate({},{})", cell_center.x, cell_center.y).as_str(),
                )?;
        */
        target.append_child(&polygon)?;

        Ok(())
    }

    fn render_sprites(
        &self,
        document: &Document,
        target: &Element,
        cell: &C,
    ) -> Result<(), RenderError> {
        // Render BuildingViews.
        if let Some(building) = game_state().building_at(&cell.coord()) {
            let building_view =
                document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?;
            let cell_size: f32 = self
                .layout
                .cell_corner_offset(0)
                .x
                .max(self.layout.cell_corner_offset(0).y)
                .abs();
            building_view.set_attribute("width", cell_size.to_string().as_str())?;
            building_view.set_attribute("height", cell_size.to_string().as_str())?;
            building_view.set_attribute("x", (-cell_size / 2.0).to_string().as_str())?;
            building_view.set_attribute("y", (-cell_size / 2.0).to_string().as_str())?;
            building_view.set_attribute("style", "fill:brown;stroke:purple;stroke-width:1")?;
            building_view.set_attribute("title", String::from(&building.state).as_str())?;

            let cell_center = self.layout.cell_to_pixel(cell);
            building_view.set_attribute(
                "transform",
                format!("translate({},{})", cell_center.x, cell_center.y).as_str(),
            )?;

            target.append_child(&building_view)?;
        }

        Ok(())
    }

    fn render_overlay(
        &self,
        document: &Document,
        target: &Element,
        cell: &C,
    ) -> Result<(), RenderError> {
        let cell_size: f32 = self
            .layout
            .cell_corner_offset(0)
            .x
            .max(self.layout.cell_corner_offset(0).y)
            .abs();

        let cell_center = self.layout.cell_to_pixel(cell);

        // Render BuildingView overlays.
        if let Some(building) = game_state().building_at(&cell.coord()) {
            // Add nodes to each edge.
            for direction in cell.directions() {
                let node_view =
                    document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
                let edge_center = self.layout.polygon_edge_center(cell, *direction);
                node_view.set_attribute("cx", (edge_center.x).to_string().as_str())?;
                node_view.set_attribute("cy", (edge_center.y).to_string().as_str())?;
                node_view.set_attribute("r", (cell_size * 0.2).to_string().as_str())?;
                node_view.set_attribute("style", "fill:blue;stroke:black;stroke-width:1")?;
                node_view.set_attribute("class", "building-node")?;

                node_view.set_attribute(
                    "transform",
                    format!("translate({},{})", cell_center.x, cell_center.y).as_str(),
                )?;

                target.append_child(&node_view)?;
            }
        }

        let path_preview =
            document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
        path_preview.set_attribute("cx", "0")?;
        path_preview.set_attribute("cy", "0")?;
        path_preview.set_attribute("r", (cell_size * 0.5).to_string().as_str())?;
        path_preview.set_attribute("style", "fill:green;stroke:black;stroke-width:1")?;
        path_preview.set_attribute("class", "path-preview")?;

        let cell_center = self.layout.cell_to_pixel(&cell);
        path_preview.set_attribute(
            "transform",
            format!("translate({},{})", cell_center.x, cell_center.y).as_str(),
        )?;

        target.append_child(&path_preview)?;

        // Render hidden belt routing for displaying where belt will be placed if accepted.
        // Add nodes to each edge.
        for direction in cell.directions() {
            let belt_preview_view =
                document.create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?;
            let edge_center = self.layout.polygon_edge_center(cell, *direction);
            belt_preview_view.set_attribute("x1", (edge_center.x).to_string().as_str())?;
            belt_preview_view.set_attribute("y1", (edge_center.y).to_string().as_str())?;
            belt_preview_view.set_attribute("x2", (self.layout.origin().x).to_string().as_str())?;
            belt_preview_view.set_attribute("y2", (self.layout.origin().y).to_string().as_str())?;
            belt_preview_view.set_attribute("style", "fill:gray;stroke:black;stroke-width:1")?;
            belt_preview_view.set_attribute("class", "belt-preview")?;

            belt_preview_view.set_attribute(
                "transform",
                format!("translate({},{})", cell_center.x, cell_center.y).as_str(),
            )?;

            target.append_child(&belt_preview_view)?;
        }

        Ok(())
    }
}
*/
