use macroquad::prelude::*;

pub struct Resources {
    pub dino: Texture2D,
    pub walk: Texture2D
}

impl Resources {
    pub fn new(dino: Texture2D, walk: Texture2D) -> Self {
        Resources {
            dino,
            walk
        }
    }
}


pub async fn load_resources() -> Resources {
    let dino: Texture2D = load_texture("res/dino-Sheet.png").await.unwrap();
    dino.set_filter(FilterMode::Nearest);
    let walk: Texture2D = load_texture("res/walk.png").await.unwrap();
    walk.set_filter(FilterMode::Nearest);
    let resources = Resources::new(dino, walk);
    return resources;
}
