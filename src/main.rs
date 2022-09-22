#[derive(Default)]
struct Graph {
    graph: [[u8; 7]; 7],
    current_state:[usize; 3],
    final_state:[usize; 3],
    visited:[[[bool; 7]; 7]; 7],
    solution:Vec<[usize;3]>,
}


impl Graph {
    fn init(&mut self) {
        self.graph = [
            [0,0,1,0,0,0,0],//0:Bone
            [0,0,0,0,0,0,0],//1:House
            [0,0,0,0,0,0,0],//2:Boat
            [0,0,0,0,0,1,0],//3:Tree
            [0,0,1,0,0,1,0],//4:Flower
            [0,0,0,1,1,0,0],//5:Well
            [0,0,0,1,0,0,0],//6:Carrot
        ];
        self.current_state = [1, 2, 3];
        self.final_state =  [6, 6, 0];
        self.visited = [[[false as bool; 7]; 7]; 7];
        self.solution = Vec::new();
    }

    fn bunny1(&self) -> usize{
        self.current_state[0]
    }
    fn bunny2(&self) -> usize{
        self.current_state[1]
    }
    fn dog(&self) -> usize{
        self.current_state[2]
    }

    fn is_move_allowed(&self, from:usize, to:usize) -> bool {
        if self.graph[from][to] == 1 {
            return true;
        }
        //somebody at carrot to go between house and bone
        else if (from == 0 && to == 1) || (from == 1 && to == 0) {
            return self.bunny1() == 6 || self.bunny2() == 6 || self.dog() == 6
        }
        //somebody at tree to go between house and boat
        else if (from == 1 && to == 2) || (from == 2 && to == 1) {
            return self.bunny1() == 3 || self.bunny2() == 3 || self.dog() == 3
        }
        //somebody at bone and somebody at flower to go between house and tree
        else if (from == 1 && to == 3) || (from == 3 && to == 1) {
            return (self.bunny1() == 0 || self.bunny2() == 0 || self.dog() == 0) &&
                    (self.bunny1() == 4 || self.bunny2() == 4 || self.dog() == 4)
        }
        //nobody at bone to go between well and carrot
        else if (from == 5 && to == 6) || (from == 6 && to == 5) {
            return self.bunny1() != 0 && self.bunny2() != 0 && self.dog() != 0
        }
        else {
            return false;
        }
    }

    
    fn allowed_next_states(&self) -> Vec<[usize; 3]> {
        let mut allowed_states:Vec<[usize;3]> = Vec::new();
        for who in 0..3 {
            allowed_states.append(&mut self.allowed_moves(who));
        }
        allowed_states
    }

    fn allowed_moves(&self, who:usize) -> Vec<[usize; 3]> {
        let mut moves:Vec<[usize;3]> = Vec::new();
        for to in 0..7 {
            if self.is_move_allowed(self.current_state[who], to) {
                let mut state:[usize; 3] = self.current_state.clone();
                state[who] = to;
                moves.push(state);
            }
        }
        moves
    }

    fn update_position(&mut self, who:usize, to:usize) {
        if self.is_move_allowed(self.current_state[who], to) {
            self.current_state[who] = to;
        } else {
            panic!("Illegal move {} to {}", self.current_state[who], to);
        }
    }

    fn update_state(&mut self, to:[usize;3]) {
        if to[0] != self.current_state[0] {
            self.update_position(0, to[0]);
            return;
        } else if to[1] != self.current_state[1] {
            self.update_position(1, to[1]);
            return;
        } else if to [2] != self.current_state[2] {
            self.update_position(2, to[2]);
            return;
        }
    }

    fn is_solved(&self) -> bool {
        self.current_state[0] == self.final_state[0] &&
        self.current_state[1] == self.final_state[1] &&
        self.current_state[2] == self.final_state[2]
    }

    fn recursive_solve(&mut self, s:[usize;3]) {
        self.solution.push(s.clone());
        self.current_state = s;
        self.visited[self.current_state[0]][self.current_state[1]][self.current_state[2]] = true;
        //Since both bunnies are equivalent
        self.visited[self.current_state[1]][self.current_state[0]][self.current_state[2]] = true;
        
        if self.is_solved() {
            println!("SOLUTION FOUND!");
            println!("{:?}", self.solution);
            return;
        }
    
        self.allowed_next_states().into_iter().rev().for_each(|s| {
            if !self.visited[s[0]][s[1]][s[2]] {
                self.recursive_solve(s);
            }
        });
        self.solution.pop();
    }
}

fn main() {
    let mut g:Graph = Default::default();
    g.init();
    let initial_state:[usize;3] = g.current_state.clone();
    g.recursive_solve(initial_state);

}
