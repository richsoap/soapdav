#[derive(Debug, Clone, Default)]
/// 漫画作品元数据结构
pub struct ComicMetadata {
    /// 作品唯一标识
    pub id: u32,
    /// 完整名称
    pub title: String,
    /// 作者姓名
    pub author: String,
    /// 标签
    pub labels: Vec<String>,
    /// 总页数
    pub pages: u16,
    /// 资源地址
    pub url: String,
    /// 来源网站标识
    pub source: String,
    /// 资源上传者
    pub uploader: String,
    /// 作品分类
    pub category: String,
    /// 本地化信息
    pub language: String,
    /// 评分
    pub score: u16,
}

impl ComicMetadata {
    /// 创建带刷新支持的实例
    fn new(
        id: u32,
        name: &str,
        author: &str,
        labels: Vec<String>,
        pages: u16,
        url: &str,
        source: &str,
        uploader: &str,
        category: &str,
        language: &str,
        score: u16,
    ) -> Self {
        Self {
            id,
            title: name.to_string(),
            author: author.to_string(),
            labels,
            pages,
            url: url.to_string(),
            source: source.to_string(),
            uploader: uploader.to_string(),
            category: category.to_string(),
            language: language.to_string(),
            score,
        }
    }

    /// 更新支持刷新的URL
    fn update_url(&mut self, new_url: &str) {
        self.url = new_url.to_string();
    }

    /// 添加新的标签
    fn add_label(&mut self, new_label: &str) {
        self.labels.push(new_label.to_string());
    }

    /// 移除指定标签
    fn remove_label(&mut self, target_label: &str) -> bool {
        if let Some(index) = self.labels.iter().position(|x| x == target_label) {
            self.labels.remove(index);
            true
        } else {
            false
        }
    }
    /// 修改评分
    fn set_score(&mut self, score: u16) {
        self.score = score
    }
}