use ::chrono::SubsecRound;
use sja::structs::{Angebot, Organisation};
use sqlx::types::chrono;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

#[test]
fn env() {
    dotenvy::from_path_override("./.env.test").unwrap();
    assert_eq!(
        env::var("DATABASE_URL").unwrap(),
        "postgresql://test_admin:adminpassword@localhost:5432/"
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn migration(pool: PgPool) -> sqlx::Result<()> {
    let expected_tables = [
        String::from("organisation"),
        String::from("angebot"),
        String::from("angebot_adresse"),
        String::from("adresse"),
        String::from("angebot_apartner"),
        String::from("ansprech_partner"),
        String::from("apartner_email"),
        String::from("email"),
        String::from("apartner_tnummer"),
        String::from("telefonnummer"),
    ];
    let tables =
        sqlx::query!("SELECT tablename FROM pg_catalog.pg_tables WHERE schemaname = 'public';")
            .fetch_all(&pool)
            .await?;
    let table_names: Vec<String> = tables
        .into_iter()
        .filter_map(|record| record.tablename)
        .collect();
    for expected_name in expected_tables {
        assert!(table_names.contains(&expected_name));
    }
    Ok(())
}

#[sqlx::test]
async fn organisation(pool: PgPool) -> sqlx::Result<()> {
    let orga: Organisation = Organisation {
        organisation_id: Uuid::new_v4(),
        organisation_name: String::from("Test Organisation"),
    };
    sqlx::query!(
        "INSERT INTO organisation(organisation_id, organisation_name) VALUES ($1, $2)",
        &orga.organisation_id,
        &orga.organisation_name,
    )
    .execute(&pool)
    .await?;
    let organisation = sqlx::query_as!(Organisation, "SELECT * FROM organisation")
        .fetch_one(&pool)
        .await?;

    assert_eq!(organisation.organisation_id, orga.organisation_id);

    assert_eq!(organisation.organisation_name, orga.organisation_name);

    Ok(())
}

#[sqlx::test]
async fn angebot(pool: PgPool) -> sqlx::Result<()> {
    let orga: Organisation = Organisation {
        organisation_id: Uuid::new_v4(),
        organisation_name: String::from("Test Organisation"),
    };
    sqlx::query!(
        "INSERT INTO organisation(organisation_id, organisation_name) VALUES ($1, $2)",
        &orga.organisation_id,
        &orga.organisation_name,
    )
    .execute(&pool)
    .await?;
    let organisation = sqlx::query_as!(Organisation, "SELECT * FROM organisation")
        .fetch_one(&pool)
        .await?;
    let ange: Angebot = Angebot {
        angebot_id: Uuid::new_v4(),
        angebot_name: String::from("Test Angebot"),
        beschreibung: Some(String::from(
            "Das ist die Beschreibung für das Test Angebot",
        )),
        organisation_id: organisation.organisation_id,
        created: chrono::Local::now().round_subsecs(6),
        last_modified: chrono::Local::now().round_subsecs(6),
    };
    sqlx::query!(
        "INSERT INTO angebot(angebot_id, angebot_name, beschreibung, organisation_id, created, last_modified) VALUES ($1, $2, $3, $4, $5, $6)",
        &ange.angebot_id,
        &ange.angebot_name,
        &ange.beschreibung.clone().unwrap(),
        &ange.organisation_id,
        &ange.created,
        &ange.last_modified,
    )
    .execute(&pool)
    .await?;
    let angebot = sqlx::query_as!(Angebot, "SELECT * FROM angebot")
        .fetch_one(&pool)
        .await?;

    assert_eq!(angebot.angebot_id, ange.angebot_id);
    assert_eq!(angebot.angebot_name, ange.angebot_name);
    assert_eq!(angebot.beschreibung, ange.beschreibung);
    assert_eq!(angebot.organisation_id, ange.organisation_id);
    assert_eq!(angebot.created, ange.created);
    assert_eq!(angebot.last_modified, ange.last_modified);

    Ok(())
}
