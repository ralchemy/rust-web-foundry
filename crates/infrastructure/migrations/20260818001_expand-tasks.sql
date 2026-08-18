ALTER TABLE tasks
    ADD COLUMN description VARCHAR(2000) NULL AFTER title,
    ADD COLUMN priority VARCHAR(16) NOT NULL DEFAULT 'normal' AFTER description,
    ADD COLUMN status VARCHAR(16) NOT NULL DEFAULT 'pending' AFTER priority,
    ADD COLUMN assignee_id CHAR(26) CHARACTER SET ascii COLLATE ascii_bin NULL AFTER status,
    ADD COLUMN estimate_minutes INT UNSIGNED NULL AFTER assignee_id,
    ADD COLUMN revision BIGINT UNSIGNED NOT NULL DEFAULT 1 AFTER estimate_minutes,
    ADD CONSTRAINT tasks_description_length
        CHECK (description IS NULL OR CHAR_LENGTH(description) BETWEEN 1 AND 2000),
    ADD CONSTRAINT tasks_description_trimmed
        CHECK (description IS NULL OR description = TRIM(description)),
    ADD CONSTRAINT tasks_priority
        CHECK (priority IN ('low', 'normal', 'high')),
    ADD CONSTRAINT tasks_status
        CHECK (status IN ('pending', 'in_progress', 'completed', 'cancelled')),
    ADD CONSTRAINT tasks_estimate_minutes
        CHECK (estimate_minutes IS NULL OR estimate_minutes >= 1),
    ADD CONSTRAINT tasks_revision
        CHECK (revision >= 1);
