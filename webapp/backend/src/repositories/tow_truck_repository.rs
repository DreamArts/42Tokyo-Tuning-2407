use crate::domains::tow_truck_service::TowTruckRepository;
use crate::errors::AppError;
use crate::models::tow_truck::TowTruck;
use sqlx::mysql::MySqlPool;

#[derive(Debug)]
pub struct TowTruckRepositoryImpl {
    pool: MySqlPool,
}

impl TowTruckRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        TowTruckRepositoryImpl { pool }
    }
}

impl TowTruckRepository for TowTruckRepositoryImpl {
    async fn get_paginated_tow_trucks(
        &self,
        page: i32,
        page_size: i32,
        status: Option<String>,
        area_id: Option<i32>,
    ) -> Result<Vec<TowTruck>, AppError> {
        let where_clause = match (status, area_id) {
            (Some(status), Some(area_id)) => format!(
                "WHERE tt.status = '{}' AND tt.area_id = {}",
                status, area_id
            ),
            (None, Some(area_id)) => format!("WHERE tt.area_id = {}", area_id),
            (Some(status), None) => format!("WHERE tt.status = '{}'", status),
            (None, None) => "".to_string(),
        };
        let limit_clause = match page_size {
            -1 => "".to_string(),
            _ => format!("LIMIT {}", page_size),
        };
        let offset_clause = match page_size {
            -1 => "".to_string(),
            page_size => format!("OFFSET {}", page * page_size),
        };

        let query = format!(
            "SELECT
                tt.id,
                tt.driver_id,
                u.username AS driver_username,
                tt.status,
                tt.area_id,
                tt.node_id
            FROM
                tow_trucks tt
            JOIN
                users u
            ON
                tt.driver_id = u.id
            {}
            ORDER BY
                tt.id ASC
            {}
            {}",
            where_clause, limit_clause, offset_clause
        );

        let tow_trucks = sqlx::query_as::<_, TowTruck>(&query)
            .fetch_all(&self.pool)
            .await?;

        Ok(tow_trucks)
    }

    async fn update_location(&self, tow_truck_id: i32, node_id: i32) -> Result<(), AppError> {
        sqlx::query("UPDATE tow_trucks SET node_id = ? WHERE id = ?")
            .bind(node_id)
            .bind(tow_truck_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn update_status(&self, tow_truck_id: i32, status: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE tow_trucks SET status = ? WHERE id = ?")
            .bind(status)
            .bind(tow_truck_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_tow_truck_by_id(&self, id: i32) -> Result<Option<TowTruck>, AppError> {
        let tow_truck = sqlx::query_as::<_, TowTruck>(
            "SELECT
                tt.id, tt.driver_id, u.username AS driver_username, tt.status, tt.node_id, tt.area_id
            FROM
                tow_trucks tt
            JOIN
                users u
            ON
                tt.driver_id = u.id
            WHERE
                tt.id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(tow_truck)
    }
}
