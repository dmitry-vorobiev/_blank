#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;

out VS_OUTPUT {
    vec3 Color;
} OUT;

uniform mat4 ModelMatrix = mat4(1.0);

void main()
{
    gl_Position = ModelMatrix * vec4(Position, 1.0);
    OUT.Color = Color.xyz;
}