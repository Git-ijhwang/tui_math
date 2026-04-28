use std::io;

use crossterm::{
    execute,
    event::{
        self, Event, KeyCode, KeyEventKind
    },
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen
    },
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
    prelude::Backend, //Backend 트레이트를 가져와서 run_app 함수에서 제네릭으로 사용할 수 있도록 함.
    layout::{Alignment, Constraint, Direction, Layout}, // Alignment 열거형을 가져와서 텍스트 정렬에 사용할 수 있도록 함.
    style::{Color, Style},
    text::{Line, Span},
};

mod division;
use division::Problem;

enum Mode {
    Asking,
    ShowingResult {
        correct: bool
    },
}

struct Attempt {
    problem: Problem,
    user_answer: u32,
}

impl Attempt {
    fn is_correct(&self) -> bool {
        self.user_answer == self.problem.answer
    }
}

struct App {
    problem: Problem,
    input: String,
    mode: Mode,
    history: Vec<Attempt>,
}

impl App {
    fn new() -> Self {
        Self {
            problem: Problem::new(),
            input: String::new(),
            mode: Mode::Asking, //처음에는 문제를 묻는 모드로 시작
            history: Vec::new(),
        }
    }

    //숫자 한 글자 추가( 입력이 너무 길어지지 않게 최대 자릿수 제한.
    fn push_digit(&mut self, c: char) {

        if self.input.len() < 3 {
            self. input.push(c);
        }
    }

    //숫자 한 글자 제거 (백스페이스 기능)
    fn pop_digit(&mut self) {
        self.input.pop();
    }

    fn submit (&mut self ) {
        if self.input.is_empty() {
            return; //입력이 비어있으면 제출 무시.
        }

        // 문자열 -> u32 변환. 숫자만 받았으니 실패하지 않지만 unwrap() 대신 unwrap_or()로 명확한 에러 메시지 제공.
        let user_answer: u32 = self.input.parse()
            .unwrap_or(u32::MAX);
        let correct = user_answer == self.problem.answer; //사용자 답과 정답 비교.

        //풀이 기록에 추가.
        self.history.push(Attempt {
            problem: self.problem.clone(), //현재 문제 복사해서 저장.
            user_answer, //사용자 답 저장.
        });

        self.mode = Mode::ShowingResult { correct }; //결과 보여주는 모드로 전환.
    }

    fn next_problem(&mut self) {
        self.problem = Problem::new(); //새 문제 생성.
        self.input.clear(); //입력 초기화.
        self.mode = Mode::Asking; //문제 묻는 모드로 전환.
    }

    //맞힌 개수 세는 메서드.
    fn correct_count(&self) -> usize {
        self.history.iter().filter( |attempt| attempt.is_correct()).count()
    }

    //정답률 계산 메서드.
    fn accuracy_percent(&self) -> Option<u32> {
        let total = self.history.len();
        if total == 0 {
            return None;
        }

        let pct = (self.correct_count() as u32 * 100) / total as u32;
        Some(pct)
    }
}


// fn run_app<B: ratatui::backend::Backend> (terminal: &mut Terminal<B>) -> io::Result<()> {
fn run_app<B: ratatui::backend::Backend>( terminal: &mut Terminal<B>,
                                          app: &mut App )
    -> io::Result<()> where std::io::Error: From<<B as Backend>::Error>
{

    // 무한 루프: 매 반복마다 "그리기 → 입력 받기"를 수행합니다. 사용자가 'q' 키를 누르면 루프를 탈출하여 앱이 종료됩니다.
    loop { //주 기능을 위한 main loop.
        // 1. 화면 그리기.
        terminal.draw( |frame| {
            let area = frame.area(); //현재 전체 화면의 크기 가져오기.
            
            //화면 좌우 5:5 로 분할하기
            let main_chunks = Layout::default()
                .direction(Direction::Horizontal) // .horizontal()는 수평 방향으로 레이아웃을 나누겠다는 의미입니다. 즉, 화면을 좌우로 나눕니다.
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]) // .constraints()는 레이아웃의 각 부분이 화면에서 차지하는 공간을 정의합니다. 여기서는 두 부분 모두 화면의 50%씩 차지하도록 설정했습니다.
                .split(area); // .split()은 실제로 화면을 나누는 메서드입니다. 여기서는 전체 화면(area)을 위에서 정의한 레이아웃(chunks)으로 나눕니다. 결과적으로 chunks[0]은 화면의 왼쪽 절반

            let left_area = main_chunks[0]; //왼쪽 절반 영역 ==> 상/하 30:70으로 나눌 예정.
            let right_area = main_chunks[1]; //오른쪽 절반 영역

            let left_chunks = Layout::default()
                .direction(Direction::Vertical) //수직 방향으로 레이아웃 나누기. 즉, 왼쪽 영역을 위아래로 나눕니다.
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)]) // .constraints()는 레이아웃의 각 부분이 화면에서 차지하는 공간을 정의합니다. 여기서는 첫 번째 부분이 최소 3줄을 차지하도록 하고, 두 번째 부분은 정확히 3줄을 차지하도록 설정했습니다.
                .split(left_area); // .split()은 실제로 화면을 나누는 메서드입니다. 여기서는 왼쪽 영역(left_area)을 위에서 정의한 레이아웃(chunks)으로 나눕니다. 결과적으로 left_chunks[0]은 왼쪽 영역의 상단 부분, left_chunks[1]은 하단 부분이 됩니다.
            let stats_area = left_chunks[0]; //왼쪽 영역의 상단 부분 (통계 표시용)
            let quiz_area = left_chunks[1]; //왼쪽 영역의 하단 부분 (문제 표시용)


            // 왼쪽 영역
            // 왼쪽 위 영역: 통계 표시
            let total = app.history.len();
            let correct = app.correct_count();

            let stats_text = match app.accuracy_percent() {
                None => "No attempts yet".to_string(),
                Some(pct) => format!("Accuracy: {}% ({} / {})", pct, correct, total),
            };

            let stats_block = Block::default()
                .title(" Accuracy ")
                .borders(Borders::ALL);
            let stats_paragraph = Paragraph::new(stats_text)
                .alignment(Alignment::Center)
                .block(stats_block);
            frame.render_widget(stats_paragraph, stats_area);

            // 왼쪽 아래 영역: 문제 표시
            let question = app.problem.question_text(); //문제 생성.

            let left_lines: Vec<Line> = match &app.mode {
                Mode::Asking => {
                    let input_display = if app.input.is_empty() {
                        "_".to_string() //입력이 비어있으면 밑줄로 표시.
                    }
                    else {
                        app.input.clone() //입력이 있으면 그대로 표시.
                    };

                    let mut v: Vec<Line> = question
                        .lines()
                        .map(|s| Line::from(s.to_string()))
                        .collect();

                    v.push(Line::from("")); //문제와 입력 사이의 빈 줄.
                    v.push(Line::from(format!("Your Answer: {}", input_display))); //사용자 입력 상태 표시.
                    v.push(Line::from(""));
                    v.push(Line::from("Press Enter to submit"));
                    v
                }

                Mode::ShowingResult { correct } => {
                    //정답 여부에 따라 결과 메시지 생성
                    let (text, color) = if *correct {
                        ("Correct!".to_string(), Color::Green)
                    } else {
                        (
                            format!("Wrong! The correct answer is {}", app.problem.answer),
                            Color::Red
                        )
                    };

                    let mut v: Vec<Line> = question
                        .lines()
                        .map(|s| Line::from(s.to_string()))
                        .collect();
                    v.push(Line::from(""));
                    v.push(Line::from(format!("Your answer: {}", app.input)));
                    v.push(Line::from(""));
                        // Span으로 감싸 색상 적용
                    v.push(Line::from(Span::styled(text, Style::default().fg(color))));
                    v.push(Line::from(""));
                    v.push(Line::from("Press Enter for next problem"));
                    v
                }
            };

            // let input_display = if app.input.is_empty() {
            //     "_".to_string()
            // }
            // else {
            //     app.input.clone()
            // };

            // let body = format!("{}\n\nYour Answer: {}", question, input_display); //문제와 입력 상태를 하나의 문자열로 합치기.

            let quiz_block = Block::default() //블록 위젯 생성.
                .title(" Terminal UI Math Game ") //블록 제목 설정.
                .borders(Borders::ALL); //모든 테두리 표시.

            let quiz_paragraph = Paragraph::new(left_lines) //Text 위젯 생성. 위에서 만든 body 출력.
                .alignment(Alignment::Center) //텍스트 중앙 정렬..
                .block(quiz_block); // 위젯을 블록에 추가.

            frame.render_widget(quiz_paragraph, quiz_area);


            
            // 오른쪽 영역
            let history_lines: Vec<Line> = app
                .history //문제 풀이 기록을 보여주는 오른쪽 영역의 텍스트 생성. 현재는 빈 벡터로 초기화.
                .iter() // .iter()는 history 벡터의 각 요소에 접근하기 위한 반복자입니다.
                // .rev() //iter()로 얻은 요소를 역순으로 뒤집기.
                .map (|entry| {
                    let (mark, color) = if entry.is_correct() {
                        ("✓", Color::Green)
                    } else {
                        ("✗", Color::Red)
                    };

                    let text = if entry.is_correct() {
                        format!("{}  {} ÷ {} = {}",
                            mark,
                            entry.problem.dividend,
                            entry.problem.divisor,
                            entry.problem.answer
                        )
                    } else {
                        format!("{}  {} ÷ {} = {}, your answer: {}",
                            mark,
                            entry.problem.dividend,
                            entry.problem.divisor,
                            entry.problem.answer,
                            entry.user_answer
                        )
                    };
                    Line::from(Span::styled(text, Style::default().fg(color)))
                })
                .collect();

            let right_block = Block::default().title(" History ").borders(Borders::ALL);
            let right_paragraph = Paragraph::new(history_lines) //오른쪽 영역에 기록 표시하는 Paragraph 위젯 생성.
                .alignment(Alignment::Left) //텍스트 왼쪽 정렬.
                .block(right_block); //블록 추가.

            frame.render_widget(right_paragraph, right_area); //위젯을 화면 전체에 렌더링.
        })?;

        // 2. 입력 처리.
        if let Event::Key(key) = event::read()? {
            //Caution: event::read()는 블로킹 함수입니다. 입력이 있을 때까지 기다립니다.
            //Key event종류는 Press, Release, Repeat이 있음. 여기에서는 Press 이벤트만 처리.
            if key.kind != KeyEventKind::Press {
                continue; //Press 이벤트가 아니면 무시하고 다음 루프로 넘어감.
            }

            if let KeyCode::Char('q') = key.code {
                return Ok(()); // 'q' 키가 눌리면 앱 종료.
            }


            //mode별 동작 처리
            match &app.mode {
                Mode::Asking => match key.code {
                    KeyCode::Char(c) if c.is_ascii_digit() => app.push_digit(c),
                    KeyCode::Backspace => app.pop_digit(),
                    KeyCode::Enter => app.submit(),
                    _ => {}
                },

                Mode::ShowingResult { .. } => match key.code {
                    KeyCode::Enter => app.next_problem(),
                    _ => {} //결과 보여주는 모드에서는 Enter와 q 외에는 다른 입력 무시.
                },
            }
        }
    }
}

fn main() -> io::Result<()>{
    
    // 1. Set up Terminal
    enable_raw_mode()?; // 입력을 엔터 없이 한글자씩 즉시 받기 위해 raw mode로 전환.
    let mut stdout = io::stdout(); //표준 출력 가져옴.
    execute!(stdout, EnterAlternateScreen)?; // 대체 화면으로 전환. 기존 화면은 보존됨. App종료후 기존 쉘 화면으로 복원.

    let backend = CrosstermBackend::new(stdout); // crossterm을 이용한 터미널 백엔드 생성.
    let mut terminal = Terminal::new(backend)?; // 터미널 객체 생성.

    // 2. App 실행
    let mut app = App::new(); // 앱 상태 초기화. 문제 생성 및 입력 초기화.
    let result = run_app(&mut terminal, &mut app); //main loop 실행. 결과는 io::Result<()> 타입.

    // 3. 종료시 터미널 복원.
    disable_raw_mode()?; // Raw mode 해제. 화면이 다시 엔터키 입력을 기다리는 일반 모드로 돌아감.
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?; // 대체 화면에서 나와서 원래 화면으로 복원.
    terminal.show_cursor()?; // 커서 보이도록 설정.

    result // run_app의 결과 반환. 성공 시 Ok(()), 에러 시 Err(e) 형태.
}

