-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
-- sessions テーブルにインデックスを追加
CREATE INDEX idx_session_token ON sessions(session_token);

-- tow_trucks テーブルにインデックスを追加
CREATE INDEX idx_tow_truck_driver_id ON tow_trucks(driver_id);

-- locations テーブルにインデックスを追加
CREATE INDEX idx_location_tow_truck_id ON locations(tow_truck_id);

-- dispatchers テーブルにインデックスを追加
CREATE INDEX idx_dispatcher_user_id ON dispatchers(user_id);

-- orders テーブルにインデックスを追加
CREATE INDEX idx_order_client_id ON orders(client_id);
CREATE INDEX idx_order_dispatcher_id ON orders(dispatcher_id);
CREATE INDEX idx_order_tow_truck_id ON orders(tow_truck_id);