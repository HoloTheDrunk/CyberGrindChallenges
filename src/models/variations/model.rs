use crate::{db, models::model::Model};

use std::ops::Range;

use {serde::Serialize, sqlx::FromRow};

macro_rules! variation {
    ($($weapon:ident),+ $(,)?) => {
        #[derive(Debug, Default)]
        pub struct Variation {
            id: Option<i64>,
            $(
                $weapon: bool
            ),+
        }

        impl Variation {
            paste::paste!{$(
                pub fn [<use_ $weapon>](&mut self) -> &mut Self {
                    self.$weapon = true;
                    self
                }
            )+}

            pub fn try_use(&mut self, weapon: &str) -> bool {
                match weapon {
                    $(
                        stringify!($weapon) => paste::paste!(self.[<use_ $weapon>]()),
                    )+
                    _ => return false,
                };

                true
            }

            pub async fn match_weapons(variation: &Self) -> sqlx::Result<Option<i64>> {
                sqlx::query! {
                    r#"
                        SELECT id FROM variations WHERE
                            piercer = $1 AND marskman = $2 AND sharpshooter = $3
                            AND core_eject = $4 AND pump_charge = $5
                            AND attractor = $6 AND overheat = $7
                            AND electric = $8 AND malicious = $9 AND drill = $10
                            AND freezeframe = $11 AND srs_cannon = $12
                    "#,
                    $(variation.$weapon),+
                }
                .fetch_optional(&*db::POOL)
                .await
                .map(|opt| opt.map(|record| record.id))
            }
        }

        #[async_trait::async_trait]
        impl Model for Variation {
            type Id = i64;

            async fn find(id: Self::Id) -> sqlx::Result<Self> {
                sqlx::query_as! {
                    Self,
                    "SELECT * FROM variations WHERE id = $1",
                    id
                }
                .fetch_one(&*db::POOL)
                .await
            }

            async fn find_batch(ids: &[Self::Id]) -> sqlx::Result<Vec<Self>> {
                sqlx::query_as! {
                    Self,
                    "SELECT * FROM variations WHERE id IN (SELECT * FROM UNNEST($1::bigint[]))",
                    ids
                }
                .fetch_all(&*db::POOL)
                .await
            }

            async fn find_all() -> sqlx::Result<Vec<Self>> {
                sqlx::query_as! {
                    Self,
                    "SELECT * FROM variations"
                }
                .fetch_all(&*db::POOL)
                .await
            }

            async fn insert(value: &Self) -> sqlx::Result<Option<Self::Id>> {
                sqlx::query! {
                    r#"
                        INSERT INTO variations(
                            piercer, marskman, sharpshooter,
                            core_eject, pump_charge,
                            attractor, overheat,
                            electric, malicious, drill,
                            freezeframe, srs_cannon
                        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                        RETURNING id
                    "#,
                    $(value.$weapon),+
                }
                .fetch_optional(&*db::POOL)
                .await
                .map(|opt| opt.map(|record| record.id))
            }

            async fn delete(id: Self::Id) -> sqlx::Result<()> {
                sqlx::query! {
                    "DELETE FROM variations WHERE id = $1",
                    id
                }
                .execute(&*db::POOL)
                .await?;

                Ok(())
            }
        }
    };
}

// Add variations to the `Model::insert` implementation above when new ones are released. Could be
// done automatically with a derive macro but ain't nobody got time for that (and the maintenance
// cost of this "dirty" solution is much lower).
// Future me, don't try to be a smartass by changing it.
variation! {
    // (Slab) Pistol
    piercer,
    marskman,
    sharpshooter,

    // Shotgun
    core_eject,
    pump_charge,

    // Nailgun / Sawblade Launcher
    attractor,
    overheat,

    // Railcannon
    electric,
    malicious,
    drill,

    // Rocket Launcher
    freezeframe,
    srs_cannon,
}
