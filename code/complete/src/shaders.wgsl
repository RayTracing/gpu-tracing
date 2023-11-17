// Originally written in 2023 by Arman Uguray <arman.uguray@gmail.com>
// SPDX-License-Identifier: CC-BY-4.0

alias QuadVertices = array<vec2f, 6>;
var<private> vertices: QuadVertices = QuadVertices(
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

const WIDTH: u32 = 800u;
const HEIGHT: u32 = 600u;

@fragment fn display_fs(@builtin(position) pos: vec4f) -> @location(0) vec4f {
  let color = pos.xy / vec2f(f32(WIDTH - 1u), f32(HEIGHT - 1u));
  return vec4f(color, 0.0, 1.0);
}
