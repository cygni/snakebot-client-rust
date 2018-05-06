use types::{Direction, Map, SnakeInfo};

#[derive(PartialEq, Debug)]
pub enum Tile<'a> {
    Wall,
    Food {
        coordinate: (i32, i32),
    },
    Obstacle {
        coordinate: (i32, i32),
    },
    Empty {
        coordinate: (i32, i32),
    },
    SnakeHead {
        coordinate: (i32, i32),
        snake: &'a SnakeInfo,
    },
    SnakeBody {
        coordinate: (i32, i32),
        snake: &'a SnakeInfo,
    },
}

impl Direction {
    pub fn as_movement_delta(&self) -> (i32, i32) {
        match *self {
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl Map {
    pub fn inside_map(&self, coordinate: (i32, i32)) -> bool {
        let (x, y) = coordinate;
        let inside_x = x >= 0 && x < self.width;
        let inside_y = y >= 0 && y < self.height;
        inside_x && inside_y
    }

    pub fn get_snake_by_id<'a>(&'a self, id: &str) -> Option<&'a SnakeInfo> {
        self.snake_infos.iter().find(|s| &s.id == id)
    }

    pub fn get_tile_at(&self, coordinate: (i32, i32)) -> Tile {
        let position = translate_coordinate(coordinate, self.width);
        let snake_at_tile = self.snake_infos.iter().find(|s| s.positions.contains(&position));

        if self.obstacle_positions.contains(&position) {
            Tile::Obstacle { coordinate: coordinate }
        } else if self.food_positions.contains(&position) {
            Tile::Food { coordinate: coordinate }
        } else if snake_at_tile.is_some() {
            let s = snake_at_tile.unwrap();
            if s.positions[0] == position {
                Tile::SnakeHead {
                    coordinate: coordinate,
                    snake: s,
                }
            } else {
                Tile::SnakeBody {
                    coordinate: coordinate,
                    snake: s,
                }
            }
        } else if !self.inside_map(coordinate) {
            Tile::Wall
        } else {
            Tile::Empty { coordinate: coordinate }
        }
    }

    pub fn is_tile_available_for_movement(&self, coordinate: (i32, i32)) -> bool {
        let tile = self.get_tile_at(coordinate);
        match tile {
            Tile::Empty { coordinate: _ } => true,
            Tile::Food { coordinate: _ } => true,
            _ => false,
        }
    }

    pub fn can_snake_move_in_direction(&self, snake: &SnakeInfo, direction: Direction) -> bool {
        let (xd, yd) = direction.as_movement_delta();
        let (x, y) = translate_position(snake.positions[0], self.width);

        self.is_tile_available_for_movement((x + xd, y + yd))
    }

    #[allow(dead_code)]
    pub fn is_coordinate_out_of_bounds(&self, coordinate: (i32, i32)) -> bool {
        let (x, y) = coordinate;
        x < 0 || x >= self.width || y < 0 || y >= self.height
    }
}

#[allow(dead_code)]
pub fn translate_position(position: i32, map_width: i32) -> (i32, i32) {
    let pos = position as f64;
    let width = map_width as f64;

    let y = (pos / width).floor();
    let x = (pos - y * width).abs();

    (x as i32, y as i32)
}

#[allow(dead_code)]
pub fn translate_positions(positions: &Vec<i32>, map_width: i32) -> Vec<(i32, i32)> {
    positions
        .into_iter()
        .map(|pos| translate_position(*pos, map_width))
        .collect()
}

#[allow(dead_code)]
pub fn translate_coordinate(coordinates: (i32, i32), map_width: i32) -> i32 {
    let (x, y) = coordinates;
    x + y * map_width
}

#[allow(dead_code)]
pub fn get_manhattan_distance(start: (i32, i32), goal: (i32, i32)) -> i32 {
    let (x1, y1) = start;
    let (x2, y2) = goal;

    let x = (x1 - x2).abs();
    let y = (y1 - y2).abs();

    x + y
}

#[allow(dead_code)]
pub fn get_euclidian_distance(start: (i32, i32), goal: (i32, i32)) -> f64 {
    let (x1, y1) = start;
    let (x2, y2) = goal;

    let x = (x1 - x2).pow(2);
    let y = (y1 - y2).pow(2);
    let d = (x + y) as f64;

    d.sqrt().floor()
}

#[allow(dead_code)]
pub fn is_within_square(coord: (i32, i32), nw_coord: (i32, i32), se_coord: (i32, i32)) -> bool {
    let (x, y) = coord;
    let (nw_x, nw_y) = nw_coord;
    let (se_x, se_y) = se_coord;

    x >= nw_x && x <= se_x && y >= nw_y && y <= se_y
}

#[cfg(test)]
mod test {
    use types::{Map, SnakeInfo};
    use utils::{translate_coordinate, Direction, Tile};

    const MAP_WIDTH: i32 = 3;

    fn get_snake_one() -> SnakeInfo {
        SnakeInfo {
            name: String::from("1"),
            points: 0,
            tail_protected_for_game_ticks: 0,
            positions: vec![
                translate_coordinate((1, 1), MAP_WIDTH),
                translate_coordinate((0, 1), MAP_WIDTH),
            ],
            id: String::from("1"),
        }
    }

    fn get_snake_two() -> SnakeInfo {
        SnakeInfo {
            name: String::from("2"),
            points: 0,
            tail_protected_for_game_ticks: 0,
            positions: vec![translate_coordinate((1, 2), MAP_WIDTH)],
            id: String::from("2"),
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
            food_positions: vec![translate_coordinate((1, 0), MAP_WIDTH)],
            obstacle_positions: vec![translate_coordinate((2, 1), MAP_WIDTH)],
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
                Tile::Empty { coordinate: (0, 0) },
                Tile::Food { coordinate: (1, 0) },
                Tile::Empty { coordinate: (2, 0) },
            ],
            vec![
                Tile::SnakeBody {
                    coordinate: (0, 1),
                    snake: &snake_one,
                },
                Tile::SnakeHead {
                    coordinate: (1, 1),
                    snake: &snake_one,
                },
                Tile::Obstacle { coordinate: (2, 1) },
            ],
            vec![
                Tile::Empty { coordinate: (0, 2) },
                Tile::SnakeHead {
                    coordinate: (1, 2),
                    snake: &snake_two,
                },
                Tile::Empty { coordinate: (2, 2) },
            ],
        ];
        for y in 0..map.width {
            for x in 0..map.height {
                assert_eq!(tiles[y as usize][x as usize], map.get_tile_at((x, y)));
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
                    map.is_tile_available_for_movement((x, y))
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
