fn main() {
    const NROWS: usize = 20;
    const NCOLS: usize =20;
    const CELLSIZE: i32 = 1;
    const G: f32 = 9.81;
    
    // Time Step length, output ones and simulation duration one in seconds
    const DT: f32 = 0.1; 
    const DT_OUT: f32 = 1.0;
    const DT_END: f32 = 60.0;

    let depth_0 = 0.1;
    let kst_default = 10.;

    // Digital Elevation Model in meters
    let mut terrain = [[0.0; NCOLS]; NROWS];
    let mut i = 0.;
    let mut j = 0.;
    for ri in 0..NROWS {
        for ci in 0..NCOLS {
            let z: f32 = i + j;
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
        for ri in 1..(NROWS-1) {
            for ci in 1..(NCOLS-1) {
                let mut local_energy = [[0.0; 3]; 3];

                for i in 0..3 {
                    for j in 0..3 {
                        local_energy[i][j] = energy[ri+i-1][ci+j-1];
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