UPDATE failed_logins
SET created_at = CURRENT_TIMESTAMP
WHERE created_at IS NULL;

ALTER TABLE failed_logins
ALTER COLUMN created_at SET NOT NULL;
