CREATE TABLE tasks (
    id CHAR(26) CHARACTER SET ascii COLLATE ascii_bin NOT NULL,
    title VARCHAR(200) NOT NULL,

    PRIMARY KEY (id),
    CONSTRAINT tasks_title_length
        CHECK (CHAR_LENGTH(title) BETWEEN 1 AND 200),
    CONSTRAINT tasks_title_trimmed
        CHECK (title = TRIM(title))
) ENGINE = InnoDB
  DEFAULT CHARACTER SET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;
