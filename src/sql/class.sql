-- name: create_class_table
-- Creates the table to store the classes inside of
CREATE TABLE IF NOT EXISTS `class` (
  `id` INTEGER NOT NULL PRIMARY KEY,
  `name` TEXT NOT NULL COLLATE NOCASE,
  `role` TEXT NOT NULL UNIQUE,
  `group` INTEGER,
  `channel` TEXT NOT NULL UNIQUE,
  FOREIGN KEY(`group`) REFERENCES `group`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
);

-- name: get_all_classes
-- Reads all of the classes from the table with their respective group
SELECT
  `class`.`id`,
  `class`.`name`,
  `class`.`role`,
  `class`.`channel`,
  `group`.`id`,
  `group`.`name`,
  `group`.`channel_group`,
  `group`.`vc`
FROM `class`
LEFT OUTER JOIN `group` ON `class`.`group` = `group`.`id`;

-- name: count_all_classes
-- Get the count of how many classes there
-- are in the table
SELECT
  COUNT(*)
FROM `class`;

-- name: search_classes
-- Search through the table of classes and
-- filter by the name and the group name
SELECT
  `class`.`id`,
  `class`.`name`,
  `class`.`role`,
  `class`.`channel`,
  `group`.`id`,
  `group`.`name`,
  `group`.`channel_group`,
  `group`.`vc`
FROM `class`
LEFT OUTER JOIN `group` ON `class`.`group` = `group`.`id`
WHERE
  `class`.`name` LIKE ?1
  OR `group`.`name` LIKE ?1;

-- name: filter_classes_by_roles
-- Search throgh the table of classes and
-- filter by a given array of roles
SELECT
  `class`.`id`,
  `class`.`name`,
  `class`.`role`,
  `class`.`channel`,
  `group`.`id`,
  `group`.`name`,
  `group`.`channel_group`,
  `group`.`vc`
FROM `class`
LEFT OUTER JOIN `group` ON `class`.`group` = `group`.`id`
WHERE
  `class`.`role` in rarray(?);