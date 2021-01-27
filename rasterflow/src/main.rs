fn main() {
    const NROWS: usize = 4;
    const NCOLS: usize = 4;
    const CELLSIZE: i32 = 1;
    
    // Time Step length, output ones and simulation duration one in seconds
    const DT: f32 = 0.1; 
    const DT_OUT: f32 = 1.0;
    const DT_END: f32 = 60.0;

    // Digital Elevation Model in meters
    let dem = [[1.0, 1.2, 1.1, 1.0], 
               [1.4, 1.6, 1.3, 1.1], 
               [1.1, 1.3, 1.0, 0.8],
               [1.0, 1.2, 1.0, 1.1]];

    // Water depth in meters at t0
    let depth0 = [[0.05, 0.05, 0.05, 0.05],
                  [0.05, 0.05, 0.05, 0.05], 
                  [0.05, 0.05, 0.05, 0.05], 
                  [0.05, 0.05, 0.05, 0.05]];

    // Roughness values 
    let kst = [[10., 10., 10., 10.],
               [10., 10., 10., 10.],
               [10., 10., 10., 10.],
               [10., 10., 10., 10.]];
    
    // Calculate initial water levels
    let mut water_levels = [[0.0; NCOLS]; NROWS];
    for ri in 0..NROWS {
        for ci in 0..NCOLS {
            water_levels[ri][ci] = dem[ri][ci] + depth0[ri][ci];
        }
    }

    let mut dt_now = 0.0;
    while dt_now <= DT_END {
        for ri in 0..NROWS {
            for ci in 0..NCOLS {
               let tilt_vector = calc_water_level_tilt(water_levels, ri, ci, 
                                                       NROWS, NCOLS);
            }
        }
        dt_now += DT;
    }


    

    println!("Hello from RasterFlow!");
}


fn calc_hyd_radius(depth: &f32, cell_size: &f32) -> f32 {
    cell_size*depth
}
