#version 330 core

in VS_OUTPUT {
    vec2 Tex;
} IN;

out vec4 Color;

uniform sampler2D u_texture;

void main()
{
    Color = texture(u_texture, IN.Tex);

    if (Color.w < 1.0f)
        discard;
}