CREATE TABLE characters (
  id BIGINT NOT NULL AUTO_INCREMENT,
  name CHAR(30) NOT NULL,
  max_hp INT NOT NULL DEFAULT 12000,
  max_appetite INT NOT NULL DEFAULT 8000,
  PRIMARY KEY (id)
)
