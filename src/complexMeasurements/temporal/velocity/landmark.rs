/*
This file is part of MOSAIC.

MOSAIC is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the Free 
Software Foundation, either version 3 of the License, or any later version.

MOSAIC is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR 
PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
MOSAIC. If not, see <https://www.gnu.org/licenses/>.
*/

pub struct LandmarkVelocity {
    // admin info
    pub frame: Vec<u32>,
    pub timestamp: Vec<f32>,
    pub confidence: Vec<f32>,
    pub pose: Vec<bool>, // Because some pose values might be 0, we need a seperate bool value to determine if we are processing 

    pub coordinate_number: Vec<u32>,
    pub types: Vec<String>, // we need to know whether or not this point was a commissure, philtrum, etc - defulat lip points can just be called "point"
    pub vx: Vec<f64>,
    pub vy: Vec<f64>,
    pub vz: Vec<f64>,
    pub dx: Vec<f64>,
    pub dy: Vec<f64>,
    pub dz: Vec<f64>,
}

impl LandmarkVelocity {
    pub fn construction(total_frames: u32, points_per_frame: u32)-> Self{
        // reserves the memory needed based on the frame count and the points per frame
        let total_entries = total_frames * points_per_frame;

        Self {
            frame: Vec::with_capacity(total_entries.try_into().unwrap()),
            timestamp: Vec::with_capacity(total_entries.try_into().unwrap()),
            confidence: Vec::with_capacity(total_entries.try_into().unwrap()),
            pose: Vec::with_capacity(total_entries.try_into().unwrap()),

            coordinate_number: Vec::with_capacity(total_entries.try_into().unwrap()),
            types: Vec::with_capacity(total_entries.try_into().unwrap()),

            vx: Vec::with_capacity(total_entries.try_into().unwrap()), // UNIT: displacement/s
            vy: Vec::with_capacity(total_entries.try_into().unwrap()), // UNIT: displacement/s
            vz: Vec::with_capacity(total_entries.try_into().unwrap()), // UNIT: displacement/s

            dx: Vec::with_capacity(total_entries.try_into().unwrap()), // UNIT: displacement/s
            dy: Vec::with_capacity(total_entries.try_into().unwrap()), // UNIT: displacement/s
            dz: Vec::with_capacity(total_entries.try_into().unwrap()), // UNIT: displacement/s
        }

    }

    pub fn add_point(&mut self, frame: u32, time: f32, confidence: f32, pose: bool,
                     number: u32, types: String, vx: f64, vy: f64, vz: f64, dx: f64, dy: f64, dz: f64) {
        
        self.frame.push(frame);
        self.timestamp.push(time);
        self.confidence.push(confidence);
        self.pose.push(pose);
        
        self.coordinate_number.push(number);
        self.types.push(types);

        self.dx.push(vx);
        self.dz.push(vz);
        self.dy.push(vy);
        self.dx_uncertainty.push(dx);
        self.dy_uncertainty.push(dy);
        self.dz_uncertainty.push(dz);
        
    }

    pub fn save_landmark_velocity_to_parquet(data: &LandmarkVelocity, file_path: &str) -> PolarsResult<()> {
        let s_frame = Series::new("frame", &data.frame);
        let s_time = Series::new("timestamp", &data.timestamp);
        let s_pose = Series::new("pose_detected", &data.pose);

        let s_num = Series::new("point_id", &data.coordinate_number);
        let s_type = Series::new("label", &data.types);

        let s_vx = Series::new("vx", &data.vx);
        let s_vy = Series::new("vy", &data.vy);
        let s_vz = Series::new("vz", &data.vz);

        let s_dx_ = Series::new("dx", &data.dx);
        let s_dy_ = Series::new("dy", &data.dy);
        let s_dz_ = Series::new("dz", &data.dz);

        let mut df = DataFrame::new(vec![
            s_frame, s_time,
            s_num, s_type, 
            s_vx, s_vy, s_vz,
            s_dx, s_dy, s_dz,
        ])?;

        let file = File::create(file_path).map_err(PolarsError::from)?;
        ParquetWriter::new(file).finish(&mut df)?;

        println!("Successfully exported Lanmark Velocity data to: {}", file_path);
        Ok(())
    }
}

pub struct CalculateVelocity;

impl CalculateVelocity{
    pub fn velocity(umd: &UMD) {
        let total_points = umd.frame.len();
        if total_points == 0 {
            return LandmarkVelocity::construction(0);
        }

        let mut velocity_data = LandmarkVelocity::construction(total_points / 68);

        while i < total_points {
            let start_idx = i;
            let current_frame = umd.frame[i];

            let dx: f64 = 0.0;
            let dy: f64 = 0.0;
            let dz: f64 = 0.0;

            let dx_uncertainty: f64 = 0.0;
            let dy_uncertainty: f64 = 0.0;
            let dz_uncertainty: f64 = 0.0

            if current_frame == 1 {

            }
        }
        
    }

}
