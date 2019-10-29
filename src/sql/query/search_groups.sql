SELECT
  `group`.`id`,
  `group`.`name`,
  `group`.`channel_group`,
  `group`.`vc`
FROM `group`
WHERE
  `group`.`name` LIKE ?