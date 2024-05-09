-- This is a hack to make diesel work with cockroachdb
-- Because empty migrations are not allowed, we need to add a dummy statement
SELECT 1;
