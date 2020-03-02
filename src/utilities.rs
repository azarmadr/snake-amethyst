use amethyst::renderer::{TextureMetadata,ScreenDimensions,Projection,Camera,PngFormat,Texture,MaterialTextureSet,Sprite,SpriteSheetHandle,TextureCoordinates,SpriteSheet};
use amethyst::assets::{AssetStorage,Loader};
use amethyst::prelude::*;
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform};
use amethyst::ui::{UiTransform,Anchor,UiImage,UiButton,UiText,get_default_font,FontAsset,LineMode};

fn load_texture_from_image(world: &mut World,image_path: &str,texture_id: u64) {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let texture_handle = loader.load(
            image_path,
            PngFormat,
            TextureMetadata::srgb(),
            (),
            &texture_storage);
    let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
    material_texture_set.insert(texture_id, texture_handle); 
}
pub fn decompile_as_sprites(world: &mut World,image_path: &str,image_size: (f32,f32), sprite_size: (f32,f32),texture_id: u64) -> SpriteSheetHandle {    
    let sprites_in_x = (image_size.0 / sprite_size.0).trunc();
    let sprites_in_y = (image_size.1 / sprite_size.1).trunc();

    let sprite_offset_x_in_image = 1.0 / sprites_in_x;
    let sprite_offset_y_in_image = 1.0 / sprites_in_y;

    let mut sprites = vec![];

    for y in 0..sprites_in_y as u32{
        for x in 0..sprites_in_x as u32{
            let left = x as f32 * sprite_offset_x_in_image;
            let right = (x + 1) as f32 * sprite_offset_x_in_image;

            let top = (y + 1) as f32 * sprite_offset_y_in_image;
            let bottom = y as f32 * sprite_offset_y_in_image;

            let tex_coords = TextureCoordinates{
                left,
                right,
                bottom,
                top,
            };

            let sprite = Sprite{
                width: sprite_size.0,
                height: sprite_size.1,
                //offsets: [sprite_size.0 / 2.0, sprite_size.1 / 2.0],
                offsets: [0.0, 0.0],
                tex_coords,
            };

            sprites.push(sprite);
        }
    }

    let sprite_sheet = SpriteSheet{
        texture_id: texture_id,
        sprites: sprites,
    };

    load_texture_from_image(world, image_path, texture_id);

    let sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

        loader.load_from_data(sprite_sheet, (), &sprite_sheet_storage)
    };
    sprite_sheet_handle
}

pub fn initialise_camera(world: &mut World) {
    let (width,height) = {
        let dimn = world.read_resource::<ScreenDimensions>();
        (dimn.width(), dimn.height())
    };

    
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            width,
            height,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()
        ))
        .build();
}
pub fn initialize_ui(world: &mut World) {
    let (width,height) = {
        let dimn = world.read_resource::<ScreenDimensions>();
        (dimn.width(), dimn.height())
    };
    let (texture_handle) = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        let tex_handle = loader.load(
            "ui/play_button.png",
            PngFormat,
            TextureMetadata::srgb(),
            (),
            &texture_storage);
        let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
        //let font_handle = get_default_font(&loader, &font_storage);
        (tex_handle)
    };
    let ui_transform = UiTransform::new("play_button".to_owned(), Anchor::Middle, 0.5, 0.5, 0.0, 32.0, 32.0, 0);
    let ui_image = UiImage { texture : texture_handle };
    //let ui_text = UiText::new(default_font,"".to_owned(),[0.0;4],0.0);
    let ui_button = UiButton::new([0.0;4],None,None);
    world.create_entity()
        .with(ui_button)
      //  .with(ui_text)
        .with(ui_image)
        .with(ui_transform)
        .build();

}

pub struct Backpack {
    pub snake_sheet: Option<SpriteSheetHandle>,
    pub food_sheet: Option<SpriteSheetHandle>,
}
impl Backpack {
    pub fn new(sheet: SpriteSheetHandle,food_sheet: SpriteSheetHandle) -> Self {
        Backpack {
            snake_sheet: Some(sheet),
            food_sheet: Some(food_sheet),
        }
    }
}
impl Default for Backpack {
    fn default() -> Self {
        Backpack {
            snake_sheet: None,
            food_sheet: None,
        }
    }
}