use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::order::OrderWithData;

// Input Data Structure

#[derive(Deserialize, Debug)]
pub struct ClientOrderRequestDto {
    pub client_id: i32,
    pub node_id: i32,
    pub car_value: f64,
}

#[derive(Deserialize, Debug)]
pub struct DispatcherOrderRequestDto {
    pub order_id: i32,
    pub dispatcher_id: i32,
    pub tow_truck_id: i32,
    pub order_time: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateOrderStatusRequestDto {
    pub order_id: i32,
    pub status: String,
}

// Output Data Structure

#[derive(Serialize, Debug)]
pub struct OrderDto {
    pub id: i32,
    pub client_id: i32,
    pub client_username: Option<String>,
    pub dispatcher_id: Option<i32>,
    pub dispatcher_user_id: Option<i32>,
    pub dispatcher_username: Option<String>,
    pub tow_truck_id: Option<i32>,
    pub driver_user_id: Option<i32>,
    pub driver_username: Option<String>,
    pub status: String,
    pub node_id: i32,
    pub area_id: i32,
    pub car_value: f64,
    pub order_time: DateTime<Utc>,
    pub completed_time: Option<DateTime<Utc>>,
}

impl OrderDto {
    pub fn from_entity(entity: OrderWithData) -> Self {
        OrderDto {
            id: entity.id,
            client_id: entity.client_id,
            client_username: entity.client_username,
            dispatcher_id: entity.dispatcher_id,
            dispatcher_user_id: entity.dispatcher_user_id,
            dispatcher_username: entity.dispatcher_username,
            tow_truck_id: entity.tow_truck_id,
            driver_user_id: entity.driver_user_id,
            driver_username: entity.driver_username,
            status: entity.status,
            node_id: entity.node_id,
            area_id: entity.area_id,
            car_value: entity.car_value,
            order_time: entity.order_time,
            completed_time: entity.completed_time,
        }
    }
}
