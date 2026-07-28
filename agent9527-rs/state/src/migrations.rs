use std::borrow::Cow;

use sqlx::AssertSqlSafe;
use sqlx::SqlSafeStr;
use sqlx::SqlitePool;
use sqlx::migrate::Migration;
use sqlx::migrate::Migrator;

pub(crate) static STATE_MIGRATOR: Migrator = sqlx::migrate!("./migrations");
pub(crate) static LOGS_MIGRATOR: Migrator = sqlx::migrate!("./logs_migrations");
pub(crate) static GOALS_MIGRATOR: Migrator = sqlx::migrate!("./goals_migrations");
pub(crate) static MEMORIES_MIGRATOR: Migrator = sqlx::migrate!("./memory_migrations");
pub(crate) static THREAD_HISTORY_MIGRATOR: Migrator = sqlx::migrate!("./thread_history_migrations");

/// Allow an older Agent9527 binary to open a database that has already been
/// migrated by a newer binary running in parallel.
///
/// We intentionally ignore applied migration versions that are newer than the
/// embedded migration set. Known migration versions are still validated by
/// checksum, so this only relaxes the "database is ahead of me" case.
fn runtime_migrator(base: &'static Migrator) -> Migrator {
    Migrator {
        migrations: Cow::Borrowed(base.migrations.as_ref()),
        ignore_missing: true,
        locking: base.locking,
        no_tx: base.no_tx,
        table_name: base.table_name.clone(),
        create_schemas: base.create_schemas.clone(),
    }
}

pub(crate) fn runtime_state_migrator() -> Migrator {
    runtime_migrator(&STATE_MIGRATOR)
}

pub(crate) fn runtime_logs_migrator() -> Migrator {
    runtime_migrator(&LOGS_MIGRATOR)
}

pub(crate) fn runtime_goals_migrator() -> Migrator {
    runtime_migrator(&GOALS_MIGRATOR)
}

pub(crate) fn runtime_memories_migrator() -> Migrator {
    runtime_migrator(&MEMORIES_MIGRATOR)
}

// The paginated history projector will call this when it takes ownership of opening the database.
#[allow(dead_code)]
pub(crate) fn runtime_thread_history_migrator() -> Migrator {
    runtime_migrator(&THREAD_HISTORY_MIGRATOR)
}

/// Repair migration checksums written by the Windows release that embedded
/// CRLF-normalized SQL files. Only exact CRLF equivalents of the current
/// migration are accepted, so genuinely modified migrations still fail the
/// normal sqlx checksum validation.
pub(crate) async fn repair_crlf_migration_checksums(
    pool: &SqlitePool,
    migrator: &Migrator,
) -> anyhow::Result<()> {
    if !migrations_table_exists(pool).await? {
        return Ok(());
    }

    let applied_migrations = sqlx::query_as::<_, (i64, Vec<u8>)>(
        "SELECT version, checksum FROM _sqlx_migrations WHERE success = TRUE",
    )
    .fetch_all(pool)
    .await?;

    for migration in migrator.migrations.iter() {
        let Some((_, applied_checksum)) = applied_migrations
            .iter()
            .find(|(version, _)| *version == migration.version)
        else {
            continue;
        };
        if applied_checksum.as_slice() == migration.checksum.as_ref() {
            continue;
        }

        let normalized_sql = migration.sql.as_str().replace("\r\n", "\n");
        let crlf_sql = normalized_sql.replace('\n', "\r\n");
        if crlf_sql == migration.sql.as_str() {
            continue;
        }
        let crlf_migration = Migration::new(
            migration.version,
            migration.description.clone(),
            migration.migration_type,
            AssertSqlSafe(crlf_sql).into_sql_str(),
            migration.no_tx,
        );
        if applied_checksum.as_slice() != crlf_migration.checksum.as_ref() {
            continue;
        }

        sqlx::query(
            r#"
UPDATE _sqlx_migrations
SET checksum = ?
WHERE version = ?
  AND success = TRUE
  AND checksum = ?
            "#,
        )
        .bind(migration.checksum.as_ref())
        .bind(migration.version)
        .bind(applied_checksum)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub(crate) async fn repair_legacy_recency_migration_version(
    pool: &SqlitePool,
    migrator: &Migrator,
) -> anyhow::Result<()> {
    let Some(recency_migration) = migrator
        .migrations
        .iter()
        .find(|migration| migration.version == 39)
    else {
        return Ok(());
    };
    if !migrations_table_exists(pool).await? {
        return Ok(());
    }

    let legacy_recency_needs_repair = sqlx::query_scalar::<_, i64>(
        r#"
SELECT 1
FROM _sqlx_migrations
WHERE version = ?
  AND checksum = ?
  AND NOT EXISTS (
      SELECT 1 FROM _sqlx_migrations WHERE version = ?
  )
        "#,
    )
    .bind(38_i64)
    .bind(recency_migration.checksum.as_ref())
    .bind(recency_migration.version)
    .fetch_optional(pool)
    .await?
    .is_some();
    if !legacy_recency_needs_repair {
        return Ok(());
    }

    sqlx::query(
        r#"
UPDATE _sqlx_migrations
SET version = ?, description = ?
WHERE version = ?
  AND checksum = ?
  AND NOT EXISTS (
      SELECT 1 FROM _sqlx_migrations WHERE version = ?
  )
        "#,
    )
    .bind(recency_migration.version)
    .bind(recency_migration.description.as_ref())
    .bind(38_i64)
    .bind(recency_migration.checksum.as_ref())
    .bind(recency_migration.version)
    .execute(pool)
    .await?;
    Ok(())
}

async fn migrations_table_exists(pool: &SqlitePool) -> anyhow::Result<bool> {
    Ok(sqlx::query_scalar::<_, i64>(
        "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = '_sqlx_migrations'",
    )
    .fetch_optional(pool)
    .await?
    .is_some())
}

#[cfg(test)]
#[path = "migrations_tests.rs"]
mod tests;
