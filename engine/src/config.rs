use amethyst_context::ContextConfig;
use amethyst_processors::RendererConfig;

use amethyst_config::Element;
use std::path::Path;

config!(
    struct Config {
        pub context_config: ContextConfig = ContextConfig::default(),
        pub renderer_config: RendererConfig = RendererConfig::default(),
    }
);
