@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;
@group(0) @binding(1)
var<storage,read_write> input_array: array<u32>;
@group(0) @binding(2)
var<storage,read_write> output_array: array<u32>;

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let idx = invocation_id.y * u32(100) + invocation_id.x;
    let value = input_array[idx];
    let r = u32((value >> u32(16))) & u32(0xFF);
    let g = u32(value >> u32(8)) & u32(0xFF);
    let b = u32((value)) & u32(0xFF);

    let color = vec4<f32>(f32(r) / 256.0, f32(g) / 256.0,f32(b) / 256.0,1.0);

    textureStore(texture, location, color);
}


@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    // let color = vec2<f32>(position.x % 256.0 / 256.0, position.y % 129.0 / 129.0);
    // picked_color[0] = color.r;
    // picked_color[1] = color.g;
    // picked_color[2] = 0.0;
} 