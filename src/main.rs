use tcod::colors::*;
use tcod::console::*;
use tcod::system::set_fps;

//const
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
//structs
struct Tcod {
    root: Root,
    con: Offscreen,
}

struct Object {
    x: i32,
    y: i32,
    character: char,
    color: Color,
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map,
}

//impl

impl Object {
    pub fn new(x: i32, y: i32, character: char, color: Color) -> Self {
        Object {
            x,
            y,
            character,
            color,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.character, BackgroundFlag::None);
    }
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

//functions
fn handle_key(tcod: &mut Tcod, player: &mut Object, game: &Game) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,

        _ => {}
    }
    false
}

fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    map
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object]) {
    for object in objects {
        object.draw(&mut tcod.con);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    blit(
        &tcod.con,
        (0, 0),
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}
//main
fn main() {
    //initialize tcode
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Asci rustlike")
        .init();
    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    set_fps(LIMIT_FPS);

    let mut tcod = Tcod { root, con };
    //setup
    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    let player = Object::new(player_x, player_y, '@', WHITE);

    let npc = Object::new(player_x - 5, player_y - 5, '@', YELLOW);
    let mut objects = [player, npc];

    let mut game = Game { map: make_map() };

    game.map[30][22] = Tile::wall();
    game.map[50][22] = Tile::wall();

    //game loop
    while !tcod.root.window_closed() {
        tcod.con.clear();
        render_all(&mut tcod, &game, &objects);
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
        let player = &mut objects[0];
        let exit = handle_key(&mut tcod, player, &game);
        if exit {
            break;
        }
    }
}
