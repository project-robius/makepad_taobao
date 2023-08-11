#[derive(Debug, Clone)]
pub struct CatalogDataItem {
    pub title: String,
    pub subtitle: String,
    pub price: String,
}

pub struct CatalogData {
    pub items: Vec<CatalogDataItem>,
}

impl CatalogData {
    pub fn new() -> Self {
        Self {
            items: catalog_data(),
        }
    }
}

fn catalog_data() -> Vec<CatalogDataItem> {
    vec![
        CatalogDataItem {
            title: "男士人字拖 2023".to_string(),
            subtitle: "500+人付费".to_string(),
            price: "58".to_string(),
        },
        CatalogDataItem {
            title: "巧克力大地色调".to_string(),
            subtitle: "10000+人付费".to_string(),
            price: "8.9".to_string(),
        },
        CatalogDataItem {
            title: "冰丝防沙发垫夏季垫t".to_string(),
            subtitle: "50+人付费".to_string(),
            price: "20.9".to_string(),
        },
        CatalogDataItem {
            title: "胡萝卜奶锅婴儿不粘锅".to_string(),
            subtitle: "100+人付费".to_string(),
            price: "89".to_string(),
        },
        CatalogDataItem {
            title: "脆皮芝士香蕉味网红".to_string(),
            subtitle: "200+人付费".to_string(),
            price: "18.5".to_string(),
        },
        CatalogDataItem {
            title: "德国浓缩乳清蛋白y".to_string(),
            subtitle: "200+人付费".to_string(),
            price: "20.3".to_string(),
        },
        CatalogDataItem {
            title: "不可思议的戒指".to_string(),
            subtitle: "500+人付费".to_string(),
            price: "100".to_string(),
        },
        CatalogDataItem {
            title: "路由器AX6000".to_string(),
            subtitle: "13人付费".to_string(),
            price: "266".to_string(),
        },
        CatalogDataItem {
            title: "男士人字拖 2023".to_string(),
            subtitle: "500+人付费".to_string(),
            price: "58".to_string(),
        },
        CatalogDataItem {
            title: "巧克力大地色调".to_string(),
            subtitle: "10000+人付费".to_string(),
            price: "8.9".to_string(),
        },
        CatalogDataItem {
            title: "冰丝防沙发垫夏季垫t".to_string(),
            subtitle: "50+人付费".to_string(),
            price: "20.9".to_string(),
        },
        CatalogDataItem {
            title: "胡萝卜奶锅婴儿不粘锅".to_string(),
            subtitle: "100+人付费".to_string(),
            price: "89".to_string(),
        },
        CatalogDataItem {
            title: "脆皮芝士香蕉味网红".to_string(),
            subtitle: "200+人付费".to_string(),
            price: "18.5".to_string(),
        },
        CatalogDataItem {
            title: "德国浓缩乳清蛋白".to_string(),
            subtitle: "200+人付费".to_string(),
            price: "20.3".to_string(),
        },
    ]
}

