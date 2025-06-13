use getch_rs::{Getch, Key};
use rand::{rngs::ThreadRng, seq::IndexedRandom};

// [2]상수를 정의하는 곳

const MAZE_WIDTH: usize = 8;
const MAZE_HEIGHT: usize = 8;

const GOAL_X: usize = MAZE_WIDTH - 1;
const GOAL_Y: usize = MAZE_HEIGHT - 1;

// [3]열거 상수를 정의하는 곳

// [3-1]방위의 종류를 정의한다
#[derive(Clone, Copy)]
enum DirectionEnum {
    North = 0,
    West,
    South,
    East,
    Max,
}

impl TryFrom<usize> for DirectionEnum {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == DirectionEnum::North as usize => Ok(DirectionEnum::North),
            x if x == DirectionEnum::West as usize => Ok(DirectionEnum::West),
            x if x == DirectionEnum::South as usize => Ok(DirectionEnum::South),
            x if x == DirectionEnum::East as usize => Ok(DirectionEnum::East),
            _ => Err(()),
        }
    }
}

// [3-2]플레이어로부터의 상대 위치 종류를 정의한다
enum LocationEnum {
    FrontLeft = 0,
    FrontRight,
    Front,
    Left,
    Right,
    Center,
    Max,
}

#[derive(Clone, Copy, PartialEq)]
enum AAEnum {
    All = 0,
    FrontLeftNorth,
    FrontRightNorth,
    FrontNorth,
    FrontWest,
    FrontEast,
    LeftNorth,
    RightNorth,
    North,
    West,
    East,
    None,
}

impl TryFrom<usize> for AAEnum {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == AAEnum::All as usize => Ok(AAEnum::All),
            x if x == AAEnum::FrontLeftNorth as usize => Ok(AAEnum::FrontLeftNorth),
            x if x == AAEnum::FrontRightNorth as usize => Ok(AAEnum::FrontRightNorth),
            x if x == AAEnum::FrontNorth as usize => Ok(AAEnum::FrontNorth),
            x if x == AAEnum::FrontWest as usize => Ok(AAEnum::FrontWest),
            x if x == AAEnum::FrontEast as usize => Ok(AAEnum::FrontEast),
            x if x == AAEnum::LeftNorth as usize => Ok(AAEnum::LeftNorth),
            x if x == AAEnum::RightNorth as usize => Ok(AAEnum::RightNorth),
            x if x == AAEnum::North as usize => Ok(AAEnum::North),
            x if x == AAEnum::West as usize => Ok(AAEnum::West),
            x if x == AAEnum::East as usize => Ok(AAEnum::East),
            x if x == AAEnum::None as usize => Ok(AAEnum::None),
            _ => Err(()),
        }
    }
}

// [4]구조체를 선언하는 곳

// [4-1]벡터의 구조체를 선언한다
#[derive(Clone, Copy, Default)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    // [6-1]벡터를 더하는 함수를 선언한다
    pub fn add(&mut self, other: &Vec2) {
        self.x += other.x;
        self.y += other.y;
    }

    // [6-1]벡터를 더하는 함수를 선언한다
    pub fn add_new(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }

    // [6-2]대상 좌표가 미로의 범위 내인지 여부를 판정하는 함수를 선언한다
    pub fn is_inside_maze(&self) -> bool {
        self.x >= 0 && self.x < MAZE_WIDTH as isize && self.y >= 0 && self.y < MAZE_HEIGHT as isize
    }
}

// [4-2]미로 칸의 구조체를 선언한다
#[derive(Clone, Copy)]
struct Tile {
    walls: [bool; DirectionEnum::Max as usize],
}

impl Tile {
    pub fn new() -> Self {
        Self {
            walls: [false; DirectionEnum::Max as usize],
        }
    }
}

// [4-3]플레이어의 구조체를 선언한다
struct Character {
    position: Vec2,
    direction: DirectionEnum,
}

impl Character {
    pub fn new() -> Self {
        Self {
            position: Vec2::default(),
            direction: DirectionEnum::North,
        }
    }
    pub fn turn_left(&mut self) {
        self.direction = ((self.direction as usize + 1) % DirectionEnum::Max as usize)
            .try_into()
            .unwrap();
    }
    pub fn turn_back(&mut self) {
        self.direction = ((self.direction as usize + 2) % DirectionEnum::Max as usize)
            .try_into()
            .unwrap();
    }
    pub fn turn_right(&mut self) {
        self.direction = ((self.direction as usize + DirectionEnum::Max as usize - 1)
            % DirectionEnum::Max as usize)
            .try_into()
            .unwrap();
    }
}

struct Resource {
    all: [String; 8],
    front_left_north: [String; 8],
    front_right_north: [String; 8],
    front_north: [String; 8],
    front_west: [String; 8],
    front_east: [String; 8],
    left_north: [String; 8],
    right_north: [String; 8],
    north: [String; 8],
    west: [String; 8],
    east: [String; 8],

    aa_table: [[AAEnum; DirectionEnum::Max as usize]; LocationEnum::Max as usize],
    locations: [[Vec2; LocationEnum::Max as usize]; DirectionEnum::Max as usize],
}

impl Resource {
    pub fn new() -> Self {
        Self {
            // [5-2]기준이 되는 아스키아트를 선언한다
            all: [
                "L       /\n".to_string(),
                "#L     /#\n".to_string(),
                "#|L   /|#\n".to_string(),
                "#|#|#|#|#\n".to_string(),
                "#|#| |#|#\n".to_string(),
                "#|/   L|#\n".to_string(),
                "#/     L#\n".to_string(),
                "/       L\n".to_string(),
            ],
            // [5-3]왼쪽 전방 벽의 아스키아트를 선언한다
            front_left_north: [
                "         \n".to_string(),
                "         \n".to_string(),
                "  _      \n".to_string(),
                " |#|     \n".to_string(),
                " |_|     \n".to_string(),
                "         \n".to_string(),
                "         \n".to_string(),
                "         \n".to_string(),
            ],
            // [5-4]오른쪽 전방 벽의 아스키아트를 선언한다
            front_right_north: [
                "         \n".to_string(),
                "         \n".to_string(),
                "      _  \n".to_string(),
                "     |#| \n".to_string(),
                "     |_| \n".to_string(),
                "         \n".to_string(),
                "         \n".to_string(),
                "         \n".to_string(),
            ],
            // [5-5]전방 앞쪽 벽의 아스키아트를 선언한다
            front_north: [
                "         \n".to_string(),
                "         \n".to_string(),
                "    _    \n".to_string(),
                "   |#|   \n".to_string(),
                "   |_|   \n".to_string(),
                "         \n".to_string(),
                "         \n".to_string(),
                "         \n".to_string(),
            ],
            // [5-6]전방 왼쪽 벽의 아스키아트를 선언한다
            front_west: [
                "         \n".to_string(),
                "         \n".to_string(),
                ".|L      \n".to_string(),
                ".|#|     \n".to_string(),
                ".|#|     \n".to_string(),
                ".|/      \n".to_string(),
                "         \n".to_string(),
                "         \n".to_string(),
            ],
            // [5-7]전방 오른쪽 벽의 아스키아트를 선언한다
            front_east: [
                "         \n".to_string(),
                "         \n".to_string(),
                "      /| \n".to_string(),
                "     |#| \n".to_string(),
                "     |#| \n".to_string(),
                "      L| \n".to_string(),
                "         \n".to_string(),
                "         \n".to_string(),
            ],
            // [5-8]왼쪽 전방 벽의 아스키아트를 선언한다
            left_north: [
                "         \n".to_string(),
                "_        \n".to_string(),
                "#|       \n".to_string(),
                "#|       \n".to_string(),
                "#|       \n".to_string(),
                "_|       \n".to_string(),
                "         \n".to_string(),
                "         \n".to_string(),
            ],
            // [5-9]오른쪽 전방 벽의 아스키아트를 선언한다
            right_north: [
                "         \n".to_string(),
                "        _\n".to_string(),
                "       |#\n".to_string(),
                "       |#\n".to_string(),
                "       |#\n".to_string(),
                "       |_\n".to_string(),
                "         \n".to_string(),
                "         \n".to_string(),
            ],
            // [5-10]앞쪽 벽의 아스키아트를 선언한다
            north: [
                "         \n".to_string(),
                "  _____  \n".to_string(),
                " |#####| \n".to_string(),
                " |#####| \n".to_string(),
                " |#####| \n".to_string(),
                " |_____| \n".to_string(),
                "         \n".to_string(),
                "         \n".to_string(),
            ],
            // [5-11]왼쪽 벽의 아스키아트를 선언한다
            west: [
                "L        \n".to_string(),
                "#L       \n".to_string(),
                "#|       \n".to_string(),
                "#|       \n".to_string(),
                "#|       \n".to_string(),
                "#|       \n".to_string(),
                "#/       \n".to_string(),
                "/        \n".to_string(),
            ],
            // [5-12]오른쪽 벽의 아스키아트를 선언한다
            east: [
                "        /\n".to_string(),
                "       /#\n".to_string(),
                "       |#\n".to_string(),
                "       |#\n".to_string(),
                "       |#\n".to_string(),
                "       |#\n".to_string(),
                "       L#\n".to_string(),
                "        L\n".to_string(),
            ],

            // [5-13]아스키아트의 테이블을 선언한다
            aa_table: [
                // LOCATION_FRONT_LEFT  왼쪽 앞
                [
                    AAEnum::FrontLeftNorth, // DIRECTION_NORTH  북쪽
                    AAEnum::None,           // DIRECTION_WEST   서쪽
                    AAEnum::None,           // DIRECTION_SOUTH  남쪽
                    AAEnum::None,           // DIRECTION_EAST   동쪽
                ],
                // LOCATION_FRONT_RIGHT 오른쪽 앞
                [
                    AAEnum::FrontRightNorth, // DIRECTION_NORTH  북쪽
                    AAEnum::None,            // DIRECTION_WEST   서쪽
                    AAEnum::None,            // DIRECTION_SOUTH  남쪽
                    AAEnum::None,            // DIRECTION_EAST   동쪽
                ],
                // LOCATION_FRONT       앞쪽
                [
                    AAEnum::FrontNorth, // DIRECTION_NORTH  북쪽
                    AAEnum::FrontWest,  // DIRECTION_WEST   서쪽
                    AAEnum::None,       // DIRECTION_SOUTH  남쪽
                    AAEnum::FrontEast,  // DIRECTION_EAST   동쪽
                ],
                // LOCATION_LEFT        왼쪽
                [
                    AAEnum::LeftNorth, // DIRECTION_NORTH  북쪽
                    AAEnum::None,      // DIRECTION_WEST   서쪽
                    AAEnum::None,      // DIRECTION_SOUTH  남쪽
                    AAEnum::None,      // DIRECTION_EAST   동쪽
                ],
                // LOCATION_RIGHT       오른쪽
                [
                    AAEnum::RightNorth, // DIRECTION_NORTH  북쪽
                    AAEnum::None,       // DIRECTION_WEST   서쪽
                    AAEnum::None,       // DIRECTION_SOUTH  남쪽
                    AAEnum::None,       // DIRECTION_EAST   동쪽
                ],
                // LOCATION_CENTER      중심
                [
                    AAEnum::North, // DIRECTION_NORTH  북쪽
                    AAEnum::West,  // DIRECTION_WEST   서쪽
                    AAEnum::None,  // DIRECTION_SOUTH  남쪽
                    AAEnum::East,  // DIRECTION_EAST   동쪽
                ],
            ],
            locations: [
                // direction_north
                [
                    Vec2::new(-1, -1),
                    Vec2::new(1, -1),
                    Vec2::new(0, -1),
                    Vec2::new(-1, 0),
                    Vec2::new(1, 0),
                    Vec2::new(0, 0),
                ],
                // direction_west
                [
                    Vec2::new(-1, 1),
                    Vec2::new(-1, -1),
                    Vec2::new(-1, 0),
                    Vec2::new(0, 1),
                    Vec2::new(0, -1),
                    Vec2::new(0, 0),
                ],
                // direction_south
                [
                    Vec2::new(1, 1),
                    Vec2::new(-1, 1),
                    Vec2::new(0, 1),
                    Vec2::new(1, 0),
                    Vec2::new(-1, 0),
                    Vec2::new(0, 0),
                ],
                // direction_east
                [
                    Vec2::new(1, -1),
                    Vec2::new(1, 1),
                    Vec2::new(1, 0),
                    Vec2::new(0, -1),
                    Vec2::new(0, 1),
                    Vec2::new(0, 0),
                ],
            ],
        }
    }
}
struct Context {
    maze: [Tile; MAZE_HEIGHT * MAZE_WIDTH],
    directions: [Vec2; DirectionEnum::Max as usize],
    player: Character,
    resource: Resource,
    g: Getch,
    rng: ThreadRng,
}

impl Context {
    pub fn new() -> Self {
        // [5-1]각 방위의 벡터를 선언한다
        let directions = [
            Vec2::new(0, -1),
            Vec2::new(-1, 0),
            Vec2::new(0, 1),
            Vec2::new(1, 0),
        ];
        Self {
            maze: [Tile::new(); MAZE_HEIGHT * MAZE_WIDTH],
            player: Character::new(),
            directions,
            resource: Resource::new(),
            g: Getch::new(),
            rng: rand::rng(),
        }
    }

    // [6-3]벽을 파는 함수를 선언한다
    fn dig_wall(&mut self, position: &Vec2, direction: DirectionEnum) {
        if !position.is_inside_maze() {
            return;
        }

        // [6-3-3]대상 벽을 판다
        self.maze[position.y as usize * MAZE_WIDTH + position.x as usize].walls
            [direction as usize] = false;

        // [6-3-4]옆 칸의 좌표를 선언한다
        let next_position = position.add_new(&self.directions[direction as usize]);

        if next_position.is_inside_maze() {
            // [6-3-6]옆방의 파는 벽의 방위를 선언한다
            let next_direction = (direction as usize + 2) % DirectionEnum::Max as usize;

            // [6-3-7]옆 방의 벽을 판다
            self.maze[next_position.y as usize * MAZE_WIDTH + next_position.x as usize].walls
                [next_direction as usize] = false;
        }
    }

    // [6-4]대상 벽을 파도 좋은지 여부를 판정하는 함수를 선언한다
    fn can_dig_wall(&self, position: &Vec2, direction: DirectionEnum) -> bool {
        // [6-4-1]옆의 좌표를 선언한다
        let next_position = position.add_new(&self.directions[direction as usize]);

        // [6-4-2]옆의 좌표가 미로 범위 내인지 여부를 판정한다
        if !next_position.is_inside_maze() {
            return false;
        }
        for i in 0..DirectionEnum::Max as usize {
            // [6-4-5]벽을 팔 수 있는지 여부를 판정한다
            if !self.maze[next_position.y as usize * MAZE_WIDTH + next_position.x as usize].walls[i]
            {
                return false; // [6-4-6]파서는 안된다는 결과를 반환한다
            }
        }
        return true; // [6-4-7]파도 좋다는 결과를 반환한다
    }

    // [6-5]미로를 랜덤으로 생성하는 함수를 선언한다
    fn generate_map(&mut self) {
        for y in 0..MAZE_HEIGHT {
            for x in 0..MAZE_WIDTH {
                for i in 0..DirectionEnum::Max as usize {
                    // [6-5-4]대상 방위를 벽으로 한다
                    self.maze[y * MAZE_WIDTH + x].walls[i] = true;
                }
            }
        }

        // [6-5-5]현재 좌표를 선언한다
        let mut current_position = Vec2::new(0, 0);

        // [6-5-6]벽을 파야 하는 칸의 리스트를 선언한다
        let mut to_dig_wall_positions: Vec<Vec2> = Vec::new();

        // [6-5-7]벽을 파야 하는 칸 리스트에 현재 칸을 더한다
        to_dig_wall_positions.push(current_position);

        loop {
            // [6-5-9]팔 수 있는 벽의 방위 리스트를 선언한다
            let mut can_dig_directions = Vec::new();

            for i in 0..DirectionEnum::Max as usize {
                if self.can_dig_wall(&current_position, i.try_into().unwrap()) {
                    // [6-5-12]팔 수 있는 벽의 방위 리스트에 추가한다
                    can_dig_directions.push(i);
                }
            }

            if can_dig_directions.len() > 0 {
                // [6-5-14]파는 벽의 방위를 선언한다
                let dig_direction = *can_dig_directions.choose(&mut self.rng).unwrap();

                // [6-5-15]대상 벽을 판다
                self.dig_wall(&current_position, dig_direction.try_into().unwrap());

                // [6-5-16]판 벽의 건너편으로 이동한다
                current_position.add(&self.directions[dig_direction]);

                // [6-5-17]벽을 파야 하는 칸의 좌표 리스트에 현재 좌표를 더한다
                to_dig_wall_positions.push(current_position);
            } else {
                // [6-5-18]팔 데가 없을 때

                // [6-5-19]벽을 파야 하는 칸 리스트에서 현재 칸을 삭제한다
                to_dig_wall_positions.remove(0);

                // [6-5-20]벽을 파야 하는 칸 리스트가 비어 있는지 여부를 판정한다
                if to_dig_wall_positions.len() <= 0 {
                    break;
                }

                // [6-5-22]벽을 파야 하는 칸 리스트에서 맨앞의 칸을 구하여 이동한다
                current_position = to_dig_wall_positions[0];
            }
        }
    }

    // [6-6]맵을 그리는 함수를 선언한다
    pub fn draw_map(&self) {
        for y in 0..MAZE_HEIGHT {
            for x in 0..MAZE_WIDTH {
                //  [6-6-3]북쪽 벽을 그린다
                print!(
                    "+{}+",
                    if self.maze[y * MAZE_WIDTH + x].walls[DirectionEnum::North as usize] {
                        '-'
                    } else {
                        ' '
                    }
                );
            }
            println!();
            for x in 0..MAZE_WIDTH {
                let mut floor_aa = ' ';

                // [6-6-7]플레이어의 좌표를 그리는 중이면
                if x == self.player.position.x as usize && y == self.player.position.y as usize {
                    const DIRECTION_AA: [char; DirectionEnum::Max as usize] = ['↑', '←', '↓', '→'];

                    // [6-6-9]바닥의 아스키아트에 플레이어의 아스키아트를 복사한다
                    floor_aa = DIRECTION_AA[self.player.direction as usize];
                } else if x == GOAL_X && y == GOAL_Y {
                    // [6-6-10]목표 지점의 좌표를 그리는 중이면

                    // [6-6-11]바닥 아스키아트에 목표 지점 아스키아트를 복사한다
                    floor_aa = 'G';
                }
                // [6-6-12]서쪽 벽, 중심 바닥, 동쪽 벽을 그린다
                print!(
                    "{}{}{}",
                    if self.maze[y * MAZE_WIDTH + x].walls[DirectionEnum::West as usize] {
                        '-'
                    } else {
                        ' '
                    },
                    floor_aa,
                    if self.maze[y * MAZE_WIDTH + x].walls[DirectionEnum::East as usize] {
                        '-'
                    } else {
                        ' '
                    }
                );
            }
            println!();
            for x in 0..MAZE_WIDTH {
                // [6-6-15]남쪽 벽을 그린다
                print!(
                    "+{}+",
                    if self.maze[y * MAZE_WIDTH + x].walls[DirectionEnum::South as usize] {
                        '-'
                    } else {
                        ' '
                    }
                );
            }
            println!();
        }
    }

    // [6-7]미로를 의사 3D 시점으로 그리는 함수를 선언한다
    pub fn draw_3d(&self) {
        const SCREEN_WIDTH: usize = 9;
        const SCREEN_HEIGHT: usize = 8;

        // [6-7-1]화면 버퍼를 선언한다
        let mut screen = [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ];

        // [6-7-2]모든 상대 위치를 반복한다
        for i in 0..LocationEnum::Max as usize {
            // [6-7-3]절대 위치를 선언한다
            let position = self
                .player
                .position
                .add_new(&self.resource.locations[self.player.direction as usize][i]);

            // [6-7-4]절대 위치가 미로의 범위 밖인지 여부를 판정한다
            if !position.is_inside_maze() {
                continue;
            }

            // [6-7-6]모든 방위를 반복한다
            for j in 0..DirectionEnum::Max as usize {
                // [6-7-7]상대 방위를 선언한다
                let direction = (DirectionEnum::Max as usize + j - self.player.direction as usize)
                    % DirectionEnum::Max as usize;

                // [6-7-8]대상 벽이 없는지 여부를 판정한다
                if !self.maze[position.y as usize * MAZE_WIDTH + position.x as usize].walls[j] {
                    continue;
                }

                // [6-7-10]합성하는 아스키아트가 없는지 여부를 판정한다
                if self.resource.aa_table[i][direction] == AAEnum::None {
                    continue;
                }

                let aa_index = self.resource.aa_table[i][direction];
                let aa = match aa_index {
                    AAEnum::All => &self.resource.all,
                    AAEnum::FrontLeftNorth => &self.resource.front_left_north,
                    AAEnum::FrontRightNorth => &self.resource.front_right_north,
                    AAEnum::FrontNorth => &self.resource.front_north,
                    AAEnum::FrontWest => &self.resource.front_west,
                    AAEnum::FrontEast => &self.resource.front_east,
                    AAEnum::LeftNorth => &self.resource.left_north,
                    AAEnum::RightNorth => &self.resource.right_north,
                    AAEnum::North => &self.resource.north,
                    AAEnum::West => &self.resource.west,
                    AAEnum::East => &self.resource.east,
                    AAEnum::None => unreachable!(),
                };

                for k in 0..SCREEN_HEIGHT {
                    for l in 0..SCREEN_WIDTH {
                        let ch = aa[k].chars().nth(l).unwrap();
                        // [6-7-13]대상 문자가 스페이스가 아닌지 여부를 판정한다
                        if ch != ' ' {
                            // [6-7-14]화면 버퍼에 합성하는 아스키아트를 적어넣는다
                            screen[k][l] = aa[k].chars().nth(l).unwrap();
                        }
                    }
                }
            }
        }

        for k in 0..SCREEN_HEIGHT {
            for l in 0..SCREEN_WIDTH {
                // [6-7-16]화면 버퍼의 반각 문자를 전각 문자로 변환하여 그린다
                match screen[k][l] {
                    ' ' => print!(" "),
                    '#' => print!(" "),
                    '_' => print!("_"),
                    '|' => print!("|"),
                    '/' => print!("/"),
                    'L' => print!("\\"),
                    // [6-7-23]상기 이외의 문자는 그대로 그린다
                    _ => print!("{}", screen[k][l]),
                }
            }
            println!();
        }
    }

    // [6-8]게임을 초기화하는 함수를 선언한다
    pub fn init(&mut self) {
        // [6-8-1]미로를 랜덤으로 생성하는 함수를 호출한다
        self.generate_map();

        // [6-8-2]플레이어의 좌표를 초기화한다
        self.player.position = Vec2::new(0, 0);

        // [6-8-3]플레이어의 방위를 초기화한다
        self.player.direction = DirectionEnum::North;
    }
}

fn main() {
    let mut ctx = Context::new();

    ctx.init();

    loop {
        clearscreen::clear().unwrap();

        // [6-9-5]미로를 의사 3D 시점으로 그리는 함수를 호출한다
        ctx.draw_3d();

        // [6-9-6]맵을 그리는 함수를 호출한다
        ctx.draw_map();

        // [6-9-7]입력된 키로 분기한다
        match ctx.g.getch() {
            Ok(Key::Char('w')) => {
                // [6-9-9]플레이어의 눈앞이 벽인지 아닌지를 판정한다
                if !ctx.maze
                    [ctx.player.position.y as usize * MAZE_WIDTH + ctx.player.position.x as usize]
                    .walls[ctx.player.direction as usize]
                {
                    // [6-9-10]전진 목적지의 좌표를 선언한다
                    let next_position = ctx
                        .player
                        .position
                        .add_new(&ctx.directions[ctx.player.direction as usize]);

                    // [6-9-11]전진 목적지의 좌표가 미로 내인지 여부를 판정한다
                    if next_position.is_inside_maze() {
                        // [6-9-12]전진 목적지의 좌표를 적용한다
                        ctx.player.position = next_position;

                        // [6-9-13]목표 지점에 도달했는지 여부를 판정한다
                        if ctx.player.position.x as usize == GOAL_X
                            && ctx.player.position.y as usize == GOAL_Y
                        {
                            clearscreen::clear().unwrap();
                            println!(" * * CONGRATULATIONS * * ");
                            println!();
                            println!(" 드디어 전설의 부적을 손에 넣었다!");
                            println!();
                            println!("  하지만 고난을 함께 한 무엇과도 바꿀 수 없는");
                            println!(" [동료]라는 보물을 손에 넣은 지금,");
                            println!(" 부적의 광채는 더 이상 눈에 들어오지 않는다   ");
                            println!();
                            println!("     ~  THE END ~");

                            let _ = ctx.g.getch();

                            ctx.init();
                        }
                    }
                }
            }
            Ok(Key::Char('s')) => {
                ctx.player.turn_back();
            }
            Ok(Key::Char('a')) => {
                ctx.player.turn_left();
            }
            Ok(Key::Char('d')) => {
                ctx.player.turn_right();
            }
            Ok(Key::Esc) => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
}
