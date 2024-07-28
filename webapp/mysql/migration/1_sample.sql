CALL DropIndexIfExists('areas', 'idx_area_name_ft');
CREATE FULLTEXT INDEX idx_area_name_ft ON areas(name) WITH PARSER ngram;

CALL DropIndexIfExists('users', 'idx_users_username_ft');
CREATE FULLTEXT INDEX idx_users_username_ft ON users(username);

CALL DropIndexIfExists('users', 'idx_user_role_ft');
CREATE FULLTEXT INDEX idx_user_role_ft ON users(role) WITH PARSER ngram;

CALL DropIndexIfExists('sessions', 'idx_session_user_id');
CREATE INDEX idx_session_user_id ON sessions(user_id);

CALL DropIndexIfExists('sessions', 'idx_session_token');
CREATE FULLTEXT INDEX idx_session_token ON sessions(session_token) WITH PARSER ngram;

CALL DropIndexIfExists('dispatchers', 'idx_dispatcher_user_id');
CREATE INDEX idx_dispatcher_user_id ON dispatchers(user_id);

CALL DropIndexIfExists('dispatchers', 'idx_dispatcher_area_id');
CREATE INDEX idx_dispatcher_area_id ON dispatchers(area_id);

CALL DropIndexIfExists('tow_trucks', 'idx_tow_trucks_status');
CREATE INDEX idx_tow_trucks_status ON tow_trucks(status);

CALL DropIndexIfExists('tow_trucks', 'idx_tow_trucks_area_id');
CREATE INDEX idx_tow_trucks_area_id ON tow_trucks(area_id);

CALL DropIndexIfExists('tow_trucks', 'idx_tow_truck_driver_id');
CREATE INDEX idx_tow_truck_driver_id ON tow_trucks(driver_id);

CALL DropIndexIfExists('tow_trucks', 'idx_tow_truck_status_and_area_id');
CREATE INDEX idx_tow_truck_status_and_area_id ON tow_trucks(status, area_id);

CALL DropIndexIfExists('nodes', 'idx_node_name_ft');
CREATE FULLTEXT INDEX idx_node_name_ft ON nodes(name) WITH PARSER ngram;

CALL DropIndexIfExists('nodes', 'idx_node_area_id');
CREATE INDEX idx_node_area_id ON nodes(area_id);

CALL DropIndexIfExists('edges', 'idx_edges_node_a_id');
CREATE INDEX idx_edges_node_a_id ON edges(node_a_id);

CALL DropIndexIfExists('edges', 'idx_edges_node_b_id');
CREATE INDEX idx_edges_node_b_id ON edges(node_b_id);

CALL DropIndexIfExists('locations', 'idx_location_tow_truck_id');
CREATE INDEX idx_location_tow_truck_id ON locations(tow_truck_id);

CALL DropIndexIfExists('locations', 'idx_location_node_id');
CREATE INDEX idx_location_node_id ON locations(node_id);

CALL DropIndexIfExists('locations', 'idx_location_timestamp');
CREATE INDEX idx_location_timestamp ON locations(timestamp);

CALL DropIndexIfExists('orders', 'idx_order_client_id');
CREATE INDEX idx_order_client_id ON orders(client_id);

CALL DropIndexIfExists('orders', 'idx_order_dispatcher_id');
CREATE INDEX idx_order_dispatcher_id ON orders(dispatcher_id);

CALL DropIndexIfExists('orders', 'idx_order_tow_truck_id');
CREATE INDEX idx_order_tow_truck_id ON orders(tow_truck_id);

CALL DropIndexIfExists('orders', 'idx_order_status');
CREATE INDEX idx_order_status ON orders(status);

CALL DropIndexIfExists('orders', 'idx_order_node_id');
CREATE INDEX idx_order_node_id ON orders(node_id);

CALL DropIndexIfExists('orders', 'idx_order_order_time');
CREATE INDEX idx_order_order_time ON orders(order_time);

CALL DropIndexIfExists('orders', 'idx_order_completed_time');
CREATE INDEX idx_order_completed_time ON orders(completed_time);

CALL DropIndexIfExists('completed_orders', 'idx_completed_order_order_id');
CREATE INDEX idx_completed_order_order_id ON completed_orders(order_id);

CALL DropIndexIfExists('completed_orders', 'idx_completed_order_tow_truck_id');
CREATE INDEX idx_completed_order_tow_truck_id ON completed_orders(tow_truck_id);

CALL DropIndexIfExists('completed_orders', 'idx_completed_order_completed_time');
CREATE INDEX idx_completed_order_completed_time ON completed_orders(completed_time);