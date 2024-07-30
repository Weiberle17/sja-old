use crate::database::structs::{Adresse, Angebot, Ansprechpartner, JsonAngebot, Link, Sonstiges};
use anyhow::Context;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn select_links(pool: &Pool<Postgres>) -> anyhow::Result<Vec<Link>> {
    sqlx::query_as!(
        Link,
        r#"
        SELECT
          *
        FROM
          link;"#
    )
    .fetch_all(pool)
    .await
    .context("Error fetching links from database")
}

pub async fn select_all_angebote(pool: &Pool<Postgres>) -> anyhow::Result<Vec<JsonAngebot>> {
    let angebote = sqlx::query_as!(
        Angebot,
        r#"
        SELECT
          *
        FROM
          angebot;"#
    )
    .fetch_all(pool)
    .await
    .context("Error fetching all Angebote from database")?;

    let mut json_angebote: Vec<JsonAngebot> = Vec::new();
    for angebot in angebote {
        let adressen = select_adressen_for_angebot(pool, angebot.angebot_id).await?;
        let links = select_links_for_angebot(pool, angebot.angebot_id).await?;
        let apartner = select_apartner_for_angebot(pool, angebot.angebot_id).await?;
        let sonstiges = select_sonstiges_for_angebot(pool, angebot.angebot_id).await?;

        json_angebote.push(JsonAngebot {
            angebot,
            adressen,
            links,
            apartner,
            sonstiges,
        })
    }

    Ok(json_angebote)
}

async fn select_adressen_for_angebot(
    pool: &Pool<Postgres>,
    angebot_id: Uuid,
) -> anyhow::Result<Vec<Adresse>> {
    sqlx::query_as!(
        Adresse,
        r#"
        SELECT
          adresse.*
        FROM
          adresse
          JOIN angebot_adresse ON adresse.adresse_id = angebot_adresse.adresse_id
        WHERE
          angebot_id = $1;"#,
        angebot_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| {
        format!(
            "Error fetching Adressen for Angebot: {} from database",
            angebot_id
        )
    })
}

async fn select_links_for_angebot(
    pool: &Pool<Postgres>,
    angebot_id: Uuid,
) -> anyhow::Result<Vec<Link>> {
    sqlx::query_as!(
        Link,
        r#"
        SELECT
          link.*
        FROM
          link
          JOIN angebot_link ON link.link_id = angebot_link.link_id
        WHERE
          angebot_id = $1;"#,
        angebot_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| {
        format!(
            "Error fetching Links for Angebot: {} from database",
            angebot_id
        )
    })
}

async fn select_apartner_for_angebot(
    pool: &Pool<Postgres>,
    angebot_id: Uuid,
) -> anyhow::Result<Vec<Ansprechpartner>> {
    let x = sqlx::query_as!(
        Ansprechpartner,
        r#"
        SELECT
          ansprechpartner.*
        FROM
          ansprechpartner
          JOIN angebot_apartner ON ansprechpartner.ansprechpartner_id = angebot_apartner.ansprechpartner_id
        WHERE
          angebot_id = $1;"#,
        angebot_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| {
        format!(
            "Error fetching Ansprechpartner for Angebot: {} from database",
            angebot_id
        )
    });
    dbg!(&angebot_id, &x);
    x
}

async fn select_sonstiges_for_angebot(
    pool: &Pool<Postgres>,
    angebot_id: Uuid,
) -> anyhow::Result<Vec<Sonstiges>> {
    sqlx::query_as!(
        Sonstiges,
        r#"
        SELECT
          sonstiges.*
        FROM
          sonstiges
          JOIN angebot ON sonstiges.angebot_id = angebot.angebot_id
        WHERE
          angebot.angebot_id = $1;"#,
        angebot_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| {
        format!(
            "Error fetching Sonstiges for Angebot: {} from database",
            angebot_id
        )
    })
}

// pub async fn get_organisationen(Extension(pool): Extension<Pool<Postgres>>) -> impl IntoResponse {
//     let organisationen = match sqlx::query_as!(
//         Organisation,
//         r#"
//         SELECT
//           *
//         FROM
//           organisation;
//         "#
//     )
//     .fetch_all(&pool)
//     .await
//     {
//         Ok(organisationen) => organisationen,
//         Err(e) => {
//             return (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 format!("Error fetching all Organisationen: {}", e),
//             )
//                 .into_response()
//         }
//     };
//
//     (StatusCode::OK, Json(organisationen)).into_response()
// }

// pub async fn get_ansprechpartner(Extension(pool): Extension<Pool<Postgres>>) -> impl IntoResponse {
//     let ansprechpartner = match sqlx::query_as!(
//         Ansprechpartner,
//         r#"
//         SELECT
//           *
//         FROM
//           ansprech_partner;
//         "#
//     )
//     .fetch_all(&pool)
//     .await
//     {
//         Ok(ansprechpartner) => ansprechpartner,
//         Err(e) => {
//             return (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 format!("Error fetching all Ansprechpartner: {}", e),
//             )
//                 .into_response()
//         }
//     };
//
//     (StatusCode::OK, Json(ansprechpartner)).into_response()
// }
