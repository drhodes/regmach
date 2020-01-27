// -*- glsl -*-

// #ifdef GL_ES
// precision mediump float;
// #endif

precision mediump float;
varying vec4 vColor;
void main(void) {
  gl_FragColor = vec4(vColor);
}
            
// void main() {
//      gl_FragColor = vec4(0.5, 0.0, 1.0, 1.0);
// }
