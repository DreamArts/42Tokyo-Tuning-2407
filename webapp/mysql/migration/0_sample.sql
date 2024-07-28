-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
-- Create stored procedure for dropping indexes if they exist
DELIMITER $$
DROP PROCEDURE IF EXISTS DropIndexIfExists$$
CREATE PROCEDURE DropIndexIfExists(IN tableName VARCHAR(64), IN indexName VARCHAR(64))
BEGIN
    IF (SELECT COUNT(*)
            FROM information_schema.statistics
            WHERE table_schema = 'app'
                AND table_name = tableName
                AND index_name = indexName) > 0 THEN
        SET @s = CONCAT('DROP INDEX ', indexName, ' ON ', tableName);
        PREPARE stmt FROM @s;
        EXECUTE stmt;
        DEALLOCATE PREPARE stmt;
    END IF;
END$$
DELIMITER ;