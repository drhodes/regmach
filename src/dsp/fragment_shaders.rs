pub const FRAGMENT_SHADER_SOURCE: &str = r#"

#version 310 es

out highp vec4 FragColor;
in highp vec3 color;
void main()
{
   // Set the fragment color to the color passed from the vertex shader
   FragColor = vec4(color, 1.0);
}

"#;
