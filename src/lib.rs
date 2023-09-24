mod snake;
mod world;
// use std::{sync::Mutex, time::Duration};

// use once_cell::sync::Lazy;
// use world::World;

// use anyhow::*;
// use js_sys::Array;

// use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, KeyboardEvent, Event};
// use yew::{function_component, html, Html};

// const WORLD_WIDTH: i32 = 16;
// const CELL_SIZE: i32 = 20;
// const SNAKE_LENGHT: i32 = 2;
// static GLOBAL_WORLD: Lazy<Mutex<World>> =
//     Lazy::new(|| Mutex::new(World::new(WORLD_WIDTH, SNAKE_LENGHT)));

// static mut WORLD: Lazy<World> = Lazy::new(|| World::new(8, 3));

// #[wasm_bindgen]
// pub fn load_game(){
//     let w2 = window().context("no se pudo obtener la ventana").unwrap();
//     w2.alert_with_message("WASM Loaded!!").unwrap();
//     draw();
//     w2.add_event_listener_with_callback(
//         "keydown",
//         Closure::wrap(Box::new(|e: KeyboardEvent| {
//             let key = e.key();
//             let mut world_config = GLOBAL_WORLD.lock().unwrap();
//             let old_dir = world_config.snake.direction.clone();
//             let new_dir = match key.as_str() {
//                 "ArrowUp" => Direction::Up,
//                 "ArrowDown" => Direction::Down,
//                 "ArrowLeft" => Direction::Left,
//                 "ArrowRight" => Direction::Right,
//                 _ => Direction::None,
//             };
//             if new_dir != Direction::None {
//                 world_config.snake.direction = new_dir;
//                 let pos = move_snake(world_config.to_owned());
//                 if pos == world_config.snake.body[1].0 {
//                     world_config.snake.direction = old_dir;
//                 }
//             }
//         }) as Box<dyn FnMut(_)>)
//         .into_js_value()
//         .as_ref()
//         .unchecked_ref(),
//     )
//     .unwrap();
//     //let t = yew::platform::time::interval(Duration::from_secs_f32(1.0));
//     let closure = Closure::wrap(Box::new(draw) as Box<dyn FnMut()>);
//     w2.set_interval_with_callback_and_timeout_and_arguments(
//         closure.into_js_value().as_ref().unchecked_ref(),
//         300,
//         &Array::new(),
//     )
//     .unwrap();
// }

// #[function_component(App)]
// fn my_app() -> Html {
//      let w = window().context("no se pudo obtener la ventana").unwrap();
//     let closure = Closure::wrap(Box::new(move |_e:Event| {load_game();}) as Box<dyn FnMut(_)>)
//      .into_js_value();
//     //let body = w.document().unwrap().body().unwrap();
//     //body.set_onload(Option::Some(c.as_ref().unchecked_ref()));
//     w.set_onload(Option::Some(closure.as_ref().unchecked_ref()));
//     html! {
//         <div>
//         <Header />
//         <CanvasPane />
//         </div>
//     }
// }

// fn draw() {
    // let mut world_config = GLOBAL_WORLD.lock().unwrap();
    // let head = world_config.snake.get_snake_head().unwrap();
    // if world_config.snake.contains_in_body(head) {
    //     let w2 = window().context("no se pudo obtener la ventana").unwrap();
    //     w2.clear_interval();
    //     world_config.reset_world(WORLD_WIDTH, SNAKE_LENGHT);
    //     w2.alert_with_message("Has Perdido!!").unwrap();
    //     return;
    // }

    // let w2 = window().context("no se pudo obtener la ventana").unwrap();
    // let doc = w2.document().context("error al obtener documento").unwrap();
    // let canvas: HtmlCanvasElement = doc.get_element_by_id("canvas").unwrap().dyn_into().unwrap();
    // let ctx: CanvasRenderingContext2d = canvas
    //     .get_context("2d")
    //     .unwrap()
    //     .unwrap()
    //     .dyn_into()
    //     .unwrap();
    // let canvas_size: u32 = (world_config.world_size * CELL_SIZE) as u32;
    // canvas.set_height(canvas_size);
    // canvas.set_width(canvas_size);
    // if world_config.snake.get_snake_head().unwrap() == world_config.price_cell {
    //     let snake_length = world_config.snake.get_snake_length();
    //     world_config.append_snake_cell();
    //     world_config.reset_price();
    //     world_config.score += snake_length;
    // }
    // let scoreboard = doc
    //     .get_element_by_id("scoreboard")
    //     .unwrap()
    //     .dyn_into::<HtmlElement>()
    //     .unwrap();
    // scoreboard.set_text_content(Some(format!("{}", world_config.score).as_str()));
    // ctx.begin_path();

    // draw_grid(&ctx, world_config.to_owned());
    // draw_snake(&ctx, world_config.to_owned());
    // draw_price(&ctx, world_config.to_owned());
    // ctx.stroke();
    // let pos = move_snake(world_config.to_owned());
    // if pos != world_config.snake.body[1].0 {
    //     world_config.set_snake_position(pos);
    // }
    // drop(canvas);
    // drop(ctx);
    // drop(w2);
// }

// #[wasm_bindgen]
// pub fn move_snake(world_size: i32, head: i32, direction: Direction) -> i32 {
//     let size = world_size * world_size;
//     // let head = world_config.snake.get_snake_head().unwrap();

//     let col = head % world_size;
//     let row = head / world_size;

//     match direction {
//         snake::Direction::Right => ((row * world_size) + (col + 1)) % size,
//         snake::Direction::Left => {
//             if col == 0 {
//                 if row == 0 {
//                     world_size * world_size
//                 } else {
//                     (((row - 1) * world_size) + (world_size - 1)) % size
//                 }
//             } else {
//                 ((row * world_size) + (col - 1)) % size
//             }
//         }
//         snake::Direction::Up => {
//             if row == 0 {
//                 if col == world_size - 1 {
//                     ((world_size - 1) * world_size) % size
//                 } else {
//                     (((world_size - 1) * world_size) + (col + 1)) % size
//                 }
//             } else {
//                 (((row - 1) * world_size) + col) % size
//             }
//         }
//         snake::Direction::Down => {
//             if row == world_size - 1 {
//                 if col == 0 {
//                     ((world_size * world_size) + (world_size - 1)) % size
//                 } else {
//                     (((world_size) * world_size) + (col - 1)) % size
//                 }
//             } else {
//                 (((row + 1) * world_size) + col) % size
//             }
//         }
//         Direction::None => head,
//     }
// }
// fn draw_price(ctx: &CanvasRenderingContext2d, config: World) {
//     ctx.set_fill_style(&JsValue::from_str("#FF0000"));
//     let col = config.price_cell % config.world_size;
//     let row = config.price_cell / config.world_size;
//     ctx.fill_rect(
//         (CELL_SIZE * col) as f64,
//         (CELL_SIZE * row) as f64,
//         CELL_SIZE as f64,
//         CELL_SIZE as f64,
//     );
// }

// fn draw_snake(ctx: &CanvasRenderingContext2d, config: World) {
//     ctx.set_fill_style(&JsValue::from_str("#8869AD"));
//     for cells in config.snake.body {
//         let col = cells.0 % config.world_size;
//         let row = cells.0 / config.world_size;
//         ctx.fill_rect(
//             (CELL_SIZE * col) as f64,
//             (CELL_SIZE * row) as f64,
//             CELL_SIZE as f64,
//             CELL_SIZE as f64,
//         );
//     }
// }

// fn draw_grid(ctx: &CanvasRenderingContext2d, config: World) {
//     for x in 0..config.world_size + 1 {
//         let start_position = (CELL_SIZE * x) as f64;
//         let y = (config.world_size * CELL_SIZE) as f64;
//         ctx.move_to(start_position, 0.0);
//         ctx.line_to(start_position, y);

//         ctx.move_to(0.0, start_position);
//         ctx.line_to(y, start_position);
//     }
// }
// #[function_component(Header)]
// fn my_header() -> Html {
//     let title = "Snake - WASM Rust";
//     html! {
//         <div>
//         <h1> {title} </h1>
//             <p>
//                 <span>{"Puntaje: "}</span><span id="scoreboard"></span>
//             </p>
//         </div>
//     }
// }

// #[function_component(CanvasPane)]
// fn my_user_pane() -> Html {

//     html! {
//         <div>
//         <canvas id={"canvas"} />
//         <br/>
//         <div>
//         <table>
//         <tr>
//         <td><span/></td>
//         <td>
//             <input type={"button"} onclick={|_e:yew::MouseEvent|{
//                 let mut world_config = GLOBAL_WORLD.lock().unwrap();
//                 let old_dir = world_config.snake.direction.clone();
//                 let new_dir =  Direction::Up;
//                 world_config.snake.direction = new_dir;
//                 let pos = move_snake(world_config.to_owned());
//                 if pos == world_config.snake.body[1].0 {
//                     world_config.snake.direction = old_dir;
//                 }
//             } } value={"Arriba"} />
//         </td>
//         <td><span/></td>
//         </tr>
//         <tr>
//         <td>
//         <input type={"button"} onclick={|_e:yew::MouseEvent|{
//             let mut world_config = GLOBAL_WORLD.lock().unwrap();
//                 let old_dir = world_config.snake.direction.clone();
//                 let new_dir =  Direction::Left;
//                 world_config.snake.direction = new_dir;
//                 let pos = move_snake(world_config.to_owned());
//                 if pos == world_config.snake.body[1].0 {
//                     world_config.snake.direction = old_dir;
//                 }
//         } } value={"Izquierda"} />
//         </td>
//         <td><span/>
//         </td>
//         <td>
//         <input type={"button"} onclick={|_e:yew::MouseEvent|{
//             let mut world_config = GLOBAL_WORLD.lock().unwrap();
//             let old_dir = world_config.snake.direction.clone();
//             let new_dir =  Direction::Right;
//             world_config.snake.direction = new_dir;
//             let pos = move_snake(world_config.to_owned());
//             if pos == world_config.snake.body[1].0 {
//                 world_config.snake.direction = old_dir;
//             }
//         } } value={"Derecha"} />
// </td>
//         </tr>
//         <tr>
//         <td><span/></td>
//         <td>
//             <input type={"button"} onclick={|_e:yew::MouseEvent|{
//                 let mut world_config = GLOBAL_WORLD.lock().unwrap();
//                 let old_dir = world_config.snake.direction.clone();
//                 let new_dir =  Direction::Down;
//                 world_config.snake.direction = new_dir;
//                 let pos = move_snake(world_config.to_owned());
//                 if pos == world_config.snake.body[1].0 {
//                     world_config.snake.direction = old_dir;
//                 }
//             } } value={"Abajo"} />
//         </td>
//         <td><span/></td>
//         </tr>
//         </table>
//         </div>
//         </div>
//     }
// }
