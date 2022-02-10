#version 330 core
layout (location = 0) out vec4 FragColor;

uniform vec3 albedo;

void main()
{
    FragColor = vec4(albedo, 1.0);
}