use crate::app::error::{ApiError, ApiResult};
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseSchedule {
    pub id: String,            // "星期几-节次-课程名"
    pub name: String,          // 课程名称
    pub teacher: String,       // 教师
    pub time_range: String,    // 时间范围 "08:00~09:30"
    pub day_of_week: String,   // 星期几 "星期一"~"星期日"
    pub section: usize,        // 节次 1~8
    pub weeks: String,         // 周次 "4-17周"
    pub weeks_array: Vec<i32>, // 周次数组 [4, 5, 6, ..., 17]
    pub classroom: String,     // 教室
    pub duration: String,      // 持续节次 "01-02节"
}

/// 解析周次字符串为数组，如 "4-17周" -> [4, 5, 6, ..., 17]
fn parse_weeks(weeks_str: &str) -> Vec<i32> {
    if weeks_str.is_empty() {
        return vec![];
    }

    // 移除"周"字和空格
    let cleaned = weeks_str.replace("周", "").replace(" ", "");
    if cleaned.is_empty() {
        return vec![];
    }

    let mut weeks = Vec::new();

    // 按逗号分隔
    for part in cleaned.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        // 检查是否是范围 (例如: "1-17")
        if part.contains('-') {
            let parts: Vec<&str> = part.splitn(2, '-').collect();
            if parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (
                    parts[0].trim().parse::<i32>(),
                    parts[1].trim().parse::<i32>(),
                ) {
                    for w in start..=end {
                        weeks.push(w);
                    }
                }
            }
        } else {
            // 单个周次
            if let Ok(w) = part.parse::<i32>() {
                weeks.push(w);
            }
        }
    }

    // 去重并排序
    weeks.sort();
    weeks.dedup();
    weeks
}

fn extract_text(el: Option<scraper::element_ref::ElementRef>, selector: &str) -> String {
    if let Some(e) = el {
        e.select(&Selector::parse(selector).unwrap())
            .next()
            .map(|n| n.text().collect::<String>().trim().to_string())
            .unwrap_or_default()
    } else {
        String::new()
    }
}

fn extract_duration(el: Option<scraper::element_ref::ElementRef>) -> String {
    if let Some(e) = el {
        let html = e.inner_html();
        let re = regex::Regex::new(r"\[((?:\d{2}-?)+)]节?").unwrap();
        if let Some(caps) = re.captures(&html) {
            return format!("{}节", &caps[1]);
        }
    }
    String::new()
}

fn extract_time_ranges(text: &str) -> String {
    let re = regex::Regex::new(r"\d{2}:\d{2}~\d{2}:\d{2}").unwrap();
    let matches: Vec<String> = re.find_iter(text).map(|m| m.as_str().to_string()).collect();
    if matches.is_empty() {
        String::new()
    } else {
        matches.join("-")
    }
}

pub fn parse_course_schedule(html: &str) -> ApiResult<Vec<CourseSchedule>> {
    let doc = Html::parse_document(html);
    let table = doc
        .select(&Selector::parse("#kbtable").unwrap())
        .next()
        .ok_or_else(|| ApiError::Custom("未找到id=kbtable的表格".to_string()))?;

    let week_days = [
        "星期一",
        "星期二",
        "星期三",
        "星期四",
        "星期五",
        "星期六",
        "星期日",
    ];
    let section_names = ["一", "二", "三", "四", "五", "六", "七", "八"];

    let mut courses = Vec::new();

    for row in table.select(&Selector::parse("tr").unwrap()).skip(1) {
        let section_th = row.select(&Selector::parse("th").unwrap()).next();
        if section_th.is_none() {
            continue;
        }
        let section_text = section_th
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_string();
        let section_num = section_names
            .iter()
            .position(|&cn| section_text.contains(&format!("第{}大节", cn)))
            .map(|idx| idx + 1);
        if section_num.is_none() {
            continue;
        }
        let section_num = section_num.unwrap();

        let time_range = extract_time_ranges(&section_text);

        for (day_index, cell) in row
            .select(&Selector::parse("td").unwrap())
            .take(7)
            .enumerate()
        {
            let course_div = cell.select(&Selector::parse(".kbcontent1").unwrap()).next();
            if course_div.is_none()
                || course_div
                    .as_ref()
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .trim()
                    .is_empty()
            {
                continue;
            }
            let course_name = course_div
                .unwrap()
                .inner_html()
                .split("<br>")
                .next()
                .unwrap_or("")
                .trim()
                .to_string();

            let weeks =
                extract_text(Some(cell.clone()), "font[title*=\"周次\"]").replace("(周)", "周");
            let classroom = extract_text(Some(cell.clone()), "font[title*=\"教室\"]");
            let detail_div = cell.select(&Selector::parse(".kbcontent").unwrap()).next();

            let teacher = extract_text(detail_div, "font[title*=\"老师\"]");
            let duration = extract_duration(detail_div);

            courses.push(CourseSchedule {
                id: format!("{}-{}-{}", day_index + 1, section_num, course_name),
                name: course_name,
                teacher: if teacher.is_empty() {
                    "未提供".to_string()
                } else {
                    teacher
                },
                time_range: time_range.clone(),
                day_of_week: week_days[day_index].to_string(),
                section: section_num,
                weeks_array: parse_weeks(&weeks),
                weeks,
                classroom,
                duration: if duration.is_empty() {
                    "未提供".to_string()
                } else {
                    duration
                },
            });
        }
    }
    Ok(courses)
}
