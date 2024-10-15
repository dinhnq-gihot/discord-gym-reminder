-- Your SQL goes here
CREATE TABLE schedules_musculatures (
    schedule_id INTEGER REFERENCES schedules(id) ON DELETE CASCADE,
    musculature_id INTEGER REFERENCES musculatures(id) ON DELETE CASCADE,
    PRIMARY KEY (schedule_id, musculature_id)
);