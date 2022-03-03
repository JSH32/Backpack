
CREATE TYPE theme_color AS ENUM (
    'gray',
    'red',
    'orange',
    'yellow',
    'green',
    'teal',
    'blue',
    'cyan',
    'purple',
    'pink'
);

CREATE TABLE settings
(
    -- Only true is allowed and this is set to unique. Prevents multiple settings rows
    one_row_enforce BOOLEAN     PRIMARY KEY DEFAULT TRUE                             NOT NULL UNIQUE,
	app_name        VARCHAR(64)             DEFAULT 'Backpack'                       NOT NULL,
	app_description TEXT                    DEFAULT 'A file host for all your needs' NOT NULL,
	color           theme_color             DEFAULT 'purple'::theme_color            NOT NULL

	CONSTRAINT one_row_unique CHECK (one_row_enforce)
);

-- Insert default settings at first, configuration done through web UI
INSERT INTO settings (one_row_enforce) VALUES (true);