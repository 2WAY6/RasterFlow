fn main() {
    const NROWS: usize = 4;
    const NCOLS: usize = 4;
    const CELLSIZE: i32 = 1;
    const G: f32 = 9.81;
    
    // Time Step length, output ones and simulation duration one in seconds
    const DT: f32 = 0.1; 
    const DT_OUT: f32 = 1.0;
    const DT_END: f32 = 60.0;

    // Digital Elevation Model in meters
    let terrain = [[1.0, 1.2, 1.1, 1.0], 
                   [1.4, 1.6, 1.3, 1.1], 
                   [1.1, 1.3, 1.0, 0.8],
                   [1.0, 1.2, 1.0, 1.1]];

    // Water depth in meters at t0
    let mut depth = [[0.05, 0.05, 0.05, 0.05],
                     [0.05, 0.05, 0.05, 0.05], 
                     [0.05, 0.05, 0.05, 0.05], 
                     [0.05, 0.05, 0.05, 0.05]];

    let mut velocity = [[[0., 0.], [0., 0.], [0., 0.]],
                        [[0., 0.], [0., 0.], [0., 0.]],
                        [[0., 0.], [0., 0.], [0., 0.]]];
    
    // Roughness values 
    let kst = [[10., 10., 10., 10.],
               [10., 10., 10., 10.],
               [10., 10., 10., 10.],
               [10., 10., 10., 10.]];
    
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
            energy[ri][ci] = bernoulli(terrain[ri][ci],
                                       depth[ri][ci],
                                       velocity[ri][ci],
                                       9.81);
        }
    }



    let mut dt_now = 0.0;
    while dt_now <= DT_END {
        for ri in 0..NROWS {
            for ci in 0..NCOLS {
                let mut local_energy = [[0.0; 3]; 3];

                for i in ri-1..ri+1 {
                    for j in ci-1..ci+1 {
                        if ri < 0 || ri >= NROWS || ci < 0 || ci >= NCOLS {
                            local_energy[i][j] = 0.0; 
                        } else {
                            local_energy[i][j] = bernoulli(&terrain[i][j], 
                                                           &depth[i][j],
                                                           &velocity[i][j],
                                                           &G);
                        }
                    }
                }

                let tilt_vector = energy_tilt(&local_energy, &ri, &ci, &NROWS, 
                                              &NCOLS);
            }
        }
        dt_now += DT;
    }


    

    println!("Hello from RasterFlow!");
}



fn bernoulli(z: &f32, h: &f32, v: &[f32], g: &f32) -> f32 {
    let v_res = (v[0]*v[0] + v[1]*v[1]).sqrt();
    z + h + (v_res*v_res)/2.0/g
}


fn energy_tilt(energy: &[[f32; 3]; 3], ri: &usize, ci: &usize, n_rows: &usize,
               n_cols: &usize) {

}