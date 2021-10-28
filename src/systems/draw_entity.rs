use crate::prelude::*;
use crate::camera::Camera;

#[system]
#[read_component(Point)]
#[read_component(Renderable)]
pub fn draw_entity
(
    ecs: &SubWorld,
    #[resource] camera: &Camera
)
{
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Renderable)>::query()
    .iter(ecs)
    .for_each
    (|(pos, entity)|
        {
            draw_batch.set
            (
                *pos - offset,
                ColorPair::new(entity.fg, entity.bg),
                entity.glyph
            );
        }
    );
    draw_batch.submit(5000).expect("Batch error");
}
