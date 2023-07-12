CREATE TABLE `user_progress` (
  `uuid` VARCHAR(36) NOT NULL,
  `progress` JSON,
  `score` INT,
  `user_name` VARCHAR(36),
  PRIMARY KEY (`uuid`)
);
