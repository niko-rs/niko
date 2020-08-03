precision highp float;
varying vec2 uv;

uniform sampler2D sprite;
uniform vec4 color;

void main()
{
    gl_FragColor = color * texture2D(sprite, uv);
}
