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
    layout::Alignment, // Alignment 열거형을 가져와서 텍스트 정렬에 사용할 수 있도록 함.
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
struct App {
    problem: Problem,
    input: String,
    mode: Mode,
}

impl App {
    fn new() -> Self {
        Self {
            problem: Problem::new(),
            input: String::new(),
            mode: Mode::Asking, //처음에는 문제를 묻는 모드로 시작
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
        self.mode = Mode::ShowingResult { correct }; //결과 보여주는 모드로 전환.
    }

    fn next_problem(&mut self) {
        self.problem = Problem::new(); //새 문제 생성.
        self.input.clear(); //입력 초기화.
        self.mode = Mode::Asking; //문제 묻는 모드로 전환.
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

            let question = app.problem.question_text(); //문제 생성.

            let lines: Vec<Line> = match &app.mode {
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

            let block = Block::default() //블록 위젯 생성.
                .title(" Terminal UI Math Game ") //블록 제목 설정.
                .borders(Borders::ALL); //모든 테두리 표시.

            let paragraph = Paragraph::new(lines) //Text 위젯 생성. 위에서 만든 body 출력.
                .alignment(Alignment::Center) //텍스트 중앙 정렬..
                .block(block); // 위젯을 블록에 추가.

            frame.render_widget(paragraph, area); //위젯을 화면 전체에 렌더링.
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
    terminal.show_cursor()?; // 커서 보이도록 설정.

    result // run_app의 결과 반환. 성공 시 Ok(()), 에러 시 Err(e) 형태.
}

