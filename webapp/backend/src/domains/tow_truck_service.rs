use super::dto::tow_truck::TowTruckDto;
use super::map_service::MapRepository;
use super::order_service::OrderRepository;
use crate::errors::AppError;
use crate::models::graph::Graph;
use crate::models::tow_truck::TowTruck;

use rayon::prelude::*;
use std::sync::Mutex;

pub trait TowTruckRepository {
    async fn get_paginated_tow_trucks(
        &self,
        page: i32,
        page_size: i32,
        status: Option<String>,
        area_id: Option<i32>,
    ) -> Result<Vec<TowTruck>, AppError>;
    async fn update_location(&self, truck_id: i32, node_id: i32) -> Result<(), AppError>;
    async fn update_status(&self, truck_id: i32, status: &str) -> Result<(), AppError>;
    async fn find_tow_truck_by_id(&self, id: i32) -> Result<Option<TowTruck>, AppError>;
}

#[derive(Debug)]
pub struct TowTruckService<
    T: TowTruckRepository + std::fmt::Debug,
    U: OrderRepository + std::fmt::Debug,
    V: MapRepository + std::fmt::Debug,
> {
    tow_truck_repository: T,
    order_repository: U,
    map_repository: V,
}

impl<
        T: TowTruckRepository + std::fmt::Debug,
        U: OrderRepository + std::fmt::Debug,
        V: MapRepository + std::fmt::Debug,
    > TowTruckService<T, U, V>
{
    pub fn new(tow_truck_repository: T, order_repository: U, map_repository: V) -> Self {
        TowTruckService {
            tow_truck_repository,
            order_repository,
            map_repository,
        }
    }

    pub async fn get_tow_truck_by_id(&self, id: i32) -> Result<Option<TowTruckDto>, AppError> {
        let tow_truck = self.tow_truck_repository.find_tow_truck_by_id(id).await?;
        Ok(tow_truck.map(TowTruckDto::from_entity))
    }

    pub async fn get_all_tow_trucks(
        &self,
        page: i32,
        page_size: i32,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<TowTruckDto>, AppError> {
        let tow_trucks = self
            .tow_truck_repository
            .get_paginated_tow_trucks(page, page_size, status, area)
            .await?;
        let tow_truck_dtos = tow_trucks
            .into_iter()
            .map(TowTruckDto::from_entity)
            .collect();

        Ok(tow_truck_dtos)
    }

    pub async fn update_location(&self, truck_id: i32, node_id: i32) -> Result<(), AppError> {
        self.tow_truck_repository
            .update_location(truck_id, node_id)
            .await?;

        Ok(())
    }

    pub async fn get_nearest_available_tow_trucks(
        &self,
        order_id: i32,
    ) -> Result<Option<TowTruckDto>, AppError> {
        let order = self.order_repository.find_order_by_id(order_id).await?;
        let area_id = self
            .map_repository
            .get_area_id_by_node_id(order.node_id)
            .await?;
        let tow_trucks = self
            .tow_truck_repository
            .get_paginated_tow_trucks(0, -1, Some("available".to_string()), Some(area_id))
            .await?;

        let (nodes, edges) = tokio::try_join!(
            self.map_repository.get_all_nodes(Some(area_id)),
            self.map_repository.get_all_edges(Some(area_id))
        )?;

        let mut graph = Graph::new();

        // NodeとEdgeの追加を並列で行う
        {
            let graph_mutex = Mutex::new(&mut graph);

            nodes.par_iter().for_each(|node| {
                let mut graph = graph_mutex.lock().unwrap();
                graph.add_node(node.clone());
            });

            edges.par_iter().for_each(|edge| {
                let mut graph = graph_mutex.lock().unwrap();
                graph.add_edge(edge.clone());
            });
        }

        // 距離計算とソートを並列で行う
        let mut tow_trucks_with_distance: Vec<_> = tow_trucks
            .into_par_iter()
            .map(|truck| {
                let distance = calculate_distance(&graph, truck.node_id, order.node_id);
                (distance, truck)
            })
            .collect();

        // 並列ソート
        tow_trucks_with_distance.par_sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        if tow_trucks_with_distance.is_empty() || tow_trucks_with_distance[0].0 > 10_000_000 {
            return Ok(None);
        }

        let sorted_tow_truck_dtos: Vec<TowTruckDto> = tow_trucks_with_distance
            .into_iter()
            .map(|(_, truck)| TowTruckDto::from_entity(truck))
            .collect();

        Ok(sorted_tow_truck_dtos.first().cloned())
    }
}

fn calculate_distance(graph: &Graph, node_id_1: i32, node_id_2: i32) -> i32 {
    graph.shortest_path(node_id_1, node_id_2)
}
