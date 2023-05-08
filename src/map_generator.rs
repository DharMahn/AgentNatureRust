use rand::Rng;

use crate::cell_type::CellType;

use super::cell::Cell;
pub fn generate_map(map_width: usize, map_height: usize) -> Vec<Vec<Cell>> {
    let mut map = vec![vec![Cell::new(CellType::Empty); map_height]; map_width];
    generate_water(&mut map);
    generate_features(&mut map, 0.5, CellType::Grass);
    smooth_map(&mut map);
    generate_features(&mut map, 0.57, CellType::Sand);
    generate_trees(&mut map);
    generate_walls(&mut map);
    map
}

fn smooth_map(map: &mut Vec<Vec<Cell>>) {
    let width = map.len();
    let height = map[0].len();

    let mut temp = vec![vec![false; height]; width];

    for y in 0..height {
        for x in 0..width {
            temp[x][y] = smooth_logic(map, x, y);
        }
    }

    for y in 0..height {
        for x in 0..width {
            map[x][y].cell_type = if temp[x][y] {
                CellType::Grass
            } else {
                CellType::Water
            };
        }
    }
}

fn smooth_logic(map: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    let num_walls = calculate_neighbours(map, x, y);

    match map[x][y].cell_type {
        CellType::Grass => {
            if num_walls >= 4 {
                return true;
            }
            if num_walls < 2 {
                return false;
            }
        }
        _ => {
            if num_walls >= 5 {
                return true;
            }
        }
    }
    false
}

fn calculate_neighbours(map: &Vec<Vec<Cell>>, xpos: usize, ypos: usize) -> i32 {
    let width = map.len();
    let height = map[0].len();
    let mut neighbour_count = 0;

    for y_offset in -1..=1 {
        for x_offset in -1..=1 {
            if x_offset != 0 || y_offset != 0 {
                let temp_x = (xpos as i32 + x_offset).rem_euclid(width as i32);
                let temp_y = (ypos as i32 + y_offset).rem_euclid(height as i32);

                if map[temp_x as usize][temp_y as usize].cell_type == CellType::Grass {
                    neighbour_count += 1;
                }
            }
        }
    }
    neighbour_count
}
fn generate_walls(map: &mut Vec<Vec<Cell>>) {
    let width = map.len();
    let height = map[0].len();
    for i in 0..map.len() {
        map[i][0].cell_type = CellType::Structure;
        map[i][height - 1].cell_type = CellType::Structure;
    }
    for i in 0..map[0].len() {
        map[0][i].cell_type = CellType::Structure;
        map[width - 1][i].cell_type = CellType::Structure;
    }
}
fn generate_trees(map: &mut Vec<Vec<Cell>>) {
    let mut rng = rand::thread_rng();
    for _ in 0..(map.len() * map[0].len()) / 100 {
        let id_x = rng.gen_range(0..map[0].len());
        let id_y = rng.gen_range(0..map.len());
        if map[id_y][id_x].cell_type == CellType::Grass {
            map[id_y][id_x].cell_type = CellType::Wood;
        }
    }
}
fn generate_features(map: &mut Vec<Vec<Cell>>, treshold: f32, convert_into: CellType) {
    let pnoise = generate_perlin_noise(generate_white_noise(map[0].len(), map.len()), 8);
    for y in 0..map[0].len() {
        for x in 0..map.len() {
            if pnoise[x][y] < treshold {
                map[x][y].cell_type = convert_into.clone();
            }
        }
    }
}
fn generate_water(map: &mut Vec<Vec<Cell>>) {
    for y in 0..map[0].len() {
        for x in 0..map.len() {
            map[x][y] = Cell::new(CellType::Water);
        }
    }
}
fn generate_white_noise(width: usize, height: usize) -> Vec<Vec<f32>> {
    let mut rng = rand::thread_rng();
    let mut noise = vec![vec![0.0; height]; width];
    for i in 0..width {
        for j in 0..height {
            noise[i][j] = rng.gen::<f32>();
        }
    }
    return noise;
}
fn generate_smooth_noise(base_noise: Vec<Vec<f32>>, octave: usize) -> Vec<Vec<f32>> {
    let width = base_noise.len();
    let height = base_noise[0].len();
    let mut smooth_noise = vec![vec![0.0; width]; height];
    let sample_period = 1 << octave;
    let sample_freq = 1.0 / (sample_period as f32);
    for i in 0..width {
        let sample_i0 = (i / sample_period) * sample_period;
        let sample_i1 = (sample_i0 + sample_period) % width;
        let horizontal_blend = (i - sample_i0) as f32 * sample_freq;
        for j in 0..height {
            let sample_j0 = (j / sample_period) * sample_period;
            let sample_j1 = (sample_j0 + sample_period) % height;
            let vertical_blend = (j - sample_j0) as f32 * sample_freq;
            let top = interpolate(
                base_noise[sample_i0][sample_j0],
                base_noise[sample_i1][sample_j0],
                horizontal_blend,
            );
            let bottom = interpolate(
                base_noise[sample_i0][sample_j1],
                base_noise[sample_i1][sample_j1],
                horizontal_blend,
            );
            smooth_noise[j][i] = interpolate(top, bottom, vertical_blend);
        }
    }
    return smooth_noise;
}
fn generate_perlin_noise(base_noise: Vec<Vec<f32>>, octave_count: usize) -> Vec<Vec<f32>> {
    let width = base_noise.len();
    let height = base_noise[0].len();
    let mut smooth_noise = vec![vec![vec![0.0; width]; height]; octave_count];
    for i in 0..octave_count {
        smooth_noise[i] = generate_smooth_noise(base_noise.clone(), i);
    }
    let mut perlin_noise = vec![vec![0.0; width]; height];
    let mut amplitude = 1.0;
    let mut total_amplitude = 0.0;
    let persistence = 0.7;
    for octave in (0..octave_count).rev() {
        amplitude *= persistence;
        total_amplitude += amplitude;
        for i in 0..width {
            for j in 0..height {
                perlin_noise[j][i] += smooth_noise[octave][j][i] * amplitude;
            }
        }
    }
    for i in 0..width {
        for j in 0..height {
            perlin_noise[j][i] /= total_amplitude;
        }
    }
    return perlin_noise;
}
fn interpolate(x0: f32, x1: f32, alpha: f32) -> f32 {
    return x0 * (1.0 - alpha) + alpha * x1;
}
