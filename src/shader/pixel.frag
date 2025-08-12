#version 330 core

in vec2 TexCoords;
in vec4 VertexColor;

out vec4 color;

uniform sampler2D text;

void main() {

    if (TexCoords.x <= 0.0) {
        color = VertexColor;
    } else {
        color = vec4(VertexColor.rgb, VertexColor.a * texture(text, TexCoords).r);
    }


    // float texValue = texture(text, TexCoords).r;
    // color = VertexColor * texValue; //vec4(texValue, texValue, texValue, 1.0);
}
