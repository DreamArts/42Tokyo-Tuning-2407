use chrono::{DateTime, Utc};
use futures::future::join_all;

use super::{
    auth_service::AuthRepository,
    dto::order::{CompletedOrderDto, OrderDto},
    map_service::MapRepository,
    tow_truck_service::TowTruckRepository,
};
use crate::{
    errors::AppError,
    models::order::{CompletedOrder, Order},
};

pub trait OrderRepository {
    async fn find_order_by_id(&self, id: i32) -> Result<Order, AppError>;
    async fn update_order_status(&self, order_id: i32, status: &str) -> Result<(), AppError>;
    async fn get_paginated_orders(
        &self,
        page: i32,
        page_size: i32,
        sort_by: Option<String>,
        sort_order: Option<String>,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<Order>, AppError>;
    async fn create_order(
        &self,
        customer_id: i32,
        node_id: i32,
        car_value: f64,
    ) -> Result<(), AppError>;
    async fn update_order_dispatched(
        &self,
        id: i32,
        dispatcher_id: i32,
        tow_truck_id: i32,
    ) -> Result<(), AppError>;
    async fn create_completed_order(
        &self,
        order_id: i32,
        tow_truck_id: i32,
        completed_time: DateTime<Utc>,
    ) -> Result<(), AppError>;
    async fn get_all_completed_orders(&self) -> Result<Vec<CompletedOrder>, AppError>;
}

#[derive(Debug)]
pub struct OrderService<
    T: OrderRepository + std::fmt::Debug,
    U: TowTruckRepository + std::fmt::Debug,
    V: AuthRepository + std::fmt::Debug,
    W: MapRepository + std::fmt::Debug,
> {
    order_repository: T,
    tow_truck_repository: U,
    auth_repository: V,
    map_repository: W,
}

impl<
        T: OrderRepository + std::fmt::Debug,
        U: TowTruckRepository + std::fmt::Debug,
        V: AuthRepository + std::fmt::Debug,
        W: MapRepository + std::fmt::Debug,
    > OrderService<T, U, V, W>
{
    pub fn new(
        order_repository: T,
        tow_truck_repository: U,
        auth_repository: V,
        map_repository: W,
    ) -> Self {
        OrderService {
            order_repository,
            tow_truck_repository,
            auth_repository,
            map_repository,
        }
    }

    pub async fn update_order_status(&self, order_id: i32, status: &str) -> Result<(), AppError> {
        self.order_repository
            .update_order_status(order_id, status)
            .await
    }

    pub async fn get_order_by_id(&self, id: i32) -> Result<OrderDto, AppError> {
        let order = self.order_repository.find_order_by_id(id).await?;

        let client_username = self
            .auth_repository
            .find_user_by_id(order.client_id)
            .await
            .unwrap()
            .unwrap()
            .username;

        let dispatcher = match order.dispatcher_id {
            Some(dispatcher_id) => self
                .auth_repository
                .find_dispatcher_by_id(dispatcher_id)
                .await
                .unwrap(),
            None => None,
        };

        let (dispatcher_user_id, dispatcher_username) = match dispatcher {
            Some(dispatcher) => (
                Some(dispatcher.user_id),
                Some(
                    self.auth_repository
                        .find_user_by_id(dispatcher.user_id)
                        .await
                        .unwrap()
                        .unwrap()
                        .username,
                ),
            ),
            None => (None, None),
        };

        let tow_truck = match order.tow_truck_id {
            Some(tow_truck_id) => self
                .tow_truck_repository
                .find_tow_truck_by_id(tow_truck_id)
                .await
                .unwrap(),
            None => None,
        };

        let (driver_user_id, driver_username) = match tow_truck {
            Some(tow_truck) => (
                Some(tow_truck.driver_id),
                Some(
                    self.auth_repository
                        .find_user_by_id(tow_truck.driver_id)
                        .await
                        .unwrap()
                        .unwrap()
                        .username,
                ),
            ),
            None => (None, None),
        };

        let area_id = self
            .map_repository
            .get_area_id_by_node_id(order.node_id)
            .await
            .unwrap();

        Ok(OrderDto {
            id: order.id,
            client_id: order.client_id,
            client_username: Some(client_username),
            dispatcher_user_id,
            dispatcher_username,
            driver_user_id,
            driver_username,
            area_id,
            dispatcher_id: order.dispatcher_id,
            tow_truck_id: order.tow_truck_id,
            status: order.status,
            node_id: order.node_id,
            car_value: order.car_value,
            order_time: order.order_time,
            completed_time: order.completed_time,
        })
    }

    pub async fn get_paginated_orders(
        &self,
        page: i32,
        page_size: i32,
        sort_by: Option<String>,
        sort_order: Option<String>,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<OrderDto>, AppError> {
        let orders = self
            .order_repository
            .get_paginated_orders(page, page_size, sort_by, sort_order, status, area)
            .await?;

        let client_usernames_futures = orders
            .iter()
            .map(|order| self.auth_repository.find_user_by_id(order.client_id));
        let client_usernames_results = join_all(client_usernames_futures).await;

        let dispatchers_futures = orders.iter().map(|order| {
            async {
                match order.dispatcher_id {
                    Some(dispatcher_id) => self.auth_repository.find_dispatcher_by_id(dispatcher_id).await,
                    None => Ok(None),
                }
            }
        });
        let dispatchers_results = join_all(dispatchers_futures).await;
        
        let tow_trucks_futures = orders.iter().map(|order| {
            async {
                match order.tow_truck_id {
                    Some(tow_truck_id) => self.tow_truck_repository.find_tow_truck_by_id(tow_truck_id).await,
                    None => Ok(None),
                }
            }
        });
        let tow_trucks_results = join_all(tow_trucks_futures).await;

        let dispatcher_usernames_futures = dispatchers_results.iter().map(|dispatcher_result| {
            let auth_repository = &self.auth_repository;
            async move {
                match dispatcher_result {
                    Ok(Some(dispatcher)) => auth_repository.find_user_by_id(dispatcher.user_id).await,
                    _ => Ok(None),
                }
            }
        });
        let dispatcher_usernames_results = join_all(dispatcher_usernames_futures).await;

        let driver_usernames_futures = tow_trucks_results.iter().map(|tow_truck_result| {
            let auth_repository = &self.auth_repository;
            async move {
                match tow_truck_result {
                    Ok(Some(tow_truck)) => auth_repository.find_user_by_id(tow_truck.driver_id).await,
                    _ => Ok(None),
                }
            }
        });
        let driver_usernames_results = join_all(driver_usernames_futures).await;

        let area_ids_futures = orders
            .iter()
            .map(|order| self.map_repository.get_area_id_by_node_id(order.node_id));
        let area_ids_results = join_all(area_ids_futures).await;

        let results: Vec<OrderDto> = orders
            .into_iter()
            .enumerate()
            .map(|(i, order)| {
                let client_username = client_usernames_results[i]
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .username
                    .clone();
                let dispatcher = dispatchers_results[i].as_ref().ok().and_then(|d| d.as_ref());
                let (dispatcher_user_id, dispatcher_username) = match dispatcher {
                    Some(dispatcher) => {
                        let username = dispatcher_usernames_results[i]
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap()
                            .username
                            .clone();
                        (Some(dispatcher.user_id), Some(username))
                    }
                    None => (None, None),
                };
                let tow_truck = tow_trucks_results[i].as_ref().ok().and_then(|t| t.as_ref());
                let (driver_user_id, driver_username) = match tow_truck {
                    Some(tow_truck) => {
                        let username = driver_usernames_results[i]
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap()
                            .username
                            .clone();
                        (Some(tow_truck.driver_id), Some(username))
                    }
                    None => (None, None),
                };
                let area_id = area_ids_results[i].as_ref().unwrap().clone();

                OrderDto {
                    id: order.id,
                    client_id: order.client_id,
                    client_username: Some(client_username),
                    dispatcher_id: order.dispatcher_id,
                    dispatcher_user_id,
                    dispatcher_username,
                    tow_truck_id: order.tow_truck_id,
                    driver_user_id,
                    driver_username,
                    area_id,
                    status: order.status,
                    node_id: order.node_id,
                    car_value: order.car_value,
                    order_time: order.order_time,
                    completed_time: order.completed_time,
                }
            })
            .collect();

        Ok(results)
    }

    pub async fn create_client_order(
        &self,
        client_id: i32,
        node_id: i32,
        car_value: f64,
    ) -> Result<(), AppError> {
        match self
            .order_repository
            .create_order(client_id, node_id, car_value)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::BadRequest),
        }
    }

    pub async fn create_dispatcher_order(
        &self,
        order_id: i32,
        dispatcher_id: i32,
        tow_truck_id: i32,
        order_time: DateTime<Utc>,
    ) -> Result<(), AppError> {
        if (self
            .order_repository
            .create_completed_order(order_id, tow_truck_id, order_time)
            .await)
            .is_err()
        {
            return Err(AppError::BadRequest);
        }

        let update_order = self.order_repository.update_order_dispatched(order_id, dispatcher_id, tow_truck_id);
        let update_tow_truck = self.tow_truck_repository.update_status(tow_truck_id, "busy");
        tokio::try_join!(update_order, update_tow_truck)?;

        Ok(())
    }

    pub async fn get_completed_orders(&self) -> Result<Vec<CompletedOrderDto>, AppError> {
        let orders = self.order_repository.get_all_completed_orders().await?;
        let order_dtos = orders
            .into_iter()
            .map(CompletedOrderDto::from_entity)
            .collect();

        Ok(order_dtos)
    }
}
