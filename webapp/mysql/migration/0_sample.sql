-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。

-- Add index to users table
CREATE INDEX idx_username ON users(username);

-- Move tow_truck's current location to tow_trucks table
ALTER TABLE tow_trucks ADD COLUMN node_id INT;

UPDATE tow_trucks tt
JOIN (
    SELECT tow_truck_id, node_id
    FROM locations
    WHERE (tow_truck_id, timestamp) IN (
        SELECT tow_truck_id, MAX(timestamp)
        FROM locations
        GROUP BY tow_truck_id
    )
) l ON tt.id = l.tow_truck_id
SET tt.node_id = l.node_id;
