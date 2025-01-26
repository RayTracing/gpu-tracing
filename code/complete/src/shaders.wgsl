// Originally written in 2023 by Arman Uguray <arman.uguray@gmail.com>
// SPDX-License-Identifier: CC-BY-4.0

const FLT_MAX: f32 = 3.40282346638528859812e+38;
const EPSILON: f32 = 1e-2;
const PI: f32 = 3.14159265358979323846264338327950288;
const TWO_PI: f32 = 2. * PI;

const MAX_PATH_LENGTH: u32 = 13u;

struct Uniforms {
  camera: CameraUniforms,
  width: u32,
  height: u32,
  frame_count: u32,
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct CameraUniforms {
  origin: vec3f,
  u: vec3f,
  v: vec3f,
  w: vec3f,
}

struct Rng {
  state: u32,
}
var<private> rng: Rng;

fn init_rng(pixel: vec2u) {
  // Seed the PRNG using the scalar index of the pixel and the current frame count.
  let seed = (pixel.x + pixel.y * uniforms.width) ^ jenkins_hash(uniforms.frame_count);
  rng.state = jenkins_hash(seed);
}

// A slightly modified version of the "One-at-a-Time Hash" function by Bob Jenkins.
// See https://www.burtleburtle.net/bob/hash/doobs.html
fn jenkins_hash(i: u32) -> u32 {
  var x = i;
  x += x << 10u;
  x ^= x >> 6u;
  x += x << 3u;
  x ^= x >> 11u;
  x += x << 15u;
  return x;
}

// The 32-bit "xor" function from Marsaglia G., "Xorshift RNGs", Section 3.
fn xorshift32() -> u32 {
  var x = rng.state;
  x ^= x << 13;
  x ^= x >> 17;
  x ^= x << 5;
  rng.state = x;
  return x;
}

// Returns a random float in the range [0...1]. This sets the floating point exponent to zero and
// sets the most significant 23 bits of a random 32-bit unsigned integer as the mantissa. That
// generates a number in the range [1, 1.9999999], which is then mapped to [0, 0.9999999] by
// subtraction. See Ray Tracing Gems II, Section 14.3.4.
fn rand_f32() -> f32 {
  return bitcast<f32>(0x3f800000u | (xorshift32() >> 9u)) - 1.;
}

fn sample_sphere(u: vec2f) -> vec3f {
  let z = 1. - 2. * u.x;
  let r = sqrt(1. - z * z);
  let phi = TWO_PI * u.y;
  return vec3(r * cos(phi), r * sin(phi), z);
}

struct Intersection {
  normal: vec3f,
  t: f32,
  material_index: u32,
}

fn no_intersection() -> Intersection {
  return Intersection(vec3(0.), -1., 0);
}

fn is_intersection_valid(hit: Intersection) -> bool {
  return hit.t > 0.;
}

struct Sphere {
  center: vec3f,
  radius: f32,
  material_index: u32,
}

fn intersect_sphere(ray: Ray, sphere: Sphere) -> Intersection {
  let v = ray.origin - sphere.center;
  let a = dot(ray.direction, ray.direction);
  let b = dot(v, ray.direction);
  let c = dot(v, v) - sphere.radius * sphere.radius;

  let d = b * b - a * c;
  if d < 0. {
    return no_intersection();
  }

  let sqrt_d = sqrt(d);
  let recip_a = 1. / a;
  let mb = -b;
  let t1 = (mb - sqrt_d) * recip_a;
  let t2 = (mb + sqrt_d) * recip_a;
  let t = select(t2, t1, t1 > EPSILON);
  if t <= EPSILON {
    return no_intersection();
  }

  let p = point_on_ray(ray, t);
  let N = (p - sphere.center) / sphere.radius;
  return Intersection(N, t, sphere.material_index);
}

fn intersect_scene(ray: Ray) -> Intersection {
  var closest_hit = no_intersection();
  closest_hit.t = FLT_MAX;
  for (var i = 0u; i < OBJECT_COUNT; i += 1u) {
    let sphere = scene[i];
    let hit = intersect_sphere(ray, sphere);
    if hit.t > 0. && hit.t < closest_hit.t {
      closest_hit = hit;
    }
  }
  if closest_hit.t < FLT_MAX {
    return closest_hit;
  }
  return no_intersection();
}

struct Scatter {
  attenuation: vec3f,
  ray: Ray,
}

fn scatter(input_ray: Ray, hit: Intersection, material: Material) -> Scatter {
  let is_front_face = dot(hit.normal, input_ray.direction) < 0.;
  let N = select(-hit.normal, hit.normal, is_front_face);
//  let u = vec2(rand_f32(), rand_f32());
//  let reflected = normalize(N + sample_sphere(u));
  let reflected = reflect(input_ray.direction, N);
  let output_ray = Ray(point_on_ray(input_ray, hit.t) + N * EPSILON, reflected);
  let attenuation = material.color;
  return Scatter(attenuation, output_ray);
}

struct Ray {
  origin: vec3f,
  direction: vec3f,
}

fn point_on_ray(ray: Ray, t: f32) -> vec3<f32> {
  return ray.origin + t * ray.direction;
}

struct Material {
  color: vec3f,
}

fn sky_color(ray: Ray) -> vec3f {
  let t = 0.5 * (normalize(ray.direction).y + 1.);
  return (1. - t) * vec3(1.) + t * vec3(0.3, 0.5, 1.);
}

const OBJECT_COUNT: u32 = 2;
alias Scene = array<Sphere, OBJECT_COUNT>;
alias Materials = array<Material, 1>;

var<private> materials: Materials = Materials(
  Material(/*color*/ vec3(0.5)),
);

var<private> scene: Scene = Scene(
  Sphere(/*center*/ vec3(0., 0.5, 0.), /*radius*/ 0.5, /*material_index*/ 0),
  Sphere(/*center*/ vec3(0., -2e4, -1.), /*radius*/ 2e4, /*material_index*/ 0),
);

@group(0) @binding(1) var radiance_samples_old: texture_2d<f32>;
@group(0) @binding(2) var radiance_samples_new: texture_storage_2d<rgba32float, write>;

alias TriangleVertices = array<vec2f, 6>;
var<private> vertices: TriangleVertices = TriangleVertices(
  vec2f(-1.0,  1.0),
  vec2f(-1.0, -1.0),
  vec2f( 1.0,  1.0),
  vec2f( 1.0,  1.0),
  vec2f(-1.0, -1.0),
  vec2f( 1.0, -1.0),
);

@vertex fn display_vs(@builtin(vertex_index) vid: u32) -> @builtin(position) vec4f {
  return vec4f(vertices[vid], 0.0, 1.0);
}

@fragment fn display_fs(@builtin(position) pos: vec4f) -> @location(0) vec4f {
  init_rng(vec2u(pos.xy));

  let origin = uniforms.camera.origin;
  let focus_distance = 1.;
  let aspect_ratio = f32(uniforms.width) / f32(uniforms.height);

  // Offset and normalize the viewport coordinates of the ray.
  let offset = vec2(rand_f32() - 0.5, rand_f32() - 0.5);
  var uv = (pos.xy + offset) / vec2f(f32(uniforms.width - 1u), f32(uniforms.height - 1u));

  // Map `uv` from y-down (normalized) viewport coordinates to camera coordinates.
  uv = (2. * uv - vec2(1.)) * vec2(aspect_ratio, -1.);

  // Compute the scene-space ray direction by rotating the camera-space vector into a new
  // basis.
  let camera_rotation = mat3x3(uniforms.camera.u, uniforms.camera.v, uniforms.camera.w);
  let direction = camera_rotation * vec3(uv, focus_distance);
  var ray = Ray(origin, direction);
  var throughput = vec3f(1.);
  var radiance_sample = vec3(0.);

  var path_length = 0u;
  while path_length < MAX_PATH_LENGTH {
    let hit = intersect_scene(ray);
    if !is_intersection_valid(hit) {
      // If no intersection was found, return the color of the sky and terminate the path.
      radiance_sample += throughput * sky_color(ray);
      break;
    }

    let material = materials[hit.material_index];
    let scattered = scatter(ray, hit, material);
    throughput *= scattered.attenuation;
    ray = scattered.ray;
    path_length += 1u;
  }

  // Fetch the old sum of samples.
  var old_sum: vec3f;
  if uniforms.frame_count > 1 {
    old_sum = textureLoad(radiance_samples_old, vec2u(pos.xy), 0).xyz;
  } else {
    old_sum = vec3(0.);
  }

  // Compute and store the new sum.
  let new_sum = radiance_sample + old_sum;
  textureStore(radiance_samples_new, vec2u(pos.xy), vec4(new_sum, 0.));

  // Display the average after gamma correction (gamma = 2.2)
  let color = new_sum / f32(uniforms.frame_count);
  return vec4(pow(color, vec3(1. / 2.2)), 1.);
}
