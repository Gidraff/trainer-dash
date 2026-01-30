-- migrations/0002_add_sessions_and_programs.sql

-- 1. Upgrade Workouts Table 
-- We add 'name' for the 8-week block and change 'plan' to JSONB 
-- for flexible exercise structures (sets, reps, rest).
ALTER TABLE workouts 
ADD COLUMN name TEXT NOT NULL DEFAULT 'Unnamed Program',
ADD COLUMN duration_weeks INT DEFAULT 8,
ADD COLUMN is_active BOOLEAN DEFAULT true;

ALTER TABLE workouts ALTER COLUMN plan TYPE JSONB USING plan::jsonb;

-- 2. Upgrade Sessions Table 
-- Adding numeric energy/rating (1-5) to support emoji UI mapping
-- and creating the link to the specific workout program.
ALTER TABLE sessions 
ADD COLUMN workout_id UUID REFERENCES workouts(id) ON DELETE SET NULL,
ADD COLUMN energy_level INT CHECK (energy_level BETWEEN 1 AND 5),
ADD COLUMN athlete_rating INT CHECK (athlete_rating BETWEEN 1 AND 5),
ADD COLUMN athlete_notes TEXT,
ADD COLUMN performance_rating INT CHECK (performance_rating BETWEEN 1 AND 5),
ADD COLUMN trainer_feedback TEXT;

-- Convert weight to NUMERIC for decimal precision (e.g., 92.5kg)
ALTER TABLE sessions ALTER COLUMN weight TYPE NUMERIC(5,2);