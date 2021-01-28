fn main() {
    const NROWS: usize = 40;
    const NCOLS: usize = 40;
    const CELLSIZE: i32 = 1;
    const G: f32 = 9.81;
    
    // Time Step length, output ones and simulation duration one in seconds
    const DT: f32 = 0.01; 
    const DT_OUT: f32 = 1.0;
    const DT_END: f32 = 60.0;

    let depth_0 = 0.1;
    let kst_default = 10.;

    // Digital Elevation Model in meters
    let center_x: f32 = 10.;
    let center_y: f32 = 10.;
    let mut terrain = [[0.0; NCOLS]; NROWS];
    let mut i: f32 = 0.;
    let mut j: f32 = 0.;
    for ri in 0..NROWS {
        for ci in 0..NCOLS {
            let z: f32 = ((i - center_x).abs() + (j - center_y).abs()).sqrt().sqrt();
            terrain[ri][ci] = z;
            i += 0.1;
        }
        j += 0.1;
    }

    // Water depth in meters at t0
    let mut depth = [[0.0; NCOLS]; NROWS];
    for ri in 0..NROWS {
        for ci in 0..NCOLS {
            depth[ri][ci] = depth_0;
        }
    }

    // Initial velocity vectors
    let mut velocity = [[[0.0; 2]; NCOLS]; NROWS];
    for ri in 0..NROWS {
        for ci in 0..NCOLS {
            velocity[ri][ci][0] = 0.;
            velocity[ri][ci][1] = 0.;
        }
    }
    
    // Default roughness values 
    let mut kst = [[0.0; NCOLS]; NROWS];
    for ri in 0..NROWS {
        for ci in 0..NCOLS {
            kst[ri][ci] = kst_default;
        }
    }
       
    // Calculate initial water levels
    let mut water_level = [[0.0; NCOLS]; NROWS];
    for ri in 0..NROWS {
        for ci in 0..NCOLS {
            water_level[ri][ci] = terrain[ri][ci] + depth[ri][ci];
        }
    }

    // Caluclate the energy height with bernoulli
    let mut energy = [[0.0; NCOLS]; NROWS];
    for ri in 0..NROWS {
        for ci in 0..NCOLS {
            energy[ri][ci] = bernoulli(&terrain[ri][ci],
                                       &depth[ri][ci],
                                       &velocity[ri][ci],
                                       &G);
        }
    }

    // -------------------------------------------------------------------------
    // Run test simulation

    let mut dt_now = 0.0;
    while dt_now <= DT_END {
        println!("{}", depth[2][20]);

        for ri in 1..(NROWS-1) {
            for ci in 1..(NCOLS-1) {
                let mut local_energy = [[0.0; 3]; 3];
                
                // Calc energy gradients
                let grad_north = energy[ri][ci] - energy[ri-1][ci];
                let grad_east = energy[ri][ci] - energy[ri][ci+1];
                let grad_south = energy[ri][ci] - energy[ri+1][ci];
                let grad_west = energy[ri][ci] - energy[ri][ci-1]; 
                let gradients = [grad_north, grad_east, grad_south, grad_west];

                // Find biggest gradient
                let mut max_grad = 0.0;
                let mut max_i = 0;
                for i in 0..4 {
                    if gradients[i] > max_grad {
                        max_grad = gradients[i];
                        max_i = i;
                    }
                }

                // continue, if the cell is the lowest 
                if max_grad == 0. {
                    continue;
                }

                let mut rn = ri.clone();
                let mut cn = ci.clone();
                if max_i == 0 {
                    rn -= 1;
                } else if max_i == 1 {
                    cn += 1;
                } else if max_i == 2 {
                    rn += 1;
                } else {
                    cn -= 1;
                }

                let kst_mean = 0.5 * (kst[ri][ci] + kst[rn][cn]);
                let rhyd_mean = 0.5 * (depth[ri][ci] + depth[rn][cn] + 1.);
                let area = 0.5 * (depth[ri][ci] + depth[rn][cn]);

                let veloc = kst_mean * rhyd_mean.powf(2./3.) * max_grad.sqrt();
                let flow = veloc * area;
                let vol = flow * DT;

                if vol <= depth[ri][ci] {
                    depth[rn][cn] += vol;
                    depth[ri][ci] -= vol;
                } else {
                    depth[rn][cn] += depth[ri][ci];
                    depth[ri][ci] = 0.;
                }
            }
        }
        dt_now += DT;
    }

    let mut max_depth_ri = 0;
    let mut max_depth_ci = 0;
    let mut max_depth = 0.0;
    for ri in 0..NROWS {
        for ci in 0..NCOLS {
            if depth[ri][ci] > max_depth {
                max_depth = depth[ri][ci];
                max_depth_ri = ri;
                max_depth_ci = ci;
            }
        }
    }
    println!("{} {}", max_depth_ri, max_depth_ci);

    

    println!("Hello from RasterFlow!");
}



fn bernoulli(z: &f32, h: &f32, v: &[f32], g: &f32) -> f32 {
    let v_res = (v[0]*v[0] + v[1]*v[1]).sqrt();
    z + h + (v_res*v_res)/2.0/g
}


// Calculate the energy tilt with four adjacent cells
// fn energy_tilt_4(energy: &[[f32; 3]; 3], ri: &usize, ci: &usize, n_rows: &usize,
//                  n_cols: &usize) {
//     let mut energy_tilt = [0.0, 0.0, 0.0];
//     energy_tilt[0] -= 1.0;
//     energy_tilt[2]  
// }