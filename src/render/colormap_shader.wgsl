// Transformation shader to convert grayscale values into RGBA format

// Input buffer containing grayscale values
[[block]]
struct InputBuffer {
    data: array<u8>;
};

// Output buffer containing RGBA values
[[block]]
struct OutputBuffer {
    data: array<vec4<f32>>;
};

// Transformation function
fn transform(input: u8) -> vec4<f32> {
    // Map grayscale value to RGBA color with full alpha
    let gray = f32(input) / 255.0;
    return vec4<f32>(gray, gray, gray, 1.0);
}

// Main entry point
[[stage(vertex)]]
fn main() {
    // Get the global position in the 3D texture
    let position = global_position();

    // Calculate the linear index from the 3D position
    let index = position.z * (width * height) + position.y * width + position.x;

    // Read the grayscale value from the input buffer
    let input_value = InputBuffer.data[index];

    // Perform the transformation
    let output_value = transform(input_value);

    // Write the RGBA value to the output buffer
    OutputBuffer.data[index] = output_value;
}
