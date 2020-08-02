use std::cmp;
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use super::cell::{Cell, CellCoord};
use super::layout::Layout;
use super::logging::{debug, info};
use super::renderer::{create_svg, get_document, get_target, Layer, RenderError, Renderable};

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
        }
    }

    pub fn render(&self, target_id: &str) -> Result<(), RenderError> {
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

        Ok(())
    }

    pub fn shortest_path(&self, from: &C, to: &C, collisions: &Collisions) -> Option<Vec<C>> {
        a_star_search(from, to, &self.base_map, collisions)
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
        debug(format!(
            "START frontier peek {:?} {:?}",
            frontier.peek().unwrap().0.cell.coord(),
            frontier.peek().unwrap().0.priority,
        ));
        debug(format!("frontier size: {:?}", frontier.len()));
        if let Some(current) = frontier.pop() {
            debug(format!("frontier size: {:?}", frontier.len()));
            debug(format!("  current: {:?}", current.0.cell.coord()));
            if current.0.cell.coord() == end.coord() {
                // Stop if we have reached the end.

                info("  found a path".to_owned());
                let mut path: Vec<C> = Vec::new();

                path.push(end.clone());

                info(format!("path: {:?}", &end.coord()));

                let mut previous = came_from.get(&end.coord()).unwrap();
                while previous.cell.coord() != start.coord() {
                    info(format!("path: {:?}", &previous.cell.coord()));
                    path.push(previous.cell.clone());
                    previous = came_from.get(&previous.cell.coord()).unwrap();
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
                debug(format!("next: {:?}", &next.coord()));
                let current_cost = cost_so_far
                    .get(&current.0.cell.coord())
                    .or(Some(&0))
                    .unwrap();
                let cost_to_next = 1; // This is the cost of traversing to this next cell.
                let new_cost = current_cost + cost_to_next;
                let next_cost = *cost_so_far.get(&next.coord()).or(Some(&0)).unwrap();
                if !cost_so_far.contains_key(&next.coord()) || new_cost < next_cost {
                    cost_so_far.insert(next.coord(), new_cost);
                    let priority = new_cost + heuristic(&end.coord(), &next.coord());
                    debug(format!("  priority {:?}", priority));
                    frontier.push(Reverse(CellPriority::new(next.clone(), priority)));
                    debug(format!(
                        "  frontier peek {:?}",
                        frontier.peek().unwrap().0.cell.coord()
                    ));
                    debug(format!("frontier size: {:?}", frontier.len()));
                    came_from.insert(next.coord(), current.0.clone());
                }
            }
            debug(format!(
                "  frontier peek {:?}",
                frontier.peek().unwrap().0.cell.coord()
            ));
            debug(format!("frontier size: {:?}", frontier.len()));
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
