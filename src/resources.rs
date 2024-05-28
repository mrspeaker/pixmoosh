use macroquad::prelude::*;

pub struct Resources {
    pub dino: Texture2D
}

impl Resources {
    pub fn new(dino: Texture2D) -> Self {
        Resources {
            dino
        }
    }
}


pub async fn load_resources() -> Resources {
    let dino: Texture2D = load_texture("res/dino-Sheet.png").await.unwrap();
    dino.set_filter(FilterMode::Nearest);
    let resources = Resources::new(dino);
    return resources;
}
