@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

fn hash(value: u32) -> u32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    return state;
}

fn randomFloat(value: u32) -> f32 {
    return f32(hash(value)) / 4294967295.0;
}

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let randomNumber = randomFloat(invocation_id.y * num_workgroups.x + invocation_id.x);
    let alive = randomNumber > 0.9;
    let color = vec4<f32>(f32(alive));


    textureStore(texture, location, textureLoad(texture, location));
}

fn is_alive(location: vec2<i32>, offset_x: i32, offset_y: i32) -> i32 {
    let value: vec4<f32> = textureLoad(texture, location + vec2<i32>(offset_x, offset_y));
    return i32(value.x);
}

fn count_alive(location: vec2<i32>) -> i32 {
    return is_alive(location, -1, -1) +
           is_alive(location, -1,  0) +
           is_alive(location, -1,  1) +
           is_alive(location,  0, -1) +
           is_alive(location,  0,  1) +
           is_alive(location,  1, -1) +
           is_alive(location,  1,  0) +
           is_alive(location,  1,  1);
}

fn sums(location: vec2<i32>) -> vec4<f32> {
    let left = textureLoad(texture, location + vec2<i32>(-1, 0));
    let right = textureLoad(texture, location + vec2<i32>(1, 0));
    let current = textureLoad(texture, location + vec2<i32>(0, 0));

    if current[3] == 0.0 {
        // Do nothing
        return current;
    } else if left[3] == 0.0 {
        // add 
        let current_bytes = ivec4(current * 255);
        let right_bytes = ivec4(right * 255);

        let current_integer = (current_bytes.r << 16) | (current_bytes.g << 8) | current_bytes.a;
        let right_integer = (right_bytes.r << 16) | (right_bytes.g << 8) | right_bytes.a;
        let next = current_integer + right_integer;
        let next_bytes = ivec4((next >> 16) && 0xFF, (next >> 8) && 0xFF, next && 0xFF, 255);
        return fvec4(next_bytes / 255);
    } else {
        // replace with right
        return right;
    }
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let n_alive = count_alive(location);

    var alive: bool;
    if (n_alive == 3) {
        alive = true;
    } else if (n_alive == 2) {
        let currently_alive = is_alive(location, 0, 0);
        alive = bool(currently_alive);
    } else {
        alive = false;
    }
    let color = sums(location);

    storageBarrier();

    textureStore(texture, location, color);
}