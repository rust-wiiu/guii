#version 330 core

layout(location = 0) in vec3 vertex;
layout(location = 1) in vec2 tex;
layout(location = 2) in vec4 color; 

out vec2 TexCoords;
out vec4 VertexColor;

uniform mat4 projection;

void main() {
    gl_Position = projection * vec4(vertex.xyz, 1.0);
    TexCoords = tex;
    VertexColor = color;
}
