use crate::database::structs::{
    Adresse, Angebot, Ansprechpartner, JsonAngebot, JsonOrganisation, Link, Organisation, Sonstiges,
};
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
        let organisation = select_organisation_for_angebot(pool, angebot.organisation_id).await?;
        let adressen = select_adressen_for_angebot(pool, angebot.angebot_id).await?;
        let links = select_links_for_angebot(pool, angebot.angebot_id).await?;
        let apartner = select_apartner_for_angebot(pool, angebot.angebot_id).await?;
        let sonstiges = select_sonstiges_for_angebot(pool, angebot.angebot_id).await?;

        json_angebote.push(JsonAngebot {
            angebot,
            organisation,
            adressen,
            links,
            apartner,
            sonstiges,
        })
    }

    Ok(json_angebote)
}

async fn select_organisation_for_angebot(
    pool: &Pool<Postgres>,
    organisation_id: Uuid,
) -> anyhow::Result<JsonOrganisation> {
    let organisation = sqlx::query_as!(
        Organisation,
        r#"
        SELECT
          *
        FROM
          organisation
        WHERE
          organisation_id = $1"#,
        organisation_id
    )
    .fetch_one(pool)
    .await
    .with_context(|| {
        format!(
            "Error fetching Organisation for Angebot: {} from database",
            organisation_id
        )
    })?;
    let adressen = select_adressen_for_organisation(pool, organisation.organisation_id).await?;
    let links = select_links_for_organisation(pool, organisation.organisation_id).await?;
    let apartner = select_apartner_for_organisation(pool, organisation.organisation_id).await?;
    let angebote = select_angebote_for_organisation(pool, organisation.organisation_id).await?;

    Ok(JsonOrganisation {
        organisation,
        adressen,
        links,
        apartner,
        angebote,
    })
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
    sqlx::query_as!(
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
    })
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

async fn select_adressen_for_organisation(
    pool: &Pool<Postgres>,
    organisation_id: Uuid,
) -> anyhow::Result<Vec<Adresse>> {
    sqlx::query_as!(
        Adresse,
        r#"
        SELECT
          adresse.*
        FROM
          adresse
          JOIN organisation_adresse ON adresse.adresse_id = organisation_adresse.adresse_id
        WHERE
          organisation_id = $1;"#,
        organisation_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| {
        format!(
            "Error fetching Adressen for Organisation: {} from database",
            organisation_id
        )
    })
}

async fn select_links_for_organisation(
    pool: &Pool<Postgres>,
    organisation_id: Uuid,
) -> anyhow::Result<Vec<Link>> {
    sqlx::query_as!(
        Link,
        r#"
        SELECT
          link.*
        FROM
          link
          JOIN organisation_link ON link.link_id = organisation_link.link_id
        WHERE
          organisation_id = $1;"#,
        organisation_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| {
        format!(
            "Error fetching Links for Angebot: {} from database",
            organisation_id
        )
    })
}

async fn select_apartner_for_organisation(
    pool: &Pool<Postgres>,
    organisation_id: Uuid,
) -> anyhow::Result<Vec<Ansprechpartner>> {
    sqlx::query_as!(
        Ansprechpartner,
        r#"
        SELECT
          ansprechpartner.*
        FROM
          ansprechpartner
          JOIN organisation_apartner ON ansprechpartner.ansprechpartner_id = organisation_apartner.ansprechpartner_id
        WHERE
          organisation_id = $1;"#,
        organisation_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| {
        format!(
            "Error fetching Ansprechpartner for Organisation: {} from database",
            organisation_id
        )
    })
}

async fn select_angebote_for_organisation(
    pool: &Pool<Postgres>,
    organisation_id: Uuid,
) -> anyhow::Result<Vec<Angebot>> {
    sqlx::query_as!(
        Angebot,
        r#"
        SELECT
          angebot.*
        FROM
          angebot
        join organisation on organisation.organisation_id = angebot.organisation_id
        WHERE
          angebot.organisation_id = $1;"#,
        organisation_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| {
        format!(
            "Error fetching Angebote for Organisation: {} from database",
            organisation_id
        )
    })
}
