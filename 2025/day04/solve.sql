-- First import raw data into a table so we can work
CREATE TABLE raw_data(line_data VARCHAR NOT NULL);
.import --csv './sample.txt' raw_data
-- Then use the auto-incrementing ID to add a y coordinate
CREATE TABLE grid_lines(
    y INTEGER PRIMARY KEY,
    line_data VARCHAR NOT NULL
);
INSERT INTO grid_lines(line_data)
SELECT line_data
FROM raw_data;
-- Now create a table to hold the paper rolls
CREATE TABLE rolls(
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    PRIMARY KEY (x, y)
);
WITH RECURSIVE cte AS (
    SELECT y,
        1 x,
        line_data,
        substr(line_data, 1, 1) c
    FROM grid_lines
    UNION ALL
    SELECT y,
        x + 1,
        line_data,
        substr(line_data, x + 1, 1)
    FROM cte
    WHERE x <= length(line_data)
)
INSERT INTO rolls
SELECT x,
    y
FROM cte
WHERE c = '@';
-- Now compute part 1
SELECT COUNT(*) as part1
FROM rolls r
WHERE (
        SELECT COUNT(*) - 1
        FROM rolls o
        WHERE o.x >= r.x - 1
            AND o.x <= r.x + 1
            AND o.y >= r.y - 1
            AND o.y <= r.y + 1
    ) < 4;
