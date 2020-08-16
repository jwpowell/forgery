use std::cmp;
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use super::cell::{Cell, CellCoord, Point};
use super::layout::Layout;
use super::logging::{debug, info};
use super::renderer::{
    add_event, add_key_event, add_mouse_event, create_svg, get_document, get_target, Layer,
    RenderError, Renderable, UserEvent,
};

use web_sys::{Document, Element, Event, MouseEvent, SvgElement, SvgsvgElement};

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;

// TODO: This will eventually be more complex and involve movement costs, etc.
type Collisions = HashSet<CellCoord>;

pub struct World<C, L>
where
    C: Cell,
    L: Layout<C = C>,
{
    pub base_map: HashSet<CellCoord>,
    pub layers: Vec<Layer>,
    pub layout: L,
    svg_view: Option<SvgsvgElement>,
}

impl<C, L> World<C, L>
where
    C: Cell,
    L: Layout<C = C>,
{
    pub fn new(layout: L) -> World<C, L> {
        World {
            base_map: HashSet::new(),
            layers: Vec::new(),
            layout: layout,
            svg_view: None,
        }
    }

    pub fn render(&mut self, target_id: &str) -> Result<(), RenderError> {
        debug("rendering world".to_owned());

        let document = get_document()?;
        let target = get_target(&document, target_id)?;

        let svg_view = create_svg(&document, -200, -200, 500, 500)?;

        for layer in &self.layers {
            let layer_element = layer.render(&document, &target, &self.layout)?;

            for (_, sprite) in &layer.sprites {
                sprite.render(&document, &layer_element, &self.layout)?;
            }

            svg_view.append_child(&layer_element)?;
        }

        target.append_child(&svg_view)?;

        let svg_target = svg_view.dyn_into::<SvgsvgElement>().unwrap();
        self.svg_view = Some(svg_target);

        Ok(())
    }

    pub fn event_point(&self, event: &MouseEvent) -> Point {
        // Get point in global SVG space
        let svg_target = self.svg_view.as_ref().expect("svg not set");
        let svg_matrix = svg_target
            .get_screen_ctm()
            .expect("failed to get screen ctm")
            .inverse()
            .expect("failed to get inverse");
        let svg_point = svg_target.create_svg_point();
        svg_point.set_x(event.client_x() as f32);
        svg_point.set_y(event.client_y() as f32);
        let svg_point = svg_point.matrix_transform(&svg_matrix);

        Point::new(svg_point.x(), svg_point.y())
    }

    pub fn event_cell(&self, event: &MouseEvent) -> C {
        let pixel = self.event_point(event);
        self.layout.pixel_to_cell(&pixel)
    }

    pub fn on_mouse_event<H>(&self, event: UserEvent, handler: H) -> Result<(), JsValue>
    where
        H: 'static + FnMut(MouseEvent),
    {
        add_mouse_event(
            &get_document()?
                .body()
                .expect("body does not exist")
                .as_ref(),
            &event,
            handler,
        );

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct CellPriority<C: Cell> {
    cell: C,
    priority: i32,
}

impl<C: Cell> CellPriority<C> {
    fn new(cell: C, priority: i32) -> CellPriority<C> {
        CellPriority { cell, priority }
    }
}

impl<C: Cell> Eq for CellPriority<C> {}

impl<C: Cell> Ord for CellPriority<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<C: Cell> PartialOrd for CellPriority<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: Cell> PartialEq for CellPriority<C> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

pub fn shortest_path<C>(
    from: &C,
    to: &C,
    world: &HashSet<CellCoord>,
    collisions: &Collisions,
) -> Option<Vec<C>>
where
    C: Cell,
{
    a_star_search(from, to, world, collisions)
}

fn heuristic(a: &CellCoord, b: &CellCoord) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

fn a_star_search<C: Cell>(
    start: &C,
    end: &C,
    base_map: &HashSet<CellCoord>,
    collisions: &HashSet<CellCoord>,
) -> Option<Vec<C>> {
    debug(format!("=============================="));
    let mut frontier: BinaryHeap<Reverse<CellPriority<C>>> = BinaryHeap::new();
    frontier.push(Reverse(CellPriority::new(start.clone(), 0)));

    let mut came_from: HashMap<CellCoord, CellPriority<C>> = HashMap::new();
    came_from.insert(start.coord(), CellPriority::new(start.clone(), 0));

    let mut cost_so_far: HashMap<CellCoord, i32> = HashMap::new();
    cost_so_far.insert(start.coord(), 0);

    while !frontier.is_empty() {
        if cost_so_far.len() > 1000 {
            // Safety break;
            break;
        }
        if let Some(current) = frontier.pop() {
            if current.0.cell.coord() == end.coord() {
                // Stop if we have reached the end.

                info("  found a path".to_owned());
                let mut path: Vec<C> = Vec::new();

                path.push(end.clone());

                info(format!("path: {:?}", &end.coord()));

                let mut previous = came_from
                    .get(&end.coord())
                    .expect("failed to where came from");
                while previous.cell.coord() != start.coord() {
                    info(format!("path: {:?}", &previous.cell.coord()));
                    path.push(previous.cell.clone());
                    previous = came_from
                        .get(&previous.cell.coord())
                        .expect("failed to get previous");
                }

                path.push(start.clone());
                info(format!("path: {:?}", &start.coord()));

                path.reverse();
                info("DONE".to_owned());
                return Some(path);
            }

            for next in &current.0.cell.neighbors() {
                if !base_map.contains(&next.coord()) || collisions.contains(&next.coord()) {
                    continue;
                }
                let current_cost = cost_so_far
                    .get(&current.0.cell.coord())
                    .or(Some(&0))
                    .expect("failed to get cost so far");
                let cost_to_next = 1; // This is the cost of traversing to this next cell.
                let new_cost = current_cost + cost_to_next;
                let next_cost = *cost_so_far
                    .get(&next.coord())
                    .or(Some(&0))
                    .expect("failed to set new cost");
                if !cost_so_far.contains_key(&next.coord()) || new_cost < next_cost {
                    cost_so_far.insert(next.coord(), new_cost);
                    let priority = new_cost + heuristic(&end.coord(), &next.coord());
                    frontier.push(Reverse(CellPriority::new(next.clone(), priority)));
                    came_from.insert(next.coord(), current.0.clone());
                }
            }
        }
    }

    // Could not find any path.
    None
}

/*
def heuristic(a, b):
    (x1, y1) = a
    (x2, y2) = b
    return abs(x1 - x2) + abs(y1 - y2)

def a_star_search(graph, start, goal):
    frontier = PriorityQueue()
    frontier.put(start, 0)
    came_from = {}
    cost_so_far = {}
    came_from[start] = None
    cost_so_far[start] = 0
    while not frontier.empty():
        current = frontier.get()
        if current == goal:
            break
        for next in graph.neighbors(current):
            new_cost = cost_so_far[current] + graph.cost(current, next)
            if next not in cost_so_far or new_cost < cost_so_far[next]:
                cost_so_far[next] = new_cost
                priority = new_cost + heuristic(goal, next)
                frontier.put(next, priority)
                came_from[next] = current
    return came_from, cost_so_far
*/
