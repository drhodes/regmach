// -*- glsl -*-


attribute vec4 position;
attribute vec4 color;
varying vec4 vColor;
uniform mat4 mvp;

void main() {
  gl_Position = mvp * position;
  vColor = color;
}

