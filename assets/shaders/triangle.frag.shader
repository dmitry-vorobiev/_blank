#version 330 core

in VS_OUTPUT {
    vec3 Color;
} IN;

out vec4 Color;

uniform sampler2D u_texture;

void main()
{
    Color = vec4(IN.Color.xyz, 1.0);
}