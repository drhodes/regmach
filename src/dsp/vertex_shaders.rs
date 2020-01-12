pub const VERTEX_SHADER_SOURCE: &str = r#"
#version 310 es

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor; // Specify a vertex attribute for color

out highp vec3 color;
void main()
{
    gl_Position = vec4(aPos.x/4.0, aPos.y/4.0, aPos.z/4.0, 1.0);
	color = aColor; // pass the color along to the fragment shader
}

"#;

// pub const VERTEX_SHADER_SOURCE: &str = r#"
// #version 310 es

// layout (location = 0) in vec3 aPos;
// layout (location = 1) in vec3 aColor; // Specify a vertex attribute for color
// out highp vec3 color;
// void main()
// {
//     gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
// 	color = aColor; // pass the color along to the fragment shader
// }

// "#;
