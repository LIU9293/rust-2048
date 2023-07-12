CREATE TABLE `user_progress` (
  `uuid` VARCHAR(36) NOT NULL,
  `progress` JSON,
  PRIMARY KEY (`uuid`)
);

CREATE TABLE `user_score` (
  `name` VARCHAR(36) NOT NULL,
  `score` INT NOT NULL
);