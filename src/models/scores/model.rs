use crate::{db, models::model::Model};

use std::ops::Range;

use {serde::Serialize, sqlx::FromRow};

#[derive(Clone, Debug, FromRow, Serialize)]
pub struct Score {
    pub id: Option<i64>,
    pub variation: i64,

    pub steam_id: i64,

    pub score: i32,
    pub progress: i32,
}

#[derive(Debug, FromRow)]
struct RankedScore {
    rank: i64,
    #[sqlx(flatten)]
    score: Score,
}

impl Score {
    async fn find_neighbourhood(
        steam_id: i64,
        variation: i64,
        range: Range<i64>,
    ) -> sqlx::Result<Vec<Self>> {
        let scores = sqlx::query_as::<_, RankedScore>(
            r#"
                SELECT 
                    RANK() OVER (ORDER BY score, progress, steam_id) rank, -- Rank
                    id, variation, steam_id, score, progress               -- Score
                FROM scores
                WHERE variation = $1
            "#,
        )
        .bind(variation)
        .fetch_all(&*db::POOL)
        .await?;

        let user_rank = scores
            .iter()
            .find_map(|RankedScore { rank, score }| (score.steam_id == steam_id).then(|| rank))
            .unwrap();

        let neighbourhood = scores
            .iter()
            .filter_map(|RankedScore { rank, score }| {
                (range.contains(&(user_rank - rank))).then(|| score)
            })
            .cloned()
            .collect::<Vec<_>>();

        Ok(neighbourhood)
    }
}

#[async_trait::async_trait]
impl Model for Score {
    type Id = i64;

    async fn find(id: Self::Id) -> sqlx::Result<Self> {
        sqlx::query_as! {
            Score,
            "SELECT * FROM scores WHERE id = $1",
            id
        }
        .fetch_one(&*db::POOL)
        .await
    }

    async fn find_batch(ids: &[Self::Id]) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as! {
            Score,
            "SELECT * FROM scores WHERE id IN (SELECT * FROM UNNEST($1::bigint[]))",
            ids
        }
        .fetch_all(&*db::POOL)
        .await
    }

    async fn find_all() -> sqlx::Result<Vec<Self>> {
        sqlx::query_as! {
            Score,
            "SELECT * FROM scores"
        }
        .fetch_all(&*db::POOL)
        .await
    }

    async fn insert(value: &Self) -> sqlx::Result<Option<Self::Id>> {
        let exists: Option<Score> = sqlx::query_as! {
            Score,
            "SELECT * FROM scores WHERE steam_id = $1 AND variation = $2",
            value.steam_id, value.variation
        }
        .fetch_optional(&*db::POOL)
        .await?;

        // A bit ugly but at least it's readable.
        // There might be a way to do all of this from within SQL using ON CONFLICT causes but I
        // couldn't get it to work.
        let upserted_id = if let Some(high_score) = exists {
            if value.score > high_score.score
                || (value.score == high_score.score && value.progress > high_score.progress)
            {
                let id = sqlx::query! {
                    r#"
                        UPDATE scores
                        SET
                            score = $1,
                            progress = $2
                        WHERE
                            steam_id = $3
                            AND variation = $4
                        RETURNING id
                    "#,
                    value.score, value.progress, value.steam_id, value.variation
                }
                .fetch_one(&*db::POOL)
                .await?
                .id;

                Some(id)
            } else {
                None
            }
        } else {
            let id = sqlx::query! {
                r#"
                    INSERT INTO scores(variation, steam_id, score, progress)
                    VALUES ($1, $2, $3, $4)
                    RETURNING id
                "#,
                value.variation, value.steam_id, value.score, value.progress
            }
            .fetch_one(&*db::POOL)
            .await?
            .id;

            Some(id)
        };

        Ok(upserted_id)
    }

    async fn delete(id: Self::Id) -> sqlx::Result<()> {
        sqlx::query! {
            "DELETE FROM scores WHERE id = $1",
            id
        }
        .execute(&*db::POOL)
        .await?;

        Ok(())
    }
}
