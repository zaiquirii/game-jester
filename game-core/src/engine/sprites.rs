use ggez::{
    glam::{UVec2, vec2},
    graphics,
};

pub struct Sprites {
    sprite_sheets: Vec<SpriteSheet>,
}

impl Sprites {
    pub fn new() -> Self {
        Self {
            sprite_sheets: Vec::new(),
        }
    }

    pub fn find_by_name(&self, name: &str) -> Option<Handle> {
        self.sprite_sheets
            .iter()
            .enumerate()
            .find(|(_, sprite)| sprite.name == name)
            .map(|s| s.0)
    }

    pub fn load_sprite_sheet(
        &mut self,
        ctx: &mut ggez::Context,
        name: &str,
        path: &str,
    ) -> Result<Handle, anyhow::Error> {
        let file = ctx.fs.open(format!("{path}.json"))?;
        let data: aseprite::Data = serde_json::from_reader(file)?;
        let image = graphics::Image::from_path(ctx, format!("{path}.png"))?;
        let sprite = data.as_sprite_sheet(name, image);
        let handle = self.sprite_sheets.len();
        self.sprite_sheets.push(sprite);
        Ok(handle)
    }

    pub fn get(&self, handle: Handle) -> &SpriteSheet {
        &self.sprite_sheets[handle]
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas, sprite: &AnimatedSprite) {
        let sprite_sheet = self.get(sprite.sprite_handle);
        let image = &sprite_sheet.image;
        let current_frame = sprite_sheet.frames[sprite.current_frame as usize];
        let current_rect = current_frame.rect;

        let normalized_rect = graphics::Rect::new(
            current_rect.x / image.width() as f32,
            current_rect.y / image.height() as f32,
            current_rect.w / image.width() as f32,
            current_rect.h / image.height() as f32,
        );
        canvas.draw(
            image,
            graphics::DrawParam::new()
                .dest(vec2(200., 100.))
                .scale(vec2(2.0, 2.0))
                .src(normalized_rect),
        );
    }

    pub fn advance_frame(&self, sprite: &mut AnimatedSprite, delta_ms: f32) {
        let sprite_sheet = self.get(sprite.sprite_handle);
        let current_frame = sprite_sheet.frames[sprite.current_frame as usize];

        sprite.accumulated_time_ms += delta_ms;
        if sprite.accumulated_time_ms >= current_frame.duration {
            sprite.accumulated_time_ms -= current_frame.duration;

            let current_tag = &sprite_sheet.tags[sprite.current_tag as usize];

            sprite.current_frame += 1;
            if sprite.current_frame > current_tag.end_frame {
                sprite.current_frame = current_tag.start_frame;
            }
        }
    }
}

pub fn advance_sprite_animations_system(
    world: &mut sparsey::World,
    sprites: &Sprites,
    delta_ms: f32,
) {
    world.for_each::<&mut AnimatedSprite>(|sprite| {
        sprites.advance_frame(sprite, delta_ms);
    })
}

pub fn render_animated_sprite_system(
    canvas: &mut graphics::Canvas,
    world: &mut sparsey::World,
    sprites: &Sprites,
) {
    world.for_each::<&AnimatedSprite>(|sprite| {
        sprites.draw(canvas, sprite);
    })
}

pub struct SpriteSheet {
    name: String,
    image: graphics::Image,
    frames: Vec<SpriteFrame>,
    tags: Vec<SpriteTag>,
}

type Handle = usize;

#[derive(Clone, Copy)]
struct SpriteFrame {
    pub rect: graphics::Rect,
    duration: f32,
}

struct SpriteTag {
    name: String,
    start_frame: u32,
    end_frame: u32,
}

pub struct AnimatedSprite {
    pub paused: bool,
    pub sprite_handle: Handle,
    pub current_frame: u32,
    pub current_tag: u32,
    pub accumulated_time_ms: f32,
}

mod aseprite {
    use super::*;

    #[derive(serde::Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Metadata {
        image: String,
        frame_tags: Vec<FrameTag>,
    }

    #[derive(serde::Deserialize, Debug)]
    struct FrameData {
        duration: u32,
        frame: Frame,
    }

    #[derive(serde::Deserialize, Debug)]
    struct Frame {
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    }

    #[derive(serde::Deserialize, Debug)]
    struct FrameTag {
        name: String,
        from: u32,
        to: u32,
    }

    #[derive(serde::Deserialize, Debug)]
    pub(crate) struct Data {
        frames: Vec<FrameData>,
        meta: Metadata,
    }

    impl Data {
        pub fn as_sprite_sheet(&self, name: &str, image: graphics::Image) -> SpriteSheet {
            let frames = self
                .frames
                .iter()
                .map(|f| SpriteFrame {
                    rect: graphics::Rect::new(
                        f.frame.x as f32,
                        f.frame.y as f32,
                        f.frame.w as f32,
                        f.frame.h as f32,
                    ),
                    duration: f.duration as f32,
                })
                .collect::<Vec<_>>();
            let mut tags = self
                .meta
                .frame_tags
                .iter()
                .map(|f| SpriteTag {
                    name: f.name.clone(),
                    start_frame: f.from,
                    end_frame: f.to,
                })
                .collect::<Vec<_>>();

            // if there are no tags let's assume that the entire spritesheet is a single animation
            if tags.is_empty() {
                tags.push(SpriteTag {
                    name: "default".into(),
                    start_frame: 0,
                    end_frame: (frames.len() - 1) as u32,
                });
            }

            SpriteSheet {
                name: name.into(),
                image,
                frames,
                tags,
            }
        }
    }
}
