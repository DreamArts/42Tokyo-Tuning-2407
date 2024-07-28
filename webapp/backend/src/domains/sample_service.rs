use futures::try_join;
use rayon::prelude::*;
use tokio::sync::Mutex;

pub async fn get_nearest_available_tow_trucks(
    &self,
    order_id: i32,
) -> Result<Option<TowTruckDto>, AppError> {
    let order_future = self.order_repository.find_order_by_id(order_id);
    let order = order_future.await?;

    let (area_id, tow_trucks, (nodes, edges)) = try_join!(
        self.map_repository.get_area_id_by_node_id(order.node_id),
        self.tow_truck_repository.get_paginated_tow_trucks(0, -1, Some("available".to_string()), Some(area_id)),
        async {
            let nodes_future = self.map_repository.get_all_nodes(Some(area_id));
            let edges_future = self.map_repository.get_all_edges(Some(area_id));
            try_join!(nodes_future, edges_future)
        }
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
    let sorted_tow_trucks_by_distance: Vec<_> = tow_trucks
        .into_par_iter()
        .map(|truck| {
            let distance = calculate_distance(&graph, truck.node_id, order.node_id);
            (distance, truck)
        })
        .collect();

    let sorted_tow_trucks_by_distance: Vec<_> = sorted_tow_trucks_by_distance
        .into_par_iter()
        .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .collect();

    if sorted_tow_trucks_by_distance.is_empty() || sorted_tow_trucks_by_distance[0].0 > 10_000_000 {
        return Ok(None);
    }

    let sorted_tow_truck_dtos: Vec<TowTruckDto> = sorted_tow_trucks_by_distance
        .into_iter()
        .map(|(_, truck)| TowTruckDto::from_entity(truck))
        .collect();

    Ok(sorted_tow_truck_dtos.first().cloned())
}
