use macroquad::prelude::*;
use macroquad_tiled as tiled;
use macroquad_platformer::*;

struct Player {
    collider: Actor,
    speed: Vec2,
}

#[macroquad::main("Dave")]
async fn main() {
    let tileset = load_texture("examples/mytileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let player = load_texture("examples/dave_walk.png").await.unwrap();
    player.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("examples/level1.json").await.unwrap();
    let tiled_map = tiled::load_map(&tiled_map_json, 
        &[("mytileset.png", tileset),
                    ("dave_walk.png", player)], &[])
                    .unwrap();
        

    let mut static_colliders = vec![];
    for (_x, _y, tile) in tiled_map.tiles("Tile Layer 1", None) {
        static_colliders.push(if tile.is_some() {
            Tile::Solid
        } else {
            Tile::Empty
        });
    }

 
    let mut world = World::new();
    world.add_static_tiled_layer(static_colliders,32., 32.,19, 1);

    let mut player = Player {
        collider: world.add_actor(vec2(60.0, 250.0), 32, 32),
        speed: vec2(0., 0.),
    };

    let camera = Camera2D::from_display_rect(Rect::new(0.0, 320.0, 608.0, -320.0));

    //let mut is_jumping = false;

    loop {
        clear_background(BLACK);
        
        set_camera(&camera);
    
        tiled_map.draw_tiles("Tile Layer 1", Rect::new(0.0, 0.0, 608.0, 320.0), None);

        //draw player
        {
            // sprite id from tiled
            const PLAYER_SPRITE: u32 = 1;

            let pos = world.actor_pos(player.collider);
            if player.speed.x >= 0.0 {
                tiled_map.spr("dave_walk", PLAYER_SPRITE, Rect::new(pos.x, pos.y, 32.0, 32.0));
            } else {
                tiled_map.spr(
                    "dave_walk",
                    PLAYER_SPRITE,
                    Rect::new(pos.x + 32.0, pos.y, -32.0, 32.0),
                );
            }
        }

        // player movement control
        {
            let pos = world.actor_pos(player.collider);
            
            let on_ground = world.collide_check(player.collider, pos + vec2(0., 1.));
            
          
            if !on_ground {
                player.speed.y += 500. * get_frame_time();

            } 
            
            if is_key_down(KeyCode::Right) {
                player.speed.x = 100.0;
            } else if is_key_down(KeyCode::Left) {
                player.speed.x = -100.0;
            } else {
                player.speed.x = 0.;
            }

            if is_key_pressed(KeyCode::Space) && on_ground {
                player.speed.y = -260.;
            }

            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x,mouse_y) = mouse_position();
                println!("(x, y) = ({}, {})",mouse_x, mouse_y);
                let camera_coords = camera.screen_to_world(vec2(mouse_x, mouse_y));
                println!("Camera coordinates: x: {}, y: {}", camera_coords.x, camera_coords.y);
            }

            world.move_h(player.collider, player.speed.x * get_frame_time());
            world.move_v(player.collider, player.speed.y * get_frame_time());
        }
        next_frame().await
    }
}
