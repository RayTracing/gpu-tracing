alias TriangleVertices = array<vec2f, 3>;
var<private> vertices: TriangleVertices = TriangleVertices(
  vec2f(-0.5, -0.5),
  vec2f( 0.5, -0.5),
  vec2f( 0.0,  0.5),
);

@vertex fn display_vs(@builtin(vertex_index) vid: u32) -> @builtin(position) vec4f {
  return vec4f(vertices[vid], 0.0, 1.0);
}

@fragment fn display_fs() -> @location(0) vec4f {
  return vec4f(1.0, 0.0, 0.0, 1.0);
}
