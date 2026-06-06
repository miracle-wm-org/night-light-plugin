use miracle_plugin::plugin::{set_screen_shader, Plugin};

/// Full-screen GLSL post-process that shades the screen toward orange overnight.
///
/// `tex` and `timeOfDay` are pre-declared by the compositor wrapper, so they are
/// used here without redeclaration. `timeOfDay` is seconds since local midnight
/// and is updated every frame, so the orange intensity is recomputed live.
const NIGHT_LIGHT_SHADER: &str = r#"
vec4 sample_to_rgba(in vec2 texcoord) {
    vec4 color = texture2D(tex, texcoord);

    // Hours since local midnight, [0, 24)
    float h = timeOfDay / 3600.0;

    // Orange shading intensity in [0, 1]:
    //   08:00-18:00 : 0       (daytime, no shading)
    //   18:00-21:00 : 0 -> 1  (evening ramp up)
    //   21:00-06:00 : 1       (strongest orange overnight)
    //   06:00-08:00 : 1 -> 0  (morning ramp down)
    float intensity;
    if (h >= 8.0 && h < 18.0) {
        intensity = 0.0;
    } else if (h >= 18.0 && h < 21.0) {
        intensity = (h - 18.0) / 3.0;
    } else if (h >= 6.0 && h < 8.0) {
        intensity = (8.0 - h) / 2.0;
    } else {
        intensity = 1.0; // 21:00-24:00 and 00:00-06:00
    }

    float r = color.r;
    float g = color.g * mix(1.0, 0.90, intensity); // slightly reduce green
    float b = color.b * mix(1.0, 0.55, intensity); // heavily reduce blue
    return vec4(r, g, b, color.a);
}
"#;

#[derive(Default)]
struct FocusBlurPlugin;

impl Plugin for FocusBlurPlugin {
    fn configure(&mut self) -> Option<miracle_plugin::config::Configuration> {
        let _ = set_screen_shader(&[NIGHT_LIGHT_SHADER]);
        None
    }
}

miracle_plugin::miracle_plugin!(FocusBlurPlugin);
