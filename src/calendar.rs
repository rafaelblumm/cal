use std::fmt;

use chrono::{Datelike, Month, Months, NaiveDate, Weekday};

#[derive(PartialEq, Debug)]
/// Calendar display length
pub enum CalendarLength {
    /// Single month
    MONTH,
    /// Year quarter (trimester)
    QUARTER,
    /// Whole year
    YEAR
}

#[derive(PartialEq, Debug)]
/// Calendar configuration
pub struct Calendar {
    /// Date's year
    year: i32,
    /// Date's month
    month: u32,
    /// Calendar display length
    length: CalendarLength
}

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = match self.length {
            CalendarLength::MONTH => Self::format_month(self.year, self.month, true),
            CalendarLength::QUARTER => {
                let trimester = Self::get_trimester(self.year, self.month);
                Self::format_trimester(trimester)
            },
            CalendarLength::YEAR => Self::format_year(self.year),
        };
        f.write_str(&lines.join("\n"))?;

        Ok(())
    }
}

impl Calendar {
    /// Creates a calendar with the given date and default display length (month)
    /// 
    /// * `year` - Date's year
    /// * `month` - Date's month
    pub fn new(year: i32, month: u32) -> Calendar {
        let length = CalendarLength::MONTH;
        Calendar {
            year,
            month,
            length
        }
    }

    /// Define calendar's display length.
    /// Returns self instance.
    /// 
    /// * `length` - Calendar's display length
    pub fn with_length(mut self, length: CalendarLength) -> Calendar {
        self.length = length;

        self
    }

    fn format_year(year: i32) -> Vec<String> {
        let mut trimesters = Vec::new();
        for month in (1..=12).step_by(3) {
            let mut months: Vec<Vec<String>> = Vec::new();
            for i in 0..3 {
                months.push(Self::format_month(year, month + i, false));
            }
            trimesters.push(months);
        }

        let mut lines = vec![
            format!("{:^66}", year),
            " ".repeat(66)
        ];
        for trimester in trimesters {
            lines.append(&mut Self::format_trimester(trimester));
        }

        lines
    }

    fn format_trimester(trimester: Vec<Vec<String>>) -> Vec<String> {
        let mut lines = Vec::new();
        for i in 0..8 {
            let mut line = Vec::new();
            for month in &trimester {
                if i < month.len() {
                    line.push(month[i].to_string());
                    continue;
                }
                line.push(" ".repeat(20));
            }
            lines.push(line.join("   "));
        }

        lines
    }

    fn format_month(year: i32, month: u32, show_year: bool) -> Vec<String> {
        let mut month_name = Month::try_from(month as u8).unwrap().name().to_string();
        if show_year {
            month_name.push_str(&format!(" {}", year));
        };

        let mut weeks = Self::format_weeks(Self::get_month_weeks(year, month));
        let mut lines = vec![
            format!("{:^20}", month_name),
            String::from("Su Mo Tu We Th Fr Sa"),  
        ];
        lines.append(&mut weeks);

        lines
    }

    fn format_weeks(weeks: Vec<Vec<u32>>) -> Vec<String> {
        let mut lines = Vec::new();

        for week in weeks {
            let days: Vec<String> = week.iter()
                .map(|day| {
                    if day.to_owned() == 0 {
                        String::from("  ")
                    } else {
                        format!("{:>2}", day)
                    }
                })
                .collect();
            lines.push(days.join(" "));
        }

        lines
    }

    /// Get informed and adjacent months
    fn get_trimester(year: i32, month: u32) -> Vec<Vec<String>> {
        let show_year = month == 1 || month == 12;

        let previous_month = if month == 1 {
            Self::format_month(year - 1, 12, show_year)
        } else {
            Self::format_month(year, month - 1, show_year)
        };
        let current_month = Self::format_month(year, month, show_year);
        let next_month = if month == 12 {
            Self::format_month(year + 1, 1, show_year)
        } else {
            Self::format_month(year, month + 1, show_year)
        };

        vec![
            previous_month,
            current_month,
            next_month
        ]
    }

    fn get_month_weeks(year: i32, month: u32) -> Vec<Vec<u32>> {
        let mut month_weeks = Vec::new();
        let mut week = vec![0; 7];

        for day in 1..=Self::get_days_in_month(year, month) {
            let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            let weekday = date.weekday();
            if weekday == Weekday::Sun && week.last().unwrap().to_owned() != 0 {
                month_weeks.push(week);
                week = vec![0; 7];
            }
            let index: usize = (weekday.number_from_sunday() - 1) as usize;
            week[index] = day;
        }

        if month_weeks.last().unwrap() != &week {
            month_weeks.push(week)
        }

        month_weeks
    }

    fn get_days_in_month(year: i32, month: u32) -> u32 {
        let start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let end = start.checked_add_months(Months::new(1)).unwrap();

        end.signed_duration_since(start).num_days() as u32
    }
}

#[cfg(test)]
mod calendar_tests {
    use super::Calendar;

    #[test]
    fn test_display() {
        let expected = String::new() +
            "    October 2024    \n" +
            "Su Mo Tu We Th Fr Sa\n" +
            "       1  2  3  4  5\n" +
            " 6  7  8  9 10 11 12\n" +
            "13 14 15 16 17 18 19\n" +
            "20 21 22 23 24 25 26\n" +
            "27 28 29 30 31      ";
        let result = Calendar::new(2024, 10)
            .with_length(super::CalendarLength::MONTH)
            .to_string();
        assert_eq!(expected, result);

        let expected = String::new() +
            "      January                February                March        \n" +
            "Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa\n" +
            "    1  2  3  4  5  6                1  2  3                   1  2\n" +
            " 7  8  9 10 11 12 13    4  5  6  7  8  9 10    3  4  5  6  7  8  9\n" +
            "14 15 16 17 18 19 20   11 12 13 14 15 16 17   10 11 12 13 14 15 16\n" +
            "21 22 23 24 25 26 27   18 19 20 21 22 23 24   17 18 19 20 21 22 23\n" +
            "28 29 30 31            25 26 27 28 29         24 25 26 27 28 29 30\n" +
            "                                              31                  ";
        let result = Calendar::new(2024, 2)
            .with_length(super::CalendarLength::QUARTER)
            .to_string();
        assert_eq!(expected, result);
        
        let expected = String::new() +
            "   December 2023           January 2024          February 2024    \n" +
            "Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa\n" +
            "                1  2       1  2  3  4  5  6                1  2  3\n" +
            " 3  4  5  6  7  8  9    7  8  9 10 11 12 13    4  5  6  7  8  9 10\n" +
            "10 11 12 13 14 15 16   14 15 16 17 18 19 20   11 12 13 14 15 16 17\n" +
            "17 18 19 20 21 22 23   21 22 23 24 25 26 27   18 19 20 21 22 23 24\n" +
            "24 25 26 27 28 29 30   28 29 30 31            25 26 27 28 29      \n" +
            "31                                                                ";
        let result = Calendar::new(2024, 1)
            .with_length(super::CalendarLength::QUARTER)
            .to_string();
        assert_eq!(expected, result);
        
        let expected = String::new() +
            "   November 2024          December 2024           January 2025    \n" +
            "Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa\n" +
            "                1  2    1  2  3  4  5  6  7             1  2  3  4\n" +
            " 3  4  5  6  7  8  9    8  9 10 11 12 13 14    5  6  7  8  9 10 11\n" +
            "10 11 12 13 14 15 16   15 16 17 18 19 20 21   12 13 14 15 16 17 18\n" +
            "17 18 19 20 21 22 23   22 23 24 25 26 27 28   19 20 21 22 23 24 25\n" +
            "24 25 26 27 28 29 30   29 30 31               26 27 28 29 30 31   \n" +
            "                                                                  ";
        let result = Calendar::new(2024, 12)
            .with_length(super::CalendarLength::QUARTER)
            .to_string();
        assert_eq!(expected, result);

        let expected = String::new() +
            "                               2024                               \n" +
            "                                                                  \n" +
            "      January                February                March        \n" +
            "Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa\n" +
            "    1  2  3  4  5  6                1  2  3                   1  2\n" +
            " 7  8  9 10 11 12 13    4  5  6  7  8  9 10    3  4  5  6  7  8  9\n" +
            "14 15 16 17 18 19 20   11 12 13 14 15 16 17   10 11 12 13 14 15 16\n" +
            "21 22 23 24 25 26 27   18 19 20 21 22 23 24   17 18 19 20 21 22 23\n" +
            "28 29 30 31            25 26 27 28 29         24 25 26 27 28 29 30\n" +
            "                                              31                  \n" +
            "       April                   May                    June        \n" +
            "Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa\n" +
            "    1  2  3  4  5  6             1  2  3  4                      1\n" +
            " 7  8  9 10 11 12 13    5  6  7  8  9 10 11    2  3  4  5  6  7  8\n" +
            "14 15 16 17 18 19 20   12 13 14 15 16 17 18    9 10 11 12 13 14 15\n" +
            "21 22 23 24 25 26 27   19 20 21 22 23 24 25   16 17 18 19 20 21 22\n" +
            "28 29 30               26 27 28 29 30 31      23 24 25 26 27 28 29\n" +
            "                                              30                  \n" +
            "        July                  August               September      \n" +
            "Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa\n" +
            "    1  2  3  4  5  6                1  2  3    1  2  3  4  5  6  7\n" +
            " 7  8  9 10 11 12 13    4  5  6  7  8  9 10    8  9 10 11 12 13 14\n" +
            "14 15 16 17 18 19 20   11 12 13 14 15 16 17   15 16 17 18 19 20 21\n" +
            "21 22 23 24 25 26 27   18 19 20 21 22 23 24   22 23 24 25 26 27 28\n" +
            "28 29 30 31            25 26 27 28 29 30 31   29 30               \n" +
            "                                                                  \n" +
            "      October                November               December      \n" +
            "Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa\n" +
            "       1  2  3  4  5                   1  2    1  2  3  4  5  6  7\n" +
            " 6  7  8  9 10 11 12    3  4  5  6  7  8  9    8  9 10 11 12 13 14\n" +
            "13 14 15 16 17 18 19   10 11 12 13 14 15 16   15 16 17 18 19 20 21\n" +
            "20 21 22 23 24 25 26   17 18 19 20 21 22 23   22 23 24 25 26 27 28\n" +
            "27 28 29 30 31         24 25 26 27 28 29 30   29 30 31            \n" +
            "                                                                  ";
        let result = Calendar::new(2024, 5)
            .with_length(super::CalendarLength::YEAR)
            .to_string();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_format_year() {
        let expected = vec![
            "                               2024                               ",
            "                                                                  ",
            "      January                February                March        ",
            "Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa",
            "    1  2  3  4  5  6                1  2  3                   1  2",
            " 7  8  9 10 11 12 13    4  5  6  7  8  9 10    3  4  5  6  7  8  9",
            "14 15 16 17 18 19 20   11 12 13 14 15 16 17   10 11 12 13 14 15 16",
            "21 22 23 24 25 26 27   18 19 20 21 22 23 24   17 18 19 20 21 22 23",
            "28 29 30 31            25 26 27 28 29         24 25 26 27 28 29 30",
            "                                              31                  ",
            "       April                   May                    June        ",
            "Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa",
            "    1  2  3  4  5  6             1  2  3  4                      1",
            " 7  8  9 10 11 12 13    5  6  7  8  9 10 11    2  3  4  5  6  7  8",
            "14 15 16 17 18 19 20   12 13 14 15 16 17 18    9 10 11 12 13 14 15",
            "21 22 23 24 25 26 27   19 20 21 22 23 24 25   16 17 18 19 20 21 22",
            "28 29 30               26 27 28 29 30 31      23 24 25 26 27 28 29",
            "                                              30                  ",
            "        July                  August               September      ",
            "Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa",
            "    1  2  3  4  5  6                1  2  3    1  2  3  4  5  6  7",
            " 7  8  9 10 11 12 13    4  5  6  7  8  9 10    8  9 10 11 12 13 14",
            "14 15 16 17 18 19 20   11 12 13 14 15 16 17   15 16 17 18 19 20 21",
            "21 22 23 24 25 26 27   18 19 20 21 22 23 24   22 23 24 25 26 27 28",
            "28 29 30 31            25 26 27 28 29 30 31   29 30               ",
            "                                                                  ",
            "      October                November               December      ",
            "Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa",
            "       1  2  3  4  5                   1  2    1  2  3  4  5  6  7",
            " 6  7  8  9 10 11 12    3  4  5  6  7  8  9    8  9 10 11 12 13 14",
            "13 14 15 16 17 18 19   10 11 12 13 14 15 16   15 16 17 18 19 20 21",
            "20 21 22 23 24 25 26   17 18 19 20 21 22 23   22 23 24 25 26 27 28",
            "27 28 29 30 31         24 25 26 27 28 29 30   29 30 31            ",
            "                                                                  "
        ];
        let result = Calendar::format_year(2024);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_format_trimester() {
        let expected = vec![
            "      January                February                March        ",
            "Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa",
            "    1  2  3  4  5  6                1  2  3                   1  2",
            " 7  8  9 10 11 12 13    4  5  6  7  8  9 10    3  4  5  6  7  8  9",
            "14 15 16 17 18 19 20   11 12 13 14 15 16 17   10 11 12 13 14 15 16",
            "21 22 23 24 25 26 27   18 19 20 21 22 23 24   17 18 19 20 21 22 23",
            "28 29 30 31            25 26 27 28 29         24 25 26 27 28 29 30",
            "                                              31                  "
        ];
        let mut trimester = Vec::new();
        for i in 1..=3 {
            trimester.push(Calendar::format_month(2024, i, false));
        }
        let result = Calendar::format_trimester(trimester);
        assert_eq!(expected, result);

        let expected = vec![
            "    January 2024          February 2024            March 2024     ",
            "Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa   Su Mo Tu We Th Fr Sa",
            "    1  2  3  4  5  6                1  2  3                   1  2",
            " 7  8  9 10 11 12 13    4  5  6  7  8  9 10    3  4  5  6  7  8  9",
            "14 15 16 17 18 19 20   11 12 13 14 15 16 17   10 11 12 13 14 15 16",
            "21 22 23 24 25 26 27   18 19 20 21 22 23 24   17 18 19 20 21 22 23",
            "28 29 30 31            25 26 27 28 29         24 25 26 27 28 29 30",
            "                                              31                  "
        ];
        let mut trimester = Vec::new();
        for i in 1..=3 {
            trimester.push(Calendar::format_month(2024, i, true));
        }
        let result = Calendar::format_trimester(trimester);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_format_month() {
        let expected = vec![
            "    October 2024    ",
            "Su Mo Tu We Th Fr Sa",
            "       1  2  3  4  5",
            " 6  7  8  9 10 11 12",
            "13 14 15 16 17 18 19",
            "20 21 22 23 24 25 26",
            "27 28 29 30 31      "
        ];
        let result = Calendar::format_month(2024, 10, true);
        assert_eq!(expected, result);

        let expected = vec![
            "     January 90     ",
            "Su Mo Tu We Th Fr Sa",
            " 1  2  3  4  5  6  7",
            " 8  9 10 11 12 13 14",
            "15 16 17 18 19 20 21",
            "22 23 24 25 26 27 28",
            "29 30 31            "
        ];
        let result = Calendar::format_month(90, 1, true);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_format_weeks() {
        // Starts and ends in the middle of the week
        let expected = vec![
            "       1  2  3  4  5",
            " 6  7  8  9 10 11 12",
            "13 14 15 16 17 18 19",
            "20 21 22 23 24 25 26",
            "27 28 29 30 31      "
        ];
        let weeks = Calendar::get_month_weeks(2024, 10);
        let result = Calendar::format_weeks(weeks);
        assert_eq!(expected, result);

        // Starts on first day of the week
        let expected = vec![
            " 1  2  3  4  5  6  7",
            " 8  9 10 11 12 13 14",
            "15 16 17 18 19 20 21",
            "22 23 24 25 26 27 28",
            "29 30               "
        ];
        let weeks = Calendar::get_month_weeks(2024, 9);
        let result = Calendar::format_weeks(weeks);
        assert_eq!(expected, result);

        // Starts on last day of the week and ends on first day of the week
        let expected = vec![
            "                   1",
            " 2  3  4  5  6  7  8",
            " 9 10 11 12 13 14 15",
            "16 17 18 19 20 21 22",
            "23 24 25 26 27 28 29",
            "30                  "
        ];
        let weeks = Calendar::get_month_weeks(2024, 6);
        let result = Calendar::format_weeks(weeks);
        assert_eq!(expected, result);

        // Ends on last day of the week
        let expected = vec![
            "                1  2",
            " 3  4  5  6  7  8  9",
            "10 11 12 13 14 15 16",
            "17 18 19 20 21 22 23",
            "24 25 26 27 28 29 30"
        ];
        let weeks = Calendar::get_month_weeks(2024, 11);
        let result = Calendar::format_weeks(weeks);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_trimester() {
        let expected = vec![
            vec![
                "      January       ",
                "Su Mo Tu We Th Fr Sa",
                "    1  2  3  4  5  6",
                " 7  8  9 10 11 12 13",
                "14 15 16 17 18 19 20",
                "21 22 23 24 25 26 27",
                "28 29 30 31         "
            ],
            vec![
                "      February      ",
                "Su Mo Tu We Th Fr Sa",
                "             1  2  3",
                " 4  5  6  7  8  9 10",
                "11 12 13 14 15 16 17",
                "18 19 20 21 22 23 24",
                "25 26 27 28 29      "
            ],
            vec![
                "       March        ",
                "Su Mo Tu We Th Fr Sa",
                "                1  2",
                " 3  4  5  6  7  8  9",
                "10 11 12 13 14 15 16",
                "17 18 19 20 21 22 23",
                "24 25 26 27 28 29 30",
                "31                  "
            ]
        ];
        let result = Calendar::get_trimester(2024, 2);
        assert_eq!(expected, result);

        let expected = vec![
            vec![
                "   December 2023    ",
                "Su Mo Tu We Th Fr Sa",
                "                1  2",
                " 3  4  5  6  7  8  9",
                "10 11 12 13 14 15 16",
                "17 18 19 20 21 22 23",
                "24 25 26 27 28 29 30",
                "31                  "
            ],
            vec![
                "    January 2024    ",
                "Su Mo Tu We Th Fr Sa",
                "    1  2  3  4  5  6",
                " 7  8  9 10 11 12 13",
                "14 15 16 17 18 19 20",
                "21 22 23 24 25 26 27",
                "28 29 30 31         "
            ],
            vec![
                "   February 2024    ",
                "Su Mo Tu We Th Fr Sa",
                "             1  2  3",
                " 4  5  6  7  8  9 10",
                "11 12 13 14 15 16 17",
                "18 19 20 21 22 23 24",
                "25 26 27 28 29      "
            ]
        ];
        let result = Calendar::get_trimester(2024, 1);
        assert_eq!(expected, result);

        let expected = vec![
            vec![
                "   November 2024    ",
                "Su Mo Tu We Th Fr Sa",
                "                1  2",
                " 3  4  5  6  7  8  9",
                "10 11 12 13 14 15 16",
                "17 18 19 20 21 22 23",
                "24 25 26 27 28 29 30",
            ],
            vec![
                "   December 2024    ",
                "Su Mo Tu We Th Fr Sa",
                " 1  2  3  4  5  6  7",
                " 8  9 10 11 12 13 14",
                "15 16 17 18 19 20 21",
                "22 23 24 25 26 27 28",
                "29 30 31            "
            ],
            vec![
                "    January 2025    ",
                "Su Mo Tu We Th Fr Sa",
                "          1  2  3  4",
                " 5  6  7  8  9 10 11",
                "12 13 14 15 16 17 18",
                "19 20 21 22 23 24 25",
                "26 27 28 29 30 31   "
            ]
        ];
        let result = Calendar::get_trimester(2024, 12);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_month_weeks() {
        let expected: Vec<Vec<u32>> = vec![
            vec![0, 0, 1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10, 11, 12],
            vec![13, 14, 15, 16, 17, 18, 19],
            vec![20, 21, 22, 23, 24, 25, 26],
            vec![27, 28, 29, 30, 31, 0, 0]
        ];
        let result = Calendar::get_month_weeks(2024, 10);
        assert_eq!(expected, result);

        let expected: Vec<Vec<u32>> = vec![
            vec![0, 0, 0, 0, 1, 2, 3],
            vec![4, 5, 6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15, 16, 17],
            vec![18, 19, 20, 21, 22, 23, 24],
            vec![25, 26, 27, 28, 29, 0, 0]
        ];
        let result = Calendar::get_month_weeks(2024, 2);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_days_in_month() {
        let result = Calendar::get_days_in_month(2024, 1);
        assert_eq!(31, result);

        let result = Calendar::get_days_in_month(2024, 2);
        assert_eq!(29, result);
    }
}