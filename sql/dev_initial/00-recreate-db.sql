-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
 usename = 'ganin_user' OR datname = 'ganin_ws_db';
DROP DATABASE IF EXISTS ganin_ws_db;
DROP USER IF EXISTS ganin_user;

-- DEV ONLY - Dev only password (for local dev and unit test).
CREATE USER ganin_user PASSWORD 'dev_only_pwd';
CREATE DATABASE ganin_ws_db owner ganin_user ENCODING = 'UTF-8';