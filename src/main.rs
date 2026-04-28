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
};

mod division;
use division::Problem;

struct App {
    division: Problem,
}

impl App {
    fn new() -> Self {
        Self {
            division: Problem::new(),
        }
    }
}


// fn run_app<B: ratatui::backend::Backend> (terminal: &mut Terminal<B>) -> io::Result<()> {
fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> where std::io::Error: From<<B as Backend>::Error>{

    // 무한 루프: 매 반복마다 "그리기 → 입력 받기"를 수행합니다. 사용자가 'q' 키를 누르면 루프를 탈출하여 앱이 종료됩니다.
    loop { //주 기능을 위한 main loop.
        // 1. 화면 그리기.
        terminal.draw( |frame| {
            let area = frame.area(); //현재 전체 화면의 크기 가져오기.
            let block = Block::default() //블록 위젯 생성.
                .title("Terminal UI Math Game") //블록 제목 설정.
                .borders(Borders::ALL); //모든 테두리 표시.

            let paragraph = Paragraph::new("Press 'q' to quit") //Text 위젯 생성. 위에서 만든 박스를 배경으로 적용.
                .block(block); // 위젯을 블록에 추가.

            frame.render_widget(paragraph, area); //위젯을 화면 전체에 렌더링.
        })?;

        // 2. 입력 처리.
        if let Event::Key(key) = event::read()? {
            //Caution: event::read()는 블로킹 함수입니다. 입력이 있을 때까지 기다립니다.
            //Key event종류는 Press, Release, Repeat이 있음. 여기에서는 Press 이벤트만 처리.
            if key.kind == KeyEventKind::Press {
                if let KeyCode::Char('q') = key.code {
                    return Ok(()); // 'q' 키가 눌리면 앱 종료. Ok(())는 성공적으로 종료되었음을 나타냄.
                }
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
    let result = run_app(&mut terminal); //main loop 실행. 결과는 io::Result<()> 타입.

    // 3. 종료시 터미널 복원.
    disable_raw_mode()?; // Raw mode 해제. 화면이 다시 엔터키 입력을 기다리는 일반 모드로 돌아감.
    terminal.show_cursor()?; // 커서 보이도록 설정.

    result // run_app의 결과 반환. 성공 시 Ok(()), 에러 시 Err(e) 형태.
}

