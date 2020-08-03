precision highp float;
attribute vec4 position;

varying vec2 uv;

void main() {
    uv = position.zw;
    gl_Position = vec4(position.x, position.y, 0.0, 1.0);
}
