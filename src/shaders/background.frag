#version 450

layout(location = 0) in vec4 v_Position;
layout(location = 1) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

void main() {
    o_Target = vec4(v_Uv, 0.1, 1.0);
}
