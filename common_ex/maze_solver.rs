use std::collections::VecDeque;

pub fn solve_maze(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    if maze.is_empty() || maze[0].is_empty() {
        return vec![];
    }

    let rows = maze.len();
    let cols = maze[0].len();

    if start.0 >= rows || start.1 >= cols || end.0 >= rows || end.1 >= cols {
        return vec![];
    }

    if start == end {
        return vec![start];
    }

    // parent[r][c] = the cell we came from to reach (r, c)
    let mut parent: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; cols]; rows];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(start);
    visited[start.0][start.1] = true;

    // up, down, left, right
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut found = false;

    while let Some((row, col)) = queue.pop_front() {
        if (row, col) == end {
            found = true;
            break;
        }

        for (dr, dc) in directions {
            // wrapping_add_signed: going off the top/left wraps to a huge
            // usize, which fails the bounds check below.
            let new_row = row.wrapping_add_signed(dr);
            let new_col = col.wrapping_add_signed(dc);

            if new_row >= rows || new_col >= cols {
                continue;
            }
            if visited[new_row][new_col] {
                continue;
            }
            if maze[new_row][new_col] == '#' {
                continue;
            }

            visited[new_row][new_col] = true;
            parent[new_row][new_col] = Some((row, col));
            queue.push_back((new_row, new_col));
        }
    }

    if !found {
        return vec![];
    }

    // Backtrack from end to start, then reverse.
    let mut path = Vec::new();
    let mut current = end;
    loop {
        path.push(current);
        if current == start {
            break;
        }
        match parent[current.0][current.1] {
            Some(prev) => current = prev,
            None => return vec![],
        }
    }
    path.reverse();
    path
}

pub fn main() {
    let maze = vec![
        vec!['S', '.', '#', '#', '#'],
        vec!['#', '.', '#', '.', '.'],
        vec!['#', '.', '.', '.', '#'],
        vec!['#', '#', '#', '.', '#'],
        vec!['#', '#', '#', 'E', '#'],
    ];
    let start = (0, 0);
    let end = (4, 3);

    let path = solve_maze(maze, start, end);
    assert_eq!(
        path,
        vec![
            (0, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (2, 2),
            (2, 3),
            (3, 3),
            (4, 3),
        ]
    );

    // Dead-end maze: no path
    let blocked = vec![
        vec!['S', '#'],
        vec!['#', 'E'],
    ];
    assert_eq!(solve_maze(blocked, (0, 0), (1, 1)), vec![]);

    println!("All tests passed");
}
