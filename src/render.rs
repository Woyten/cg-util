use glium::Blend;
use glium::BlendingFunction;
use glium::Depth;
use glium::DepthTest;
use glium::DrawParameters;
use glium::LinearBlendingFactor;

pub fn default_draw_parameters<'a>() -> DrawParameters<'a> {
    DrawParameters {
        depth: Depth {
            test: DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn transparency_draw_parameters<'a>(opacity: f32) -> DrawParameters<'a> {
    DrawParameters {
        depth: Depth {
            test: DepthTest::IfLess,
            write: false,
            ..Default::default()
        },
        blend: Blend {
            color: BlendingFunction::Addition {
                source: LinearBlendingFactor::ConstantColor,
                destination: LinearBlendingFactor::OneMinusConstantColor,
            },
            constant_value: (opacity, opacity, opacity, opacity),
            ..Default::default()
        },
        ..Default::default()
    }
}
