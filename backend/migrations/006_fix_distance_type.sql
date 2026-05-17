-- Fix distance_km type mismatch: DECIMAL -> DOUBLE PRECISION to match Rust f64
ALTER TABLE events ALTER COLUMN distance_km TYPE DOUBLE PRECISION;
