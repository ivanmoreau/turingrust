mod vector_utils;
use vector_utils::utils::*;
enum Direction {
    L,
    R,
    S
}
type State     = i32;
type Symbol    = char;
type Smt       = (State, Symbol, Direction);
type Tape      = Vec<Symbol>;
type TapeParts = (Tape, Symbol, Tape);
struct Machine {
    init_state  : State,
    function    : fn((State, Symbol)) -> Smt,
    accept_state: State
}
struct MacState {
    state: State,
    tape : TapeParts
}

impl std::fmt::Debug for MacState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let s: Vec<char> = format!("{}", self.state).chars().collect();
    let v = add(self.tape.0.clone(),
        add(vec![' ','q','_'],
            add(s,
                add(vec![' ', self.tape.1],
                    self.tape.2.clone()))));
    let r: String = v.into_iter().collect();
    write!(f, "{}\n", r)
    }
}

fn move_tape((x,y,z): TapeParts, direction: Direction) -> TapeParts {
    use Direction::*;
    match (&x[..], y, &z[..], direction) {
        ([], _, _ , L) => (vec![], '_', add(vec![y], z)),
        (_ , _, [], R) => (add(x, vec![y]), '_', vec![]),
        (_ , _, _ , L) => (init(x.clone()), last(x), add(vec![y], z)),
        (_ , _, _ , R) => (add(x, vec![y]), head(z.clone()), tail(z)),
        (_ , _, _ , S) => (x, y, z)
    }
}

fn step(mac: Machine, (a, b, c): TapeParts, 
        stt: Vec<MacState>) -> Vec<MacState> {
    if mac.init_state == mac.accept_state {
        add(stt, vec![MacState {state : mac.init_state, 
            tape : (a, b, c)}])
    } else {
        let (x, y, z) = (mac.function)((mac.init_state, b));
        step(Machine { init_state: x,
                           function  : mac.function, 
                           accept_state: mac.accept_state },
                 move_tape((a.clone(), y, c.clone()), z), 
        add(stt, vec![MacState {state : mac.init_state, 
            tape : (a, b, c)}]))
    }
}

fn run(mac: Machine, tape: Tape) -> Vec<MacState> {
    step(mac, (vec![], head(tape.clone()), tail(tape)), vec![])
}

fn test_machine() -> Machine {
    Machine { init_state: 0, function: test_machinefn, accept_state: 30 }
}

fn test_machinefn((st, sy): (i32, char)) -> Smt {
    use Direction::*;
    match (st, sy) {
        (0 , '1') => (1,  '_', R),
        (0 , '0') => (20, '_', R),
        (1 , '1') => (1,  '1', R),
        (1 , '0') => (2,  '0', R),
        (2 , '1') => (2,  'b', R),
        (2 , '_') => (3,  '_', L),
        (3 , 'b') => (3,  'b', L),
        (3 , '1') => (3,  '1', L),
        (3 , '0') => (4,  '0', R),
        (3 , 'a') => (4,  'a', R),
        (4 , 'b') => (5,  'a', R),
        (4 , '1') => (6,  'b', R),
        (5 , 'b') => (5,  'b', R),
        (5 , '_') => (3,  '1', L),
        (6 , '1') => (6,  'b', R),
        (6 , '_') => (7,  '_', L),
        (7 , 'a') => (7,  'a', L),
        (7 , 'b') => (7,  'b', L),
        (7 , '0') => (8,  '0', L),
        (8 , '1') => (8,  '1', L),
        (8 , '_') => (0,  '_', R),
        (20, 'a') => (20, '1', R),
        (20, 'b') => (20, '_', R),
        (20, '_') => (30, '_', S),
        (5 , '1') => (5,  '1', R),
        (2 , 'a') => (2,  'a', R),
        (2 , 'b') => (2,  'b', R),
        (_ , _  ) => panic!("Invalid")
    }
}

// Side Effects
fn main() {
    let val = 
    run(
        test_machine(),
        vec!['1','1','1','1','1','1','1','1',
        '0','1','1','1','1','1','1','1','1']);
    println!("{:?}", val);
}
