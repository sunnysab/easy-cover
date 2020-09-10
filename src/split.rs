use jieba_rs::Jieba;

/// Original title
pub struct Title;

/// Split title
#[derive(Default, Debug)]
pub struct TitleResult {
    /// Full title string
    pub origin: String,
    /// Tags in the brace.
    pub tags: Vec<String>,
    /// Topic
    pub topic: Vec<String>,
}

impl Title {
    /// Parse title by regex.
    ///
    /// In common, titles are like `【计算机学院】2020年大学生暑期社会实践宣讲会活动`
    ///
    /// # Example
    /// ```rust
    /// let strings = Title::regex("【计算机学院】2020年大学生暑期社会实践宣讲会活动");
    /// assert_eq!(strings[0], "【计算机学院】");
    /// assert_eq!(strings[0], "2020年大学生暑期社会实践宣讲会活动");
    /// ```
    fn split(title: &str) -> Vec<String> {
        let mut current = String::new();
        let mut result = Vec::<String>::new();

        let processed_title = title.replace("[", "【").replace("]", "】");
        for ch in processed_title.chars().into_iter() {
            match ch {
                '】' => {
                    current.push(ch);
                    result.push(current);
                    current = String::new();
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        result.push(current);
        result
    }

    /// Remove the parentheses on the sides of the string
    ///
    /// # Example
    /// ```rust
    /// assert_eq!(remove_parentheses("【计算机学院】"), "计算机学院");
    /// ```
    fn remove_parentheses(s: &String) -> String {
        s.trim_start_matches("【").trim_end_matches("】").to_owned()
    }

    /// Read tags and topic from the title, and then return a title restult structure.
    ///
    /// # Example
    /// ```rust
    /// let title = "【经管学院】【历史社】SIT历史社联合社团特色活动";
    /// let result = Title::read(title);
    ///
    /// assert_eq!(result.tags, vec!["经管学院", "历史社"]);
    /// assert_eq!(result.topic, "SIT历史社联合社团特色活动");
    /// ```
    pub fn read(title: &str) -> TitleResult {
        let sections = Self::split(title);

        // Split tags and the topic.
        if let Some((topic, tags)) = sections.split_last() {
            let tags = tags
                .into_iter()
                .map(|x| Self::remove_parentheses(x))
                .collect();
            TitleResult {
                origin: title.to_string(),
                tags,
                topic: vec![topic.to_string()],
            }
        } else {
            TitleResult {
                origin: title.to_string(),
                ..TitleResult::default()
            }
        }
    }
}

impl TitleResult {
    /// Wrap line if the topic is longer than we expected.
    /// It uses jieba-rs to avoid abnormal separate.
    pub fn wrap_topic(mut self, jieba: Jieba, max_chars: usize) -> Self {
        if self.topic.len() == 0 {
            return self;
        }
        if self.topic[0].chars().count() <= max_chars {
            return self;
        }
        // Note: topic.len() is more than zero.
        let original_title = self.topic[0].clone();
        self.topic = Vec::new();

        // Use jieba to split words of the title.
        let words = jieba.cut(&original_title, true);
        let mut current = String::new();

        // TODO: Get a better performance
        for word in words.into_iter() {
            if word.chars().count() + current.chars().count() < max_chars {
                current.push_str(word);
            } else {
                self.topic.push(current);
                current = String::from(word);
            }
        }
        self
    }
}
