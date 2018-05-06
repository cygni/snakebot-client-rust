use std::ops::Add;
use types::{Direction, Map, Position, SnakeInfo};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate(i32, i32);

impl Coordinate {
    pub fn from_position(position: Position, map_width: i32) -> Coordinate {
        let x = position % map_width;
        let y = (position - x) / map_width;
        Coordinate(x, y)
    }

    pub fn to_position(self, map_width: i32) -> Position {
        let Coordinate(x, y) = self;
        x + y * map_width
    }

    #[allow(dead_code)]
    pub fn manhattan_distance_to(self, goal: Coordinate) -> i32 {
        let Coordinate(x0, y0) = self;
        let Coordinate(x1, y1) = goal;
        (x0 - x1).abs() + (y0 - y1).abs()
    }

    #[allow(dead_code)]
    pub fn euclidian_distance_to(self, goal: Coordinate) -> f64 {
        let Coordinate(x0, y0) = self;
        let Coordinate(x1, y1) = goal;
        (((x0 - x1).pow(2) + (y0 - y1).pow(2)) as f64).sqrt()
    }

    #[allow(dead_code)]
    pub fn is_within_square(self, nw_coord: Coordinate, se_coord: Coordinate) -> bool {
        let Coordinate(x, y) = self;
        let Coordinate(nw_x, nw_y) = nw_coord;
        let Coordinate(se_x, se_y) = se_coord;
        x >= nw_x && x <= se_x && y >= nw_y && y <= se_y
    }
}

impl Add for Coordinate {
    type Output = Coordinate;
    fn add(self, rhs: Coordinate) -> Coordinate {
        let Coordinate(x0, y0) = self;
        let Coordinate(x1, y1) = rhs;
        Coordinate(x0 + x1, y0 + y1)
    }
}

#[derive(PartialEq, Debug)]
pub enum Tile<'a> {
    Wall,
    Food {
        coordinate: Coordinate,
    },
    Obstacle {
        coordinate: Coordinate,
    },
    Empty {
        coordinate: Coordinate,
    },
    SnakeHead {
        coordinate: Coordinate,
        snake: &'a SnakeInfo,
    },
    SnakeBody {
        coordinate: Coordinate,
        snake: &'a SnakeInfo,
    },
}

impl Direction {
    pub fn to_movement_delta(&self) -> Coordinate {
        match *self {
            Direction::Down => Coordinate(0, 1),
            Direction::Up => Coordinate(0, -1),
            Direction::Left => Coordinate(-1, 0),
            Direction::Right => Coordinate(1, 0),
        }
    }
}

impl Map {
    pub fn inside_map(&self, coordinate: Coordinate) -> bool {
        let Coordinate(x, y) = coordinate;
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn get_snake_by_id<'a>(&'a self, id: &str) -> Option<&'a SnakeInfo> {
        self.snake_infos.iter().find(|s| &s.id == id)
    }

    pub fn get_tile_at(&self, coordinate: Coordinate) -> Tile {
        let position = coordinate.to_position(self.width);

        if self.obstacle_positions.contains(&position) {
            Tile::Obstacle { coordinate }
        } else if self.food_positions.contains(&position) {
            Tile::Food { coordinate }
        } else if let Some(snake) = self.snake_infos.iter().find(|s| s.positions.contains(&position)) {
            if position == *snake.positions.first().unwrap() {
                Tile::SnakeHead { coordinate, snake }
            } else {
                Tile::SnakeBody { coordinate, snake }
            }
        } else if !self.inside_map(coordinate) {
            Tile::Wall
        } else {
            Tile::Empty { coordinate }
        }
    }

    pub fn is_tile_available_for_movement(&self, coordinate: Coordinate) -> bool {
        match self.get_tile_at(coordinate) {
            Tile::Empty { .. } => true,
            Tile::Food { .. } => true,
            _ => false,
        }
    }

    pub fn can_snake_move_in_direction(&self, snake: &SnakeInfo, direction: Direction) -> bool {
        let Coordinate(dx, dy) = direction.to_movement_delta();
        let Coordinate(x, y) = Coordinate::from_position(*snake.positions.first().unwrap(), self.width);

        self.is_tile_available_for_movement(Coordinate(x + dx, y + dy))
    }

    #[allow(dead_code)]
    pub fn is_coordinate_out_of_bounds(&self, coordinate: Coordinate) -> bool {
        let Coordinate(x, y) = coordinate;
        x < 0 || x >= self.width || y < 0 || y >= self.height
    }
}

#[cfg(test)]
mod test {
    use types::{Map, SnakeInfo};
    use utils::{Coordinate, Direction, Tile};

    const MAP_WIDTH: i32 = 3;

    fn get_snake_one() -> SnakeInfo {
        SnakeInfo {
            name: "1".to_string(),
            points: 0,
            tail_protected_for_game_ticks: 0,
            positions: vec![
                Coordinate(1, 1).to_position(MAP_WIDTH),
                Coordinate(0, 1).to_position(MAP_WIDTH),
            ],
            id: "1".to_string(),
        }
    }

    fn get_snake_two() -> SnakeInfo {
        SnakeInfo {
            name: "2".to_string(),
            points: 0,
            tail_protected_for_game_ticks: 0,
            positions: vec![Coordinate(1, 2).to_position(MAP_WIDTH)],
            id: "2".to_string(),
        }
    }

    // The map used for testing, 1 and 2 represents the snakes
    //yx012
    //0  F
    //1 11#
    //2  2
    fn get_test_map() -> Map {
        Map {
            width: MAP_WIDTH,
            height: MAP_WIDTH,
            world_tick: 0,
            snake_infos: vec![get_snake_one(), get_snake_two()],
            food_positions: vec![Coordinate(1, 0).to_position(MAP_WIDTH)],
            obstacle_positions: vec![Coordinate(2, 1).to_position(MAP_WIDTH)],
        }
    }

    #[test]
    fn snake_can_be_found_by_id() {
        let map = get_test_map();
        let id = &get_snake_one().id;
        let s = map.get_snake_by_id(id);
        let found_id = &s.unwrap().id;
        assert_eq!(id, found_id);
    }

    #[test]
    fn tile_is_correctly_found() {
        let map = get_test_map();
        let snake_one = get_snake_one();
        let snake_two = get_snake_two();
        let tiles = vec![
            vec![
                Tile::Empty {
                    coordinate: Coordinate(0, 0),
                },
                Tile::Food {
                    coordinate: Coordinate(1, 0),
                },
                Tile::Empty {
                    coordinate: Coordinate(2, 0),
                },
            ],
            vec![
                Tile::SnakeBody {
                    coordinate: Coordinate(0, 1),
                    snake: &snake_one,
                },
                Tile::SnakeHead {
                    coordinate: Coordinate(1, 1),
                    snake: &snake_one,
                },
                Tile::Obstacle {
                    coordinate: Coordinate(2, 1),
                },
            ],
            vec![
                Tile::Empty {
                    coordinate: Coordinate(0, 2),
                },
                Tile::SnakeHead {
                    coordinate: Coordinate(1, 2),
                    snake: &snake_two,
                },
                Tile::Empty {
                    coordinate: Coordinate(2, 2),
                },
            ],
        ];
        for y in 0..map.width {
            for x in 0..map.height {
                assert_eq!(tiles[y as usize][x as usize], map.get_tile_at(Coordinate(x, y)));
            }
        }
    }

    #[test]
    fn tile_is_correctly_marked_as_movable() {
        let map = get_test_map();
        let tiles = vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![true, false, true],
        ];

        for y in 0..map.height {
            for x in 0..map.width {
                assert_eq!(
                    tiles[y as usize][x as usize],
                    map.is_tile_available_for_movement(Coordinate(x, y))
                );
            }
        }
    }

    #[test]
    fn can_snake_move_identifies_correctly() {
        let map = get_test_map();
        let id = &get_snake_one().id;
        let snake = map.get_snake_by_id(id).unwrap();

        assert_eq!(true, map.can_snake_move_in_direction(&snake, Direction::Up));
        assert_eq!(false, map.can_snake_move_in_direction(&snake, Direction::Down));
        assert_eq!(false, map.can_snake_move_in_direction(&snake, Direction::Left));
        assert_eq!(false, map.can_snake_move_in_direction(&snake, Direction::Right));
    }

    #[test]
    fn can_not_move_to_walls() {
        let map = get_test_map();
        let id = &get_snake_two().id;
        let snake = map.get_snake_by_id(id).unwrap();

        assert_eq!(false, map.can_snake_move_in_direction(&snake, Direction::Down));
    }
}
