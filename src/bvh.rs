use crate::bit_packing::bits_needed;
use crate::geometry::{Aabb, Int2, Tri};
use crate::morton;

#[derive(Clone, Copy, Debug)]
pub struct BvhNode {
    pub box_: Aabb,
    pub code: u64,
}

pub struct Bvh {
    pub global: Aabb,
    pub mask: u64,
    pub num_leaves: u64,

    pub nodes: Vec<BvhNode>,

    pub ranges: Vec<Int2>,
    pub ready: Vec<i32>,
    pub parents: Vec<i32>,
    pub siblings: Vec<i32>,
    pub code_ids: Vec<u64>,
    pub primitives: Vec<Tri>,
}

fn global_aabb(boxes: &Vec<Aabb>) -> Aabb {
    let mut global_box = boxes[0];
    for i in 0..boxes.len() {
        global_box = global_box.add(boxes[i]);
    }

    global_box
}

fn morton_sort(boxes: &Vec<Aabb>, global: &Aabb) -> Vec<u64> {
    const DIM: u32 = 3;

    let num_boxes = boxes.len() as u32;
    let x_offset = global.min_x;
    let y_offset = global.min_y;
    let z_offset = global.min_z;

    let b = bits_needed(num_boxes);
    let bits_per_dimension = ((64 - b) / DIM) as i32;
    let divisions_per_dimension = 1 << bits_per_dimension;

    let scale = (divisions_per_dimension - 1) as f32;

    let x_scale = scale / (global.max_x - global.min_x);
    let y_scale = scale / (global.max_y - global.min_y);
    let z_scale = scale / (global.max_z - global.min_z);

    let num_boxes = num_boxes as usize;

    let mut code_ids: Vec<u64> = Vec::with_capacity(num_boxes);

    for i in 0..num_boxes {
        let box_ = boxes[i];

        // get the centroid of the ith bounding box
        let cx = 0.5 * (box_.min_x + box_.max_x);
        let cy = 0.5 * (box_.min_y + box_.max_y);
        let cz = 0.5 * (box_.min_z + box_.max_z);

        let ux = ((cx - x_offset) * x_scale) as u64;
        let uy = ((cy - y_offset) * y_scale) as u64;
        let uz = ((cz - z_offset) * z_scale) as u64;

        let code: u64 = morton::encode(ux, uy, uz);

        code_ids.push((code << b) + (i as u64));
    }

    code_ids.sort();

    code_ids
}

// convenience class for bounds checking morton codes,
// and computing the number of prefix bits common to two
// 64-bit words

struct PrefixComparator {
    base: u64,
    codes: Vec<u64>,
    n: i32,
}

impl PrefixComparator {
    fn call(&self, i: i32) -> i32 {
        if i >= 0 && i < self.n {
            (self.base ^ self.codes[i as usize]).leading_zeros() as i32
        } else {
            -1
        }
    }
}

impl Bvh {
    pub fn from(_primitives: &Vec<Tri>) -> Self {
        let num_leaves = _primitives.len();

        let mut primitives: Vec<Tri> = Vec::with_capacity(num_leaves);

        let capacity = 2 * num_leaves;
        let mut nodes: Vec<BvhNode> = Vec::with_capacity(capacity);
        let ranges: Vec<Int2> = vec![Int2::default(); capacity];
        let ready: Vec<i32> = Vec::with_capacity(capacity);
        let parents: Vec<i32> = vec![0; capacity];
        let siblings: Vec<i32> = vec![0; capacity];

        let mask: u64 = ((1 as u64) << bits_needed(num_leaves as u32)) - (1 as u64);

        let mut boxes: Vec<Aabb> = Vec::with_capacity(num_leaves);
        for i in 0..num_leaves {
            boxes.push(Aabb::from(_primitives[i]));
        }

        let global = global_aabb(&boxes);

        let code_ids = morton_sort(&boxes, &global);

        for i in 0..num_leaves {
            let id = (code_ids[i] & mask) as usize;
            primitives.push(_primitives[id]);
            nodes.push(BvhNode {
                box_: boxes[id],
                code: code_ids[i],
            });
        }

        let num_leaves = num_leaves as u64;
        let mut out = Bvh {
            primitives,
            ranges,
            ready,
            parents,
            siblings,
            mask,
            num_leaves,
            global,
            code_ids,
            nodes,
        };

        out.build_radix_tree();
        // fit_bounding_boxes();

        out
    }

    fn build_radix_tree(&mut self) {
        let num_leaves = self.num_leaves as i32;
        self.parents[self.num_leaves as usize] = num_leaves;

        for i in 0..num_leaves - 1 {
            let shared_prefix = PrefixComparator {
                base: self.code_ids[i as usize],
                codes: self.code_ids.clone(),
                n: num_leaves,
            };

            // Choose search direction.
            let prefix_prev = shared_prefix.call(i - 1);
            let prefix_next = shared_prefix.call(i + 1);
            let prefix_min = prefix_prev.min(prefix_next);

            let d = if prefix_next > prefix_prev {
                1
            } else {
                -1
            };

            // Find upper bound for length.
            let mut l_max = 32;
            let mut probe: i32;
            loop {
                l_max <<= 2;
                probe = i + l_max * d;
                if !(probe < num_leaves && shared_prefix.call(probe) > prefix_min) {
                    break;
                }
            }

            // Determine length.
            let mut l = 0;
            let mut t = l_max >> 1;
            while t > 0 {
                probe = i + (l + t) * d;

                if (probe as u64) < self.num_leaves && shared_prefix.call(probe) > prefix_min {
                    l += t;
                }

                t >>= 1;
            }
            let j = i + l * d;
            let prefix_node = shared_prefix.call(j);

            // Find split point.
            let mut s = 0;
            let mut t = l;
            loop {
                t = (t + 1) >> 1;
                probe = i + (s + t) * d;
                if (probe as u64) < self.num_leaves && shared_prefix.call(probe) > prefix_node {
                    s += t;
                }

                if !(t > 1) {
                    break;
                }
            }
            let k = (i + s * d + d.min(0)) as usize;

            // Output node.
            let lo = i.min(j) as usize;
            let hi = i.min(j) as usize;

            let left = if lo == k {
                k + 0
            } else {
                k + 0 + (self.num_leaves as usize)
            };
            let right = if hi == k + 1 {
                k + 1
            } else {
                k + 1 + (self.num_leaves as usize)
            };

            self.parents[left] = i + num_leaves;
            self.parents[right] = i + num_leaves;

            self.siblings[left] = right as i32;
            self.siblings[right] = left as i32;
            self.ranges[(i as u64 + self.num_leaves) as usize] = Int2 {
                x: lo as i32,
                y: hi as i32,
            };
            // self.nodes[(i as u64 + self.num_leaves) as usize].code = ((left as u64) << 32) + (right as u64);
        }
    }

    // void bvh< T >::fit_bounding_boxes() {
    //     for (int i = 0; i < ready.size(); i++) {
    //         ready[i] = 0;
    //     }

    //     for (int i = 0; i < num_leaves; i++) {

    //         // start with the bounds of the leaf nodes
    //         // and have each thread work its way up the tree
    //         int current = i;
    //         aabb box = nodes[i].box;
    //         int32_t parent = parents[i];
    //         int state = 0;

    //         while (true) {

    //             state = ready[parent]++;

    //             // only process a parent node if the other
    //             // sibling has visited the parent as well
    //             if (state != 1) break;

    //             // compute the union of the two sibling boxes
    //             box = aabb(box, nodes[siblings[current]].box);

    //             // move up to the parent node
    //             current = parent;
    //             parent = parents[current];

    //             // and assign the new box to it
    //             nodes[current].box = box;

    //         }
    //     }
    // }
}
