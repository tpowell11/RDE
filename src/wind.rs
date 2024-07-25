use serde::{Deserialize, Serialize};
use crate::types::{float, v2, lerp};
#[derive(Serialize, Debug, Deserialize)]
pub struct Wind {
    data: std::collections::HashMap<u32, v2>
}
impl Wind {
    pub fn get_interpolated (self, h:u32) -> v2  {
        //! Returns interpolated data from the hash map of wind points.
        //! Currently uses linear interpolation, but will be upgraded to
        //! cubic spline interpolation eventually.
        let existing_points = self.data.keys();
        let mut p0 : (u32, v2) = (0,(0.0,0.0));
        let mut p1 : (u32, v2) = (u32::MAX,(float::MAX,float::MAX));
        for key in existing_points {
            if *key < h && *key > p0.0 {
                // if key is below the req. altitude and key is greater than the current lower altitude
                p0.0 = *key;
                p0.1 = *self.data.get(key).unwrap()

            }
            if *key > h && *key < p1.0 {
                // if key is higher than req. height and key is less than current upper altitude
                p1.0 = *key;
                p1.1 = *self.data.get(key).unwrap()
            }
        }
        let wind_x = lerp((p0.0 as f32 ,p0.1.0), (p1.0 as f32, p1.1.0), h as f32);
        let wind_y = lerp((p0.0 as f32, p0.1.1), (p1.0 as f32, p1.1.1), h as f32);
        return (wind_x, wind_y);
    }
}