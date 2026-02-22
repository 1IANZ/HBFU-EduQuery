use crate::app::error::{ApiError, ApiResult};
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreInfo {
    pub id: u32,
    pub term: String,
    pub course_id: String,
    pub course_name: String,
    pub score: f32,
    pub credit: f32,
    pub hours: f32,
    pub gpa: f32,
    pub exam_type: String,
    pub course_attr: String,
    pub course_nature: String,
}
pub fn parse_score(html: &str) -> ApiResult<Vec<ScoreInfo>> {
    let document = Html::parse_document(html);
    let table_selector = Selector::parse("#dataList").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let Some(table) = document.select(&table_selector).next() else {
        // 没有数据时返回空数组
        return Ok(Vec::new());
    };

    let mut scores = Vec::new();

    for (i, row) in table.select(&row_selector).enumerate() {
        if i == 0 {
            continue;
        }

        let cells: Vec<_> = row.select(&cell_selector).collect();
        if cells.len() < 11 {
            continue;
        }

        let parse_text = |index: usize| {
            cells
                .get(index)
                .map(|c| c.text().collect::<String>().trim().to_string())
                .unwrap_or_default()
        };

        let parse_f32 = |index: usize| parse_text(index).parse::<f32>().unwrap_or(0.0);

        let parse_u32 = |index: usize| parse_text(index).parse::<u32>().unwrap_or(0);

        let score = ScoreInfo {
            id: parse_u32(0),
            term: parse_text(1),
            course_id: parse_text(2),
            course_name: parse_text(3),
            score: parse_f32(4),
            credit: parse_f32(5),
            hours: parse_f32(6),
            gpa: parse_f32(7),
            exam_type: parse_text(8),
            course_attr: parse_text(9),
            course_nature: parse_text(10),
        };
        scores.push(score);
    }
    Ok(scores)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreSummary {
    pub credit_total: f32,
    pub gpa_average: f32,
}
pub fn parse_score_summary(html: &str) -> ApiResult<ScoreSummary> {
    let document = Html::parse_document(html);
    let table_selector = Selector::parse(r#"table[style*="text-align: center"]"#)
        .map_err(|_| ApiError::Custom("未找到成绩汇总表格".to_string()))?;
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    // 如果没有汇总表格，返回默认值
    let Some(table) = document.select(&table_selector).next() else {
        return Ok(ScoreSummary {
            credit_total: 0.0,
            gpa_average: 0.0,
        });
    };

    let Some(summary_row) = table.select(&tr_selector).next() else {
        return Ok(ScoreSummary {
            credit_total: 0.0,
            gpa_average: 0.0,
        });
    };

    let tds: Vec<_> = summary_row.select(&td_selector).collect();
    if tds.len() < 8 {
        return Ok(ScoreSummary {
            credit_total: 0.0,
            gpa_average: 0.0,
        });
    }
    let credit_text = tds[5].text().collect::<String>().trim().to_string();
    let gpa_text = tds[7].text().collect::<String>().trim().to_string();
    let credit_total = credit_text.parse::<f32>().unwrap_or(0.0);
    let gpa_average = gpa_text.parse::<f32>().unwrap_or(0.0);
    Ok(ScoreSummary {
        credit_total,
        gpa_average,
    })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub info: Vec<ScoreInfo>,
    pub summary: ScoreSummary,
}
pub fn parse_score_all(html: &str) -> ApiResult<Score> {
    let info = parse_score(html)?;
    let summary = parse_score_summary(html)?;
    Ok(Score { info, summary })
}
