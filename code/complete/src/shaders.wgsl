// Originally written in 2023 by Arman Uguray <arman.uguray@gmail.com>
// SPDX-License-Identifier: CC-BY-4.0

const FLT_MAX: f32 = 3.40282346638528859812e+38;
const EPSILON: f32 = 1e-3;
const TWO_PI: f32 = 6.2831853;

const MAX_PATH_LENGTH: u32 = 13u;
const GLASS_F0: f32 = 0.04;

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

// Convert RGB to a grayscale luminance value
fn luminance(rgb: vec3f) -> f32 {
  return dot(rgb, vec3(0.2126, 0.7152, 0.0722));
}

// Uniformly sample a unit sphere centered at the origin
fn sample_sphere() -> vec3f {
  let r0 = rand_f32();
  let r1 = rand_f32();

  // Map r0 to [-1, 1]
  let y = 1. - 2. * r0;

  // Compute the projected radius on the xz-plane using Pythagorean theorem
  let xz_r = sqrt(1. - y * y);

  let phi = TWO_PI * r1;
  return vec3(xz_r * cos(phi), y, xz_r * sin(phi));
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
  let t = select(t2, t1, t1 >= EPSILON);
  if t < EPSILON {
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

fn sample_lambertian(normal: vec3f) -> vec3f {
  return normal + sample_sphere() * (1. - EPSILON);
}

fn schlick_f0_from_ior(ior: f32) -> f32 {
  let sqrt_f0 = (ior - 1.) / (ior + 1.);
  return sqrt_f0 * sqrt_f0;
}

fn schlick_fresnel(f0: f32, cos_theta: f32) -> f32 {
  let u = 1 - cos_theta;
  return mix(f0, 1., u * u * u * u * u);
}

fn schlick_fresnel_vec3(f0: vec3f, cos_theta: f32) -> vec3f {
  let u = 1 - cos_theta;
  return mix(f0, vec3(1.), u * u * u * u * u);
}

fn scatter(input_ray: Ray, hit: Intersection, material: Material) -> Scatter {
  let incident = normalize(input_ray.direction);
  let incident_dot_normal = dot(incident, hit.normal);
  let is_front_face = incident_dot_normal < 0.;
  let N = select(-hit.normal, hit.normal, is_front_face);
  let cos_theta = abs(incident_dot_normal);

  // `ior` and `ref_ratio` only have meaning if the material is transmissive.
  let is_transmissive = material.metallic_or_ior < 0.;
  let ior = abs(material.metallic_or_ior);
  let ref_ratio = select(ior, 1. / ior, is_front_face);

  // Determine whether to use specular reflection.
  var is_specular = false;
  var attenuation = material.color;
  if is_transmissive {
    let cannot_refract = ref_ratio * ref_ratio * (1.0 - cos_theta * cos_theta) > 1.;
    let f0 = schlick_f0_from_ior(ref_ratio);
    is_specular = cannot_refract || schlick_fresnel(f0, cos_theta) > rand_f32();
    if is_specular {
      attenuation = vec3(1.);
    }
  } else if material.metallic_or_ior > 0. {
    let metallic = material.metallic_or_ior;
    let f0 = mix(vec3(GLASS_F0), material.color, metallic);
    let diffuse_color = material.color * (1. - metallic);

    let F = schlick_fresnel_vec3(f0, cos_theta);
    let specular = F;
    let diffuse = diffuse_color * (1. - F);

    let S = luminance(specular);
    let D = luminance(diffuse);
    let specular_pdf = S / (S + D);
    is_specular = specular_pdf > rand_f32();
    if is_specular {
      attenuation = specular / specular_pdf;
    } else {
      attenuation = diffuse / (1. - specular_pdf);
    }
  }

  var scattered: vec3f;
  if is_specular {
    scattered = reflect(incident, N);
  } else if is_transmissive {
    scattered = refract(incident, N, ref_ratio);
  } else {
    scattered = sample_lambertian(N);
  }
  let output_ray = Ray(point_on_ray(input_ray, hit.t), scattered);
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
  metallic_or_ior: f32,
}

fn sky_color(ray: Ray) -> vec3f {
  let t = 0.5 * (normalize(ray.direction).y + 1.);
  return (1. - t) * vec3(1.) + t * vec3(0.3, 0.5, 1.);
}

const OBJECT_COUNT: u32 = 7;
alias Scene = array<Sphere, OBJECT_COUNT>;
alias Materials = array<Material, OBJECT_COUNT>;

var<private> materials: Materials = Materials(
  Material(/*color*/ vec3(0.7, 0.5, 0.5), /*metallic_or_ior*/1.),
  Material(/*color*/ vec3(0.5, 0.5, 0.9), /*metallic_or_ior*/0.),
  Material(/*color*/ vec3(0.7, 0.9, 0.2), /*metallic_or_ior*/0.),
  Material(/*color*/ vec3(1.), /*metallic_or_ior*/-1.5),

  Material(/*color*/ vec3(1., 0.5, 0.), /*metallic_or_ior*/0.9),
  Material(/*color*/ vec3(1., 0.5, 0.), /*metallic_or_ior*/0.5),
  Material(/*color*/ vec3(1., 0.5, 0.), /*metallic_or_ior*/0.1),
);

var<private> scene: Scene = Scene(
  Sphere(/*center*/ vec3(-1.1, 0.5, 1.), /*radius*/ 0.5, /*material_index*/ 0),
  Sphere(/*center*/ vec3(0., 0.5, 1.),   /*radius*/ 0.5, /*material_index*/ 1),
  Sphere(/*center*/ vec3(1.1, 0.5, 1.),  /*radius*/ 0.5, /*material_index*/ 3),

  Sphere(/*center*/ vec3(-1.1, 0.5, -1.), /*radius*/ 0.5, /*material_index*/ 4),
  Sphere(/*center*/ vec3(0., 0.5, -1.),   /*radius*/ 0.5, /*material_index*/ 5),
  Sphere(/*center*/ vec3(1.1, 0.5, -1.),  /*radius*/ 0.5, /*material_index*/ 6),

  // Ground
  Sphere(/*center*/ vec3(0., -2e2 - EPSILON, 0.), /*radius*/ 2e2, /*material_index*/ 2),
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
