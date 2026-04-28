use rand::RngExt;

#[derive(Clone)]
pub struct Problem {
    pub dividend: u32,  //피제수 (나누어지는 수)
    pub divisor: u32,   //제수 (나누는 수)
    pub answer: u32,    //몫 (결과)
}

impl Problem {
    pub fn new() -> Self {
        let mut rng = rand::rng();

        let divisor = rng.random_range(2..=9); //제수 2~9 (1은 무의미)
        let quotient = rng.random_range(2..=9);//몫 2~9
        let dividend = divisor * quotient; //피제수는 제수와 몫의 곱 (항상 정수가 답이 되도록)

        Self {
            dividend,
            divisor,
            answer: quotient
        }
    }

    // 화면 출력용 문제 텍스트 생성 메서드. 예: "8 ÷ 2 = ?"
    pub fn question_text(&self) -> String {
        // 숫자를 문자열로 변환 (자릿수 계산용)
        let dividend_str = self.dividend.to_string();
        let divisor_str = self.divisor.to_string();

        // 윗줄: "제수 + )" 자리만큼 공백을 두고, 피제수 자릿수만큼 밑줄
        // 예) divisor="7"(1자), dividend="56"(2자) → "  " + "__" = "  __"
        let top = format!(
            "{}{}",
            " ".repeat(divisor_str.len() + 1), // +1은 ')' 한 칸
            "_".repeat(dividend_str.len())
        );

        // 아랫줄: 제수 + ")" + 피제수
        // 예) "7" + ")" + "56" = "7)56"
        let bottom = format!("{}){}", divisor_str, dividend_str);

        // 두 줄을 개행으로 합치기 → Paragraph가 자동으로 여러 줄로 렌더링
        format!("{}\n{}", top, bottom)
    }
}