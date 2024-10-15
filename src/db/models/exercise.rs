use diesel::prelude::*;
use crate::db::models::musculature::Musculature;

#[derive(
    Debug, Identifiable, AsChangeset, Queryable, Selectable, Associations, PartialEq, Clone,
)]
#[diesel(table_name = crate::schema::exercises)]
#[diesel(belongs_to(Musculature, foreign_key = musculature_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Exercise {
    pub id: i32,
    pub name: String,
    pub impact: String,
    pub level: String,
    pub description: String,
    pub video: Vec<Option<String>>,
    pub male_weight: String,
    pub female_weight: String,
    pub musculature_id: i32,
}

impl Exercise {
    pub fn format_for_discord(&self) -> String {
        let video_links: String = self
            .video
            .iter()
            .filter_map(|v| v.clone())
            .collect::<Vec<String>>()
            .join(", ");

        format!(
            "## {}\n\
            ***Impact:*** {}\n\
            ***Level:*** {}\n\
            ***Description:*** \n{}\n\
            ***Video Links:*** {}\n\
            ***Male Weight:*** {}\n\
            ***Female Weight:*** {}\n",
            self.name,
            self.impact,
            self.level,
            self.description,
            if video_links.is_empty() {
                "No videos".to_string()
            } else {
                video_links
            },
            self.male_weight,
            self.female_weight
        )
    }
}
