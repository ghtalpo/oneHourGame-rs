use std::process::exit;
use std::time::SystemTime;

use getch_rs::Getch;

// [2]상수를 정의하는 곳
const FIELD_WIDTH: usize = 160;
const FIELD_HEIGHT: usize = 160;
const FPS: usize = 10;
const INTERVAL: f32 = 1000.0 / FPS as f32; // 밀리 초 

struct Context {
    field: Vec<bool>, //bool[FieldHeight][FieldWidth];
    g: Getch,
    last_clock: SystemTime,
}

impl Context {
    pub fn new() -> Self {
        // [3-1]필드를 선언한다
        // let mut field = vec![false; FIELD_HEIGHT * FIELD_WIDTH];
        // field[0 * FIELD_WIDTH + 1] = true;
        // field[1 * FIELD_WIDTH + 2] = true;
        // field[2 * FIELD_WIDTH + 0] = true;
        // field[2 * FIELD_WIDTH + 1] = true;
        // field[2 * FIELD_WIDTH + 2] = true;
        Self {
            field: vec![false; FIELD_HEIGHT * FIELD_WIDTH],
            g: Getch::new(),
            last_clock: SystemTime::now(),
        }
    }
    // [4-1]필드를 그리는 함수를 선언한다
    pub fn draw_field(&self) {
        clearscreen::clear().unwrap();

        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                // [4-1-4]셀이 살아 있으면「■」를, 죽어 있으면「  」를 그립니다
                print!(
                    "{}",
                    if self.field[y * FIELD_WIDTH + x] {
                        "■"
                    } else {
                        "□"
                    }
                );
            }
            println!();
        }
    }
    // [4-2]대상 셀과 인접하는 살아 있는 셀의 수를 가져오는 함수를 선언한다
    pub fn get_living_cells_count(&self, x_: i64, y_: i64) -> u64 {
        let mut count = 0;
        for y in y_ - 1..=y_ + 1 {
            // [4-2-3]상하로 루프시키지 않는 경우는 행이 범위 내인지 여부를 판정한다
            // if y < 0 || y >= FieldHeight as i64 {
            //     continue;
            // }
            // [4-2-5]상하로 루프한 Y좌표를 선언한다
            let rooped_y = (FIELD_HEIGHT as i64 + y) % FIELD_HEIGHT as i64;
            for x in x_ - 1..=x_ + 1 {
                // [4-2-7]좌우로 루프시키지 않는 경우는 열이 범위 내인지 여부를 판정한다
                // if x < 0 || x >= FieldWidth as i64 {
                //     continue;
                // }
                // [4-2-9]좌우로 루프한 X좌표를 선언한다
                let rooped_x = (FIELD_WIDTH as i64 + x) % FIELD_WIDTH as i64;
                // [4-2-10]대상 좌표가 중심 셀과 같은지 여부를 판정한다
                if rooped_x == x_ && rooped_y == y_ {
                    continue;
                }
                // [4-2-12]대상 셀이 살아 있으면 1을, 죽어 있으면 0을 가산한다
                if self.field[rooped_y as usize * FIELD_WIDTH + rooped_x as usize] {
                    count += 1;
                }
            }
        }
        count // [4-2-13]살아 있는 셀의 수를 반환한다
    }
    // [4-3]1스텝만큼의 시뮬레이션을 실행하는 함수를 선언한다
    pub fn step_simulation(&mut self) {
        // [4-3-1]다음 세대의 필드를 선언한다
        let mut next_field = vec![false; FIELD_HEIGHT * FIELD_WIDTH];
        for y in 0..FIELD_HEIGHT as i64 {
            for x in 0..FIELD_WIDTH as i64 {
                // [4-3-4]대상 셀과 인접하는 살아 있는 셀의 수를 선언한다
                let living_cell_count = self.get_living_cells_count(x, y);
                // [4-3-5]인접하는 살아 있는 셀의 수로 분기한다
                next_field[y as usize * FIELD_WIDTH + x as usize] = match living_cell_count {
                    0 | 1 => false,
                    2 => self.field[y as usize * FIELD_WIDTH + x as usize],
                    3 => true,
                    _ => false,
                };
            }
            println!();
        }
        // [4-3-13]다음 스텝의 필드를 현재 필드에 복사한다
        self.field.clone_from_slice(&next_field);
    }
    // [4-4]패턴을 필드에 복사하는 함수를 선언한다
    pub fn pattern_transfer(
        &mut self,
        dest_x: usize,
        dest_y: usize,
        src_width: usize,
        src_height: usize,
        p_pattern: &[bool],
    ) {
        for y in 0..src_height {
            for x in 0..src_width {
                // [4-4-3]패턴을 필드에 복사한다
                self.field[(dest_y + y) * FIELD_WIDTH + dest_x + x] = p_pattern[y * src_width + x];
            }
        }
    }
}

fn main() {
    let mut ctx = Context::new();

    const PATTERN_WIDTH: usize = 10;
    const PATTERN_HEIGHT: usize = 8;

    // [4-5-3]패턴을 선언한다
    let pattern = vec![
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true, false, false, false, false, false, false, false, true,
        false, true, true, false, false, false, false, false, false, true, false, true, false,
        false, false, false, false, false, false, true, false, false, false, false, false, false,
        false, true, false, false, false, false, false, false, false, true, false, true, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false,
    ];

    // [4-5-4]패턴을 필드의 중심에 복사한다
    ctx.pattern_transfer(
        FIELD_WIDTH / 2 - PATTERN_WIDTH / 2,
        FIELD_HEIGHT / 2 - PATTERN_HEIGHT / 2,
        PATTERN_WIDTH,
        PATTERN_HEIGHT,
        &pattern,
    );

    loop {
        match ctx.last_clock.elapsed() {
            Ok(elapsed) => {
                // [4-5-8]이전 회의 경과 시간에서 대기 시간이 경과하지 않으면
                if (elapsed.as_millis() as f32) < INTERVAL {
                    continue;
                }
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {e:?}");
                exit(0);
            }
        }

        // [4-5-10]이전 회 경과 시간을 현재의 경과 시간으로 갱신한다
        ctx.last_clock = SystemTime::now();

        // [4-5-11]필드를 그리는 함수를 호출한다
        ctx.draw_field();

        // [4-5-12]키보드 입력을 기다린다
        // match self.g.getch() {
        //     Ok(Key::Esc) => {
        //         exit(0);
        //     }
        //     _ => {}
        // }

        // [4-5-13]시뮬레이션을 진행한다
        ctx.step_simulation();
    }
}
