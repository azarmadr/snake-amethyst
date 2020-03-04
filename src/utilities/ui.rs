use amethyst::{
    ui::{
        UiTransform, Anchor, UiButtonBuilder, FontAsset, UiText,
    },
    assets::{Handle},
    input::StringBindings,
    ecs::{World, WorldExt, Entity},
    ecs::prelude::{Builder},
    window::ScreenDimensions,
};
/// Generates specified UI. 
///
pub struct UiGenerator {
    font_handle: Handle<FontAsset>,
    builder: UiButtonBuilder<u32, u32>,
    widgets: Vec<Entity>,
}
impl UiGenerator {
    const FONT_SIZE: f32 = 30f32;
    const BUTTON_OFFSET: f32 = 30f32;

    pub fn new(font_handle: Handle<FontAsset>) -> Self {
        let widgets = Vec::new();

        let builder = UiButtonBuilder::new("DEFAULT BUTTON")
            .with_anchor(Anchor::Middle)
            .with_font(font_handle.clone())
            .with_font_size(Self::FONT_SIZE);
        

        Self {
            font_handle,
            builder,
            widgets,
        }
    }

    pub fn generate_buttons(&mut self,world: &mut World, buttons: Vec<&'static str>) {
        let length = buttons.len();

        for (idx, text) in buttons.iter().enumerate() {
            let (x_size, y_size) = Self::acquire_size(text.len());
            let (x, y) = Self::acquire_y_position(world, length as u32, idx as u32, (x_size, y_size));

            let ui_text = UiText::new(
                self.font_handle.clone(),
                String::from(*text),
                [0.0,0.0,0.0,1.0],
                Self::FONT_SIZE,
            );
            let ui_position = UiTransform::new(
                String::from(*text), 
                Anchor::Middle, 
                Anchor::Middle, 
                x, 
                y,
                0f32, 
                x_size, 
                y_size,
            );
              
            world.create_entity()
                .with(ui_text)
                .with(ui_position)
                .build();
        }
    }

    fn acquire_y_position(world: &mut World, num_of_buttons: u32,idx: u32, (size_width, size_height): (f32, f32)) -> (f32, f32) {
        let (mut x, mut y) = (0f32, 0f32);

        let (width, height) = { 
            println!("Trying to fetch `ScreenDimensions`.");
            let dims = world.read_resource::<ScreenDimensions>();
            (dims.width(), dims.height())
        };
        // Get the middle width of the screen.
        let middle_x = (width / 2f32).round();
        // Get the middle height of the screen.
        let middle_y = (height / 2f32).round();

        let x_offset_width = (size_width / 2f32).round();
        let y_offset_heigth = (size_height / 2f32).round();

        x = middle_x - x_offset_width;
        y = middle_y - num_of_buttons as f32 * Self::BUTTON_OFFSET + idx as f32 * Self::BUTTON_OFFSET - y_offset_heigth;

        (x, y)
    }

    fn acquire_size(text_len: usize) -> (f32, f32) {
        let size_width = text_len as f32 * Self::FONT_SIZE;
        let size_height = Self::FONT_SIZE + 10f32;

        (size_width, size_height)
    }
}