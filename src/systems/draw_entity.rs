use crate::prelude::*;
use crate::camera::Camera;

#[system]
#[read_component(Point)]
#[read_component(Renderable)]
#[read_component(Viewshed)]
pub fn draw_entity
(
    ecs: &SubWorld,
    #[resource] camera: &Camera
)
{
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    let mut viewshed_query = <&Viewshed>::query().filter(component::<Player>());
    let player_viewshed = viewshed_query.iter(ecs).nth(0).unwrap();

    <(&Point, &Renderable)>::query()
    .iter(ecs)
    .for_each
    (|(pos, entity)|
        {
            if player_viewshed.visible_tiles.contains(&pos)
            {
                draw_batch.set
                (
                    *pos - offset,
                    ColorPair::new(entity.fg, entity.bg),
                    entity.glyph
                );
            }
        }
    );
    draw_batch.submit(5000).expect("Batch error");
}
