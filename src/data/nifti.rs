use crate::utils;
use futures::io::Error;
use serde::Serialize;
use std::iter::FromIterator;
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
extern "C" {
    /// Log a string value to the console.
    #[allow(unused)]
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct Nifti1Header {
    sizeof_hdr: i32,
    data_type: [u8; 10],
    db_name: [u8; 18],
    extents: i32,
    session_error: i16,
    regular: u8,
    dim_info: u8,
    dim: [i16; 8],
    intent_p1: f32,
    intent_p2: f32,
    intent_p3: f32,
    intent_code: i16,
    datatype: i16,
    bitpix: i16,
    slice_start: i16,
    pixdim: [f32; 8],
    vox_offset: f32,
    scl_slope: f32,
    scl_inter: f32,
    slice_end: i16,
    slice_code: u8,
    xyzt_units: u8,
    cal_max: f32,
    cal_min: f32,
    slice_duration: f32,
    toffset: f32,
    glmax: i32,
    glmin: i32,
    #[serde(serialize_with = "<[_]>::serialize")]
    descrip: [u8; 80],
    aux_file: [u8; 24],
    qform_code: i16,
    sform_code: i16,
    quatern_b: f32,
    quatern_c: f32,
    quatern_d: f32,
    qoffset_x: f32,
    qoffset_y: f32,
    qoffset_z: f32,
    srow_x: [f32; 4],
    srow_y: [f32; 4],
    srow_z: [f32; 4],
    intent_name: [u8; 16],
    magic: [u8; 4],
}

#[derive(Serialize)]
pub struct NiftiData {
    pub header: Nifti1Header,
    pub image: Vec<u8>,
    pub texture: Vec<[u8; 4]>
}

#[wasm_bindgen]
impl Nifti1Header {
    #[wasm_bindgen]
    pub async fn load_from_url(url: String) -> Result<JsValue, JsError> {
        let binary_data: Vec<u8>;
        match utils::fetch_binary(url.clone()).await {
            Ok(buffer) => binary_data = buffer,
            Err(_e) => {
                return Err(JsError::new(
                    format!("could not load url: {}", url.clone()).as_str(),
                ))
            }
        };

        let nifti_data = Vec::from_iter(binary_data[0..348].iter().cloned());
        log(format!("nifti_data is sliced and {} bytes", nifti_data.len()).as_str());
        let (head, body, _tail) = unsafe { nifti_data.align_to::<Nifti1Header>() };
        assert!(head.is_empty(), "Data was not aligned");
        log("data aligned");
        let nifti_header = &body[0];
        // log(serde_json::to_string(&nifti_header).unwrap().as_str());
        log("checking descrip");
        let descrip = nifti_header.descrip.to_vec();
        log(format!("descrip is {} bytes", descrip.len()).as_str());
        match String::from_utf8(nifti_header.descrip.to_vec()) {
            Ok(string) => log(format!("{}", string).as_str()),
            Err(e) => log(format!("Error: {}", e).as_str()),
        }
        Ok(serde_wasm_bindgen::to_value(&nifti_header).unwrap())
    }
    
    #[wasm_bindgen]
    pub async fn load_and_create_texture_from_url(url: String) -> Result<JsValue, JsError> {
        // Load the entire binary data from the URL
        let binary_data: Vec<u8>;
        match utils::fetch_binary(url.clone()).await {
            Ok(buffer) => binary_data = buffer,
            Err(_e) => {
                return Err(JsError::new(
                    format!("could not load url: {}", url.clone()).as_str(),
                ))
            }
        };

        // Extract the NIfTI header from the binary data
        let nifti_data = Vec::from_iter(binary_data[0..348].iter().cloned());
        let (head, body, _tail) = unsafe { nifti_data.align_to::<Nifti1Header>() };
        assert!(head.is_empty(), "Data was not aligned");
        let nifti_header = &body[0];

        // Read the voxel data using the voxel offset
        let voxel_offset = nifti_header.vox_offset as usize;
        let voxel_data = binary_data.get(voxel_offset..);

        // Assuming you have a color lookup table (LUT)
        let color_lut = generate_color_lookup_table();

        // Calculate the size of the 3D texture
        let width = (nifti_header.dim[1] as f32 * nifti_header.pixdim[1]) as usize;
        let height = (nifti_header.dim[2] as f32 * nifti_header.pixdim[2]) as usize;
        let depth = (nifti_header.dim[3] as f32 * nifti_header.pixdim[3]) as usize;

        // Create a 3D texture using the voxel data and apply the color lookup table
        let texture_data = create_3d_texture_from_voxel_data(width, height, depth, voxel_data, color_lut);

        // Convert the NIfTI header and texture data to JsValue
        // let nifti_header_js = serde_wasm_bindgen::to_value(nifti_header).unwrap();
        // let texture_data_js = serde_wasm_bindgen::to_value(&texture_data).unwrap();
        let nifti_data_struct = NiftiData {header: *nifti_header, image: nifti_data, texture: texture_data};
        let nifti_data_js = serde_wasm_bindgen::to_value(&nifti_data_struct);
        // Ok((nifti_header_js, texture_data_js))
        Ok(nifti_data_js.unwrap())
    }
   
}

 // Helper function to generate a color lookup table (LUT)
 fn generate_color_lookup_table() -> Vec<[u8; 4]> {
    // Implement your color lookup table generation logic here
    // For simplicity, let's assume a grayscale LUT
    (0..256)
        .map(|intensity| [intensity as u8, intensity as u8, intensity as u8, 255])
        .collect()
}

// Helper function to create a 3D texture from voxel data and apply a color lookup table
fn create_3d_texture_from_voxel_data(
    width: usize,
    height: usize,
    depth: usize,
    voxel_data: Option<&[u8]>,
    color_lut: Vec<[u8; 4]>,
) -> Vec<[u8; 4]> {
    // Implement your logic to create a 3D texture and apply the color lookup table
    // ...

    // For simplicity, let's return a placeholder texture
    vec![[0, 0, 0, 255]; width * height * depth]
}

// #[derive(Serialize, Deserialize)]
// pub struct Nifti1HeaderJS {
//   sizeof_hdr: i32,
//   data_type: [u8; 10],
//   db_name: [u8; 18],
//   extents: i32,
//   session_error: i16,
//   regular: u8,
//   dim_info: u8,
//   dim: [i16; 8],
//   intent_p1: f32,
//   intent_p2: f32,
//   intent_p3: f32,
//   intent_code: i16,
//   datatype: i16,
//   bitpix: i16,
//   slice_start: i16,
//   pixdim: [f32; 8],
//   vox_offset: f32,
//   scl_slope: f32,
//   scl_inter: f32,
//   slice_end: i16,
//   slice_code: u8,
//   xyzt_units: u8,
//   cal_max: f32,
//   cal_min: f32,
//   slice_duration: f32,
//   toffset: f32,
//   glmax: i32,
//   glmin: i32,
//   descrip: [u8; 80],
//   aux_file: [u8; 24],
//   qform_code: i16,
//   sform_code: i16,
//   quatern_b: f32,
//   quatern_c: f32,
//   quatern_d: f32,
//   qoffset_x: f32,
//   qoffset_y: f32,
//   qoffset_z: f32,
//   srow_x: [f32; 4],
//   srow_y: [f32; 4],
//   srow_z: [f32; 4],
//   intent_name: [u8; 16],
//   magic: [u8; 4],
// }

// impl From<Nifti1Header> for Nifti1HeaderJS {
//   fn from(a: Nifti1Header) -> Self {
//     let serialised = serde_json::to_string(&a).unwrap();
//     serde_json::from_str(&serialised).unwrap()
//   }
// }

// impl Nifti1HeaderJS {
//   #[wasm_bindgen]
//   pub async fn load_from_url(url: String) -> Result<JsValue, JsValue> {

//     let nifti_header = match Nifti1Header::load_from_url(url).await {
//       Ok(nifti_header) => nifti_header = nifti_header,
//       Err(e) => Err(JsValue::FALSE)
//     };
//     let nifti_header_js = nifti_header as Nifti1HeaderJS;
//     Ok(serde_wasm_bindgen::to_value(&nifti_header_js))
//   }
// }
