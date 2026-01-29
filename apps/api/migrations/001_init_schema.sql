CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE clients (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  trainer_id TEXT NOT NULL, -- 👈 Added: Maps to claims.sub
  name TEXT NOT NULL,
  goal TEXT,
  profile TEXT,
  created_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE workouts (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  client_id UUID REFERENCES clients(id) ON DELETE CASCADE,
  plan TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE sessions (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  client_id UUID REFERENCES clients(id) ON DELETE CASCADE,
  date DATE NOT NULL,
  weight INT,
  mood TEXT,
  commentary TEXT,
  feedback TEXT,
  created_at TIMESTAMPTZ DEFAULT now()
);