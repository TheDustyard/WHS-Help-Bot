-- name: create_group_table
-- Creates the table to store the classes inside of
CREATE TABLE IF NOT EXISTS `group` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `name` TEXT NOT NULL COLLATE NOCASE,
  `channel_group` TEXT NOT NULL UNIQUE,
  `vc` TEXT NOT NULL UNIQUE
);

-- name: get_all_groups
-- Reads all of the groups from the table
SELECT
  `id`,
  `name`,
  `channel_group`,
  `vc`
FROM
  `group`;

-- name: count_all_groups
-- Get the count of how many groups there
-- are in the table
SELECT
  COUNT(*)
FROM
  `group`;

-- name: search_groups
-- Search through the table of groups and
-- filter by the name
SELECT
  `id`,
  `name`,
  `channel_group`,
  `vc`
FROM
  `group`
WHERE
  `name` LIKE ?;

-- name: insert_group
-- Insert a group into the table
INSERT INTO
  `group`(`name`, `channel_group`, `vc`)
VALUES
  (?, ?, ?);